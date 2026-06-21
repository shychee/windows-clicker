#![cfg(windows)]

use std::ffi::{c_void, OsStr};
use std::iter::once;
use std::mem::{size_of, zeroed};
use std::os::windows::ffi::OsStrExt;
use std::ptr::{null, null_mut};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use windows_clicker::config::{ClickerConfig, MouseButton, VirtualKey, MIN_INTERVAL_MS};
use windows_clicker::runtime::ClickerRuntime;

type Bool = i32;
type Dword = u32;
type Hbrush = isize;
type Hcursor = isize;
type HgdiObj = isize;
type Hicon = isize;
type Hinstance = isize;
type Hmenu = isize;
type Hmodule = isize;
type Hwnd = isize;
type Lparam = isize;
type Lpcwstr = *const u16;
type Lresult = isize;
type Uint = u32;
type UlongPtr = usize;
type Wparam = usize;

#[repr(C)]
struct Point {
    x: i32,
    y: i32,
}

#[repr(C)]
struct Msg {
    hwnd: Hwnd,
    message: Uint,
    wparam: Wparam,
    lparam: Lparam,
    time: Dword,
    pt: Point,
}

type WndProc = Option<unsafe extern "system" fn(Hwnd, Uint, Wparam, Lparam) -> Lresult>;

#[repr(C)]
struct WndClassW {
    style: Uint,
    lpfn_wnd_proc: WndProc,
    cb_cls_extra: i32,
    cb_wnd_extra: i32,
    h_instance: Hinstance,
    h_icon: Hicon,
    h_cursor: Hcursor,
    hbr_background: Hbrush,
    lpsz_menu_name: Lpcwstr,
    lpsz_class_name: Lpcwstr,
}

#[repr(C)]
struct CreateStructW {
    lp_create_params: *mut c_void,
    h_instance: Hinstance,
    h_menu: Hmenu,
    hwnd_parent: Hwnd,
    cy: i32,
    cx: i32,
    y: i32,
    x: i32,
    style: i32,
    lpsz_name: Lpcwstr,
    lpsz_class: Lpcwstr,
    dw_ex_style: Dword,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct MouseInput {
    dx: i32,
    dy: i32,
    mouse_data: Dword,
    dw_flags: Dword,
    time: Dword,
    dw_extra_info: UlongPtr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct KeybdInput {
    w_vk: u16,
    w_scan: u16,
    dw_flags: Dword,
    time: Dword,
    dw_extra_info: UlongPtr,
}

#[repr(C)]
union InputUnion {
    mi: MouseInput,
    ki: KeybdInput,
}

#[repr(C)]
struct Input {
    input_type: Dword,
    input: InputUnion,
}

#[link(name = "user32")]
extern "system" {
    fn AppendMenuW(h_menu: Hmenu, u_flags: Uint, u_id_new_item: usize, lp_new_item: Lpcwstr) -> Bool;
    fn CreateMenu() -> Hmenu;
    fn CreateWindowExW(
        dw_ex_style: Dword,
        lp_class_name: Lpcwstr,
        lp_window_name: Lpcwstr,
        dw_style: Dword,
        x: i32,
        y: i32,
        n_width: i32,
        n_height: i32,
        hwnd_parent: Hwnd,
        h_menu: Hmenu,
        h_instance: Hinstance,
        lp_param: *mut c_void,
    ) -> Hwnd;
    fn DefWindowProcW(hwnd: Hwnd, msg: Uint, wparam: Wparam, lparam: Lparam) -> Lresult;
    fn DestroyWindow(hwnd: Hwnd) -> Bool;
    fn DispatchMessageW(lp_msg: *const Msg) -> Lresult;
    fn GetDlgItem(hwnd: Hwnd, n_id_dlg_item: i32) -> Hwnd;
    fn GetMessageW(lp_msg: *mut Msg, hwnd: Hwnd, msg_filter_min: Uint, msg_filter_max: Uint) -> Bool;
    fn GetWindowLongPtrW(hwnd: Hwnd, n_index: i32) -> isize;
    fn GetWindowTextLengthW(hwnd: Hwnd) -> i32;
    fn GetWindowTextW(hwnd: Hwnd, lp_string: *mut u16, n_max_count: i32) -> i32;
    fn LoadCursorW(h_instance: Hinstance, lp_cursor_name: Lpcwstr) -> Hcursor;
    fn MessageBoxW(hwnd: Hwnd, lp_text: Lpcwstr, lp_caption: Lpcwstr, u_type: Uint) -> i32;
    fn PostQuitMessage(n_exit_code: i32);
    fn RegisterClassW(lp_wnd_class: *const WndClassW) -> u16;
    fn RegisterHotKey(hwnd: Hwnd, id: i32, fs_modifiers: Uint, vk: Uint) -> Bool;
    fn SendInput(c_inputs: Uint, p_inputs: *mut Input, cb_size: i32) -> Uint;
    fn SendMessageW(hwnd: Hwnd, msg: Uint, wparam: Wparam, lparam: Lparam) -> Lresult;
    fn SetMenu(hwnd: Hwnd, h_menu: Hmenu) -> Bool;
    fn SetWindowLongPtrW(hwnd: Hwnd, n_index: i32, dw_new_long: isize) -> isize;
    fn SetWindowTextW(hwnd: Hwnd, lp_string: Lpcwstr) -> Bool;
    fn ShowWindow(hwnd: Hwnd, n_cmd_show: i32) -> Bool;
    fn TranslateMessage(lp_msg: *const Msg) -> Bool;
    fn UnregisterHotKey(hwnd: Hwnd, id: i32) -> Bool;
}

#[link(name = "kernel32")]
extern "system" {
    fn GetLastError() -> Dword;
    fn GetModuleHandleW(lp_module_name: Lpcwstr) -> Hmodule;
}

#[link(name = "gdi32")]
extern "system" {
    fn GetStockObject(i: i32) -> HgdiObj;
}

const APP_CLASS: &str = "WindowsClickerWindow";
const APP_TITLE: &str = "Windows Clicker";

const ID_MOUSE_BUTTON: i32 = 1001;
const ID_MOUSE_INTERVAL: i32 = 1002;
const ID_KEY_INPUT: i32 = 1003;
const ID_KEY_INTERVAL: i32 = 1004;
const ID_MOUSE_TOGGLE: i32 = 1005;
const ID_KEY_TOGGLE: i32 = 1006;
const ID_STOP_ALL: i32 = 1007;
const ID_STATUS: i32 = 1008;
const IDC_STATIC_ID: i32 = -1;

const HOTKEY_MOUSE: i32 = 2001;
const HOTKEY_KEYBOARD: i32 = 2002;
const HOTKEY_STOP: i32 = 2003;

const MENU_EXIT: usize = 3001;

const CS_VREDRAW: Uint = 0x0001;
const CS_HREDRAW: Uint = 0x0002;
const CW_USEDEFAULT: i32 = 0x80000000u32 as i32;
const DEFAULT_GUI_FONT: i32 = 17;
const GWLP_USERDATA: i32 = -21;
const IDC_ARROW: usize = 32512;
const INPUT_KEYBOARD: Dword = 1;
const INPUT_MOUSE: Dword = 0;
const KEYEVENTF_KEYUP: Dword = 0x0002;
const MB_ICONERROR: Uint = 0x00000010;
const MB_OK: Uint = 0x00000000;
const MOUSEEVENTF_LEFTDOWN: Dword = 0x0002;
const MOUSEEVENTF_LEFTUP: Dword = 0x0004;
const MOUSEEVENTF_RIGHTDOWN: Dword = 0x0008;
const MOUSEEVENTF_RIGHTUP: Dword = 0x0010;
const MOUSEEVENTF_MIDDLEDOWN: Dword = 0x0020;
const MOUSEEVENTF_MIDDLEUP: Dword = 0x0040;
const SW_SHOW: i32 = 5;
const VK_F6: Uint = 0x75;
const VK_F7: Uint = 0x76;
const VK_F8: Uint = 0x77;
const WM_COMMAND: Uint = 0x0111;
const WM_CREATE: Uint = 0x0001;
const WM_DESTROY: Uint = 0x0002;
const WM_HOTKEY: Uint = 0x0312;
const WM_NCCREATE: Uint = 0x0081;
const WM_SETFONT: Uint = 0x0030;

const CB_ADDSTRING: Uint = 0x0143;
const CB_GETCURSEL: Uint = 0x0147;
const CB_SETCURSEL: Uint = 0x014E;

const CBS_DROPDOWNLIST: Dword = 0x0003;
const ES_AUTOHSCROLL: Dword = 0x0080;
const WS_BORDER: Dword = 0x00800000;
const WS_CAPTION: Dword = 0x00C00000;
const WS_CHILD: Dword = 0x40000000;
const WS_CLIPCHILDREN: Dword = 0x02000000;
const WS_EX_CLIENTEDGE: Dword = 0x00000200;
const WS_OVERLAPPED: Dword = 0x00000000;
const WS_SYSMENU: Dword = 0x00080000;
const WS_TABSTOP: Dword = 0x00010000;
const WS_VISIBLE: Dword = 0x10000000;

pub fn run() -> Result<(), String> {
    unsafe {
        let instance = GetModuleHandleW(null());
        if instance == 0 {
            return Err(format!("GetModuleHandleW failed: {}", GetLastError()));
        }

        let class_name = wide(APP_CLASS);
        let window_class = WndClassW {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfn_wnd_proc: Some(window_proc),
            cb_cls_extra: 0,
            cb_wnd_extra: 0,
            h_instance: instance,
            h_icon: 0,
            h_cursor: LoadCursorW(0, IDC_ARROW as Lpcwstr),
            hbr_background: GetStockObject(0) as Hbrush,
            lpsz_menu_name: null(),
            lpsz_class_name: class_name.as_ptr(),
        };

        if RegisterClassW(&window_class) == 0 {
            return Err(format!("RegisterClassW failed: {}", GetLastError()));
        }

        let mut app = Box::new(AppState::new()?);
        let app_ptr = app.as_mut() as *mut AppState;

        let hwnd = CreateWindowExW(
            0,
            class_name.as_ptr(),
            wide(APP_TITLE).as_ptr(),
            WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_CLIPCHILDREN,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            430,
            330,
            0,
            0,
            instance,
            app_ptr.cast(),
        );

        if hwnd == 0 {
            return Err(format!("CreateWindowExW failed: {}", GetLastError()));
        }

        app.hwnd = hwnd;
        Box::leak(app);

        ShowWindow(hwnd, SW_SHOW);

        let mut msg: Msg = zeroed();
        while GetMessageW(&mut msg, 0, 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }

    Ok(())
}

pub fn show_error(title: &str, message: &str) {
    unsafe {
        MessageBoxW(
            0,
            wide(message).as_ptr(),
            wide(title).as_ptr(),
            MB_OK | MB_ICONERROR,
        );
    }
}

struct AppState {
    hwnd: Hwnd,
    runtime: Arc<Mutex<ClickerRuntime>>,
    mouse_stop: Option<Arc<AtomicBool>>,
    keyboard_stop: Option<Arc<AtomicBool>>,
    mouse_thread: Option<JoinHandle<()>>,
    keyboard_thread: Option<JoinHandle<()>>,
}

impl AppState {
    fn new() -> Result<Self, String> {
        let config = ClickerConfig::new(MouseButton::Left, 100, "Space", 100)?;

        Ok(Self {
            hwnd: 0,
            runtime: Arc::new(Mutex::new(ClickerRuntime::new(config))),
            mouse_stop: None,
            keyboard_stop: None,
            mouse_thread: None,
            keyboard_thread: None,
        })
    }

    unsafe fn init_window(&mut self, hwnd: Hwnd) {
        self.hwnd = hwnd;
        create_menu(hwnd);
        create_controls(hwnd);
        self.register_hotkeys();
        self.refresh_status();
    }

    unsafe fn register_hotkeys(&self) {
        RegisterHotKey(self.hwnd, HOTKEY_MOUSE, 0, VK_F6);
        RegisterHotKey(self.hwnd, HOTKEY_KEYBOARD, 0, VK_F7);
        RegisterHotKey(self.hwnd, HOTKEY_STOP, 0, VK_F8);
    }

    unsafe fn unregister_hotkeys(&self) {
        UnregisterHotKey(self.hwnd, HOTKEY_MOUSE);
        UnregisterHotKey(self.hwnd, HOTKEY_KEYBOARD);
        UnregisterHotKey(self.hwnd, HOTKEY_STOP);
    }

    unsafe fn handle_command(&mut self, command_id: i32) {
        match command_id {
            ID_MOUSE_TOGGLE => self.toggle_mouse_from_ui(),
            ID_KEY_TOGGLE => self.toggle_keyboard_from_ui(),
            ID_STOP_ALL => self.stop_all(),
            id if id as usize == MENU_EXIT => {
                DestroyWindow(self.hwnd);
            }
            _ => {}
        }
    }

    unsafe fn toggle_mouse_from_ui(&mut self) {
        match self.read_config() {
            Ok(config) => {
                self.runtime.lock().unwrap().set_config(config);
                let running = self.runtime.lock().unwrap().toggle_mouse();
                if running {
                    self.start_mouse_worker();
                } else {
                    self.stop_mouse_worker();
                }
                self.refresh_status();
            }
            Err(err) => show_error(APP_TITLE, &err),
        }
    }

    unsafe fn toggle_keyboard_from_ui(&mut self) {
        match self.read_config() {
            Ok(config) => {
                self.runtime.lock().unwrap().set_config(config);
                let running = self.runtime.lock().unwrap().toggle_keyboard();
                if running {
                    self.start_keyboard_worker();
                } else {
                    self.stop_keyboard_worker();
                }
                self.refresh_status();
            }
            Err(err) => show_error(APP_TITLE, &err),
        }
    }

    fn start_mouse_worker(&mut self) {
        self.stop_mouse_worker();
        let stop = Arc::new(AtomicBool::new(false));
        let stop_for_thread = Arc::clone(&stop);
        let runtime = Arc::clone(&self.runtime);

        self.mouse_thread = Some(thread::spawn(move || {
            while !stop_for_thread.load(Ordering::Relaxed) {
                let snapshot = runtime.lock().unwrap().snapshot();
                click_mouse(snapshot.config.mouse_button);
                thread::sleep(Duration::from_millis(snapshot.config.mouse_interval_ms));
            }
        }));
        self.mouse_stop = Some(stop);
    }

    fn start_keyboard_worker(&mut self) {
        self.stop_keyboard_worker();
        let stop = Arc::new(AtomicBool::new(false));
        let stop_for_thread = Arc::clone(&stop);
        let runtime = Arc::clone(&self.runtime);

        self.keyboard_thread = Some(thread::spawn(move || {
            while !stop_for_thread.load(Ordering::Relaxed) {
                let snapshot = runtime.lock().unwrap().snapshot();
                press_key(snapshot.config.keyboard_key);
                thread::sleep(Duration::from_millis(snapshot.config.keyboard_interval_ms));
            }
        }));
        self.keyboard_stop = Some(stop);
    }

    fn stop_mouse_worker(&mut self) {
        if let Some(stop) = self.mouse_stop.take() {
            stop.store(true, Ordering::Relaxed);
        }
        if let Some(handle) = self.mouse_thread.take() {
            let _ = handle.join();
        }
    }

    fn stop_keyboard_worker(&mut self) {
        if let Some(stop) = self.keyboard_stop.take() {
            stop.store(true, Ordering::Relaxed);
        }
        if let Some(handle) = self.keyboard_thread.take() {
            let _ = handle.join();
        }
    }

    unsafe fn stop_all(&mut self) {
        self.runtime.lock().unwrap().stop_all();
        self.stop_mouse_worker();
        self.stop_keyboard_worker();
        self.refresh_status();
    }

    unsafe fn refresh_status(&self) {
        let snapshot = self.runtime.lock().unwrap().snapshot();
        let mouse = if snapshot.mouse_running { "ON" } else { "OFF" };
        let keyboard = if snapshot.keyboard_running { "ON" } else { "OFF" };
        let text = format!(
            "Mouse: {mouse} (F6)    Keyboard: {keyboard} (F7)    Stop all: F8"
        );
        SetWindowTextW(GetDlgItem(self.hwnd, ID_STATUS), wide(&text).as_ptr());
        SetWindowTextW(
            GetDlgItem(self.hwnd, ID_MOUSE_TOGGLE),
            wide(if snapshot.mouse_running {
                "Stop Mouse (F6)"
            } else {
                "Start Mouse (F6)"
            })
            .as_ptr(),
        );
        SetWindowTextW(
            GetDlgItem(self.hwnd, ID_KEY_TOGGLE),
            wide(if snapshot.keyboard_running {
                "Stop Keyboard (F7)"
            } else {
                "Start Keyboard (F7)"
            })
            .as_ptr(),
        );
    }

    unsafe fn read_config(&self) -> Result<ClickerConfig, String> {
        let mouse_button = match SendMessageW(
            GetDlgItem(self.hwnd, ID_MOUSE_BUTTON),
            CB_GETCURSEL,
            0,
            0,
        ) {
            1 => MouseButton::Right,
            2 => MouseButton::Middle,
            _ => MouseButton::Left,
        };
        let mouse_interval_ms = read_u64(self.hwnd, ID_MOUSE_INTERVAL, "mouse interval")?;
        let key = read_text(self.hwnd, ID_KEY_INPUT);
        let keyboard_interval_ms = read_u64(self.hwnd, ID_KEY_INTERVAL, "keyboard interval")?;

        ClickerConfig::new(mouse_button, mouse_interval_ms, &key, keyboard_interval_ms)
    }
}

impl Drop for AppState {
    fn drop(&mut self) {
        self.stop_mouse_worker();
        self.stop_keyboard_worker();
    }
}

unsafe extern "system" fn window_proc(
    hwnd: Hwnd,
    msg: Uint,
    wparam: Wparam,
    lparam: Lparam,
) -> Lresult {
    if msg == WM_NCCREATE {
        let create = lparam as *const CreateStructW;
        let app = (*create).lp_create_params as *mut AppState;
        SetWindowLongPtrW(hwnd, GWLP_USERDATA, app as isize);
        return 1;
    }

    let app = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut AppState;

    match msg {
        WM_CREATE => {
            if !app.is_null() {
                (*app).init_window(hwnd);
            }
            0
        }
        WM_COMMAND => {
            if !app.is_null() {
                (*app).handle_command((wparam & 0xffff) as i32);
            }
            0
        }
        WM_HOTKEY => {
            if !app.is_null() {
                match wparam as i32 {
                    HOTKEY_MOUSE => (*app).toggle_mouse_from_ui(),
                    HOTKEY_KEYBOARD => (*app).toggle_keyboard_from_ui(),
                    HOTKEY_STOP => (*app).stop_all(),
                    _ => {}
                }
            }
            0
        }
        WM_DESTROY => {
            if !app.is_null() {
                (*app).unregister_hotkeys();
                (*app).stop_all();
                drop(Box::from_raw(app));
                SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0);
            }
            PostQuitMessage(0);
            0
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

unsafe fn create_menu(hwnd: Hwnd) {
    let menu = CreateMenu();
    AppendMenuW(menu, 0, MENU_EXIT, wide("Exit").as_ptr());
    SetMenu(hwnd, menu);
}

unsafe fn create_controls(hwnd: Hwnd) {
    let font = GetStockObject(DEFAULT_GUI_FONT);

    static_label(hwnd, "Mouse button", 20, 20, 110, 24);
    let combo = control(
        hwnd,
        "COMBOBOX",
        "",
        WS_VISIBLE | WS_CHILD | WS_TABSTOP | WS_BORDER | CBS_DROPDOWNLIST,
        ID_MOUSE_BUTTON,
        140,
        18,
        180,
        120,
    );
    for item in ["Left", "Right", "Middle"] {
        SendMessageW(combo, CB_ADDSTRING, 0, wide(item).as_ptr() as Lparam);
    }
    SendMessageW(combo, CB_SETCURSEL, 0, 0);

    static_label(hwnd, "Mouse interval ms", 20, 58, 130, 24);
    textbox(hwnd, ID_MOUSE_INTERVAL, "100", 160, 56, 90, 24);
    static_label(hwnd, &format!("min {MIN_INTERVAL_MS}"), 260, 58, 70, 24);

    static_label(hwnd, "Keyboard key", 20, 96, 110, 24);
    textbox(hwnd, ID_KEY_INPUT, "Space", 160, 94, 90, 24);

    static_label(hwnd, "Keyboard interval ms", 20, 134, 140, 24);
    textbox(hwnd, ID_KEY_INTERVAL, "100", 160, 132, 90, 24);
    static_label(hwnd, &format!("min {MIN_INTERVAL_MS}"), 260, 134, 70, 24);

    button(hwnd, ID_MOUSE_TOGGLE, "Start Mouse (F6)", 20, 180, 170, 34);
    button(hwnd, ID_KEY_TOGGLE, "Start Keyboard (F7)", 205, 180, 180, 34);
    button(hwnd, ID_STOP_ALL, "Stop All (F8)", 20, 222, 365, 34);

    label_with_id(hwnd, ID_STATUS, "", 20, 270, 365, 24);

    for id in [
        ID_MOUSE_BUTTON,
        ID_MOUSE_INTERVAL,
        ID_KEY_INPUT,
        ID_KEY_INTERVAL,
        ID_MOUSE_TOGGLE,
        ID_KEY_TOGGLE,
        ID_STOP_ALL,
        ID_STATUS,
    ] {
        SendMessageW(GetDlgItem(hwnd, id), WM_SETFONT, font as Wparam, 1);
    }
}

unsafe fn static_label(hwnd: Hwnd, text: &str, x: i32, y: i32, width: i32, height: i32) -> Hwnd {
    label_with_id(hwnd, IDC_STATIC_ID, text, x, y, width, height)
}

unsafe fn label_with_id(
    hwnd: Hwnd,
    id: i32,
    text: &str,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
) -> Hwnd {
    control(
        hwnd,
        "STATIC",
        text,
        WS_VISIBLE | WS_CHILD,
        id,
        x,
        y,
        width,
        height,
    )
}

unsafe fn textbox(hwnd: Hwnd, id: i32, text: &str, x: i32, y: i32, width: i32, height: i32) -> Hwnd {
    control(
        hwnd,
        "EDIT",
        text,
        WS_VISIBLE | WS_CHILD | WS_TABSTOP | WS_BORDER | ES_AUTOHSCROLL,
        id,
        x,
        y,
        width,
        height,
    )
}

unsafe fn button(hwnd: Hwnd, id: i32, text: &str, x: i32, y: i32, width: i32, height: i32) -> Hwnd {
    control(
        hwnd,
        "BUTTON",
        text,
        WS_VISIBLE | WS_CHILD | WS_TABSTOP,
        id,
        x,
        y,
        width,
        height,
    )
}

unsafe fn control(
    hwnd: Hwnd,
    class_name: &str,
    text: &str,
    style: Dword,
    id: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
) -> Hwnd {
    CreateWindowExW(
        if class_name == "EDIT" {
            WS_EX_CLIENTEDGE
        } else {
            0
        },
        wide(class_name).as_ptr(),
        wide(text).as_ptr(),
        style,
        x,
        y,
        width,
        height,
        hwnd,
        id as Hmenu,
        0,
        null_mut(),
    )
}

unsafe fn read_text(hwnd: Hwnd, id: i32) -> String {
    let control = GetDlgItem(hwnd, id);
    let len = GetWindowTextLengthW(control);
    let mut buffer = vec![0u16; len as usize + 1];
    GetWindowTextW(control, buffer.as_mut_ptr(), buffer.len() as i32);
    String::from_utf16_lossy(&buffer[..len as usize])
}

unsafe fn read_u64(hwnd: Hwnd, id: i32, label: &str) -> Result<u64, String> {
    let text = read_text(hwnd, id);
    text.trim()
        .parse::<u64>()
        .map_err(|_| format!("{label} must be a whole number"))
}

fn click_mouse(button: MouseButton) {
    let (down, up) = match button {
        MouseButton::Left => (MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP),
        MouseButton::Right => (MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP),
        MouseButton::Middle => (MOUSEEVENTF_MIDDLEDOWN, MOUSEEVENTF_MIDDLEUP),
    };

    unsafe {
        send_mouse_event(down);
        send_mouse_event(up);
    }
}

fn press_key(key: VirtualKey) {
    unsafe {
        send_key_event(key, 0);
        send_key_event(key, KEYEVENTF_KEYUP);
    }
}

unsafe fn send_mouse_event(flags: Dword) {
    let mut input = Input {
        input_type: INPUT_MOUSE,
        input: InputUnion {
            mi: MouseInput {
                dx: 0,
                dy: 0,
                mouse_data: 0,
                dw_flags: flags,
                time: 0,
                dw_extra_info: 0,
            },
        },
    };

    SendInput(1, &mut input, size_of::<Input>() as i32);
}

unsafe fn send_key_event(key: VirtualKey, flags: Dword) {
    let mut input = Input {
        input_type: INPUT_KEYBOARD,
        input: InputUnion {
            ki: KeybdInput {
                w_vk: key.0,
                w_scan: 0,
                dw_flags: flags,
                time: 0,
                dw_extra_info: 0,
            },
        },
    };

    SendInput(1, &mut input, size_of::<Input>() as i32);
}

fn wide(value: &str) -> Vec<u16> {
    OsStr::new(value).encode_wide().chain(once(0)).collect()
}
