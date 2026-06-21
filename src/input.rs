use crate::config::VirtualKey;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyEventKind {
    Down,
    Up,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClickerHotkey {
    Mouse,
    Keyboard,
    StopAll,
}

pub const LOW_LEVEL_KEYBOARD_INJECTED_FLAG: u32 = 0x10;
pub const LOW_LEVEL_MOUSE_INJECTED_FLAG: u32 = 0x01;
pub const KEYBOARD_OTHER_KEY_PAUSE_MS: u64 = 45;

pub fn hold_repeat_key_events() -> [KeyEventKind; 2] {
    [KeyEventKind::Up, KeyEventKind::Down]
}

pub fn tap_key_events() -> [KeyEventKind; 2] {
    [KeyEventKind::Down, KeyEventKind::Up]
}

pub fn keyboard_release_events() -> [KeyEventKind; 1] {
    [KeyEventKind::Up]
}

pub fn should_pause_repeat_for_key(target: VirtualKey, pressed: VirtualKey) -> bool {
    pressed != target && !is_clicker_hotkey(pressed)
}

pub fn is_physical_keyboard_hook_event(flags: u32) -> bool {
    flags & LOW_LEVEL_KEYBOARD_INJECTED_FLAG == 0
}

pub fn is_physical_mouse_hook_event(flags: u32) -> bool {
    flags & LOW_LEVEL_MOUSE_INJECTED_FLAG == 0
}

pub fn clicker_hotkey_from_keyboard_event(
    vk_code: u32,
    is_key_down: bool,
    was_already_down: bool,
) -> Option<ClickerHotkey> {
    if !is_key_down || was_already_down {
        return None;
    }

    match vk_code {
        0x75 => Some(ClickerHotkey::Mouse),
        0x76 => Some(ClickerHotkey::Keyboard),
        0x77 => Some(ClickerHotkey::StopAll),
        _ => None,
    }
}

pub fn keyboard_scancode_from_virtual_key(key: VirtualKey) -> Option<u16> {
    let scancode = match key.0 {
        0x08 => 0x0E,
        0x09 => 0x0F,
        0x0D => 0x1C,
        0x10 => 0x2A,
        0x11 => 0x1D,
        0x12 => 0x38,
        0x14 => 0x3A,
        0x1B => 0x01,
        0x20 => 0x39,
        0x25 => 0x4B,
        0x26 => 0x48,
        0x27 => 0x4D,
        0x28 => 0x50,
        0x30 => 0x0B,
        0x31 => 0x02,
        0x32 => 0x03,
        0x33 => 0x04,
        0x34 => 0x05,
        0x35 => 0x06,
        0x36 => 0x07,
        0x37 => 0x08,
        0x38 => 0x09,
        0x39 => 0x0A,
        0x41 => 0x1E,
        0x42 => 0x30,
        0x43 => 0x2E,
        0x44 => 0x20,
        0x45 => 0x12,
        0x46 => 0x21,
        0x47 => 0x22,
        0x48 => 0x23,
        0x49 => 0x17,
        0x4A => 0x24,
        0x4B => 0x25,
        0x4C => 0x26,
        0x4D => 0x32,
        0x4E => 0x31,
        0x4F => 0x18,
        0x50 => 0x19,
        0x51 => 0x10,
        0x52 => 0x13,
        0x53 => 0x1F,
        0x54 => 0x14,
        0x55 => 0x16,
        0x56 => 0x2F,
        0x57 => 0x11,
        0x58 => 0x2D,
        0x59 => 0x15,
        0x5A => 0x2C,
        0x70 => 0x3B,
        0x71 => 0x3C,
        0x72 => 0x3D,
        0x73 => 0x3E,
        0x74 => 0x3F,
        0x75 => 0x40,
        0x76 => 0x41,
        0x77 => 0x42,
        0x78 => 0x43,
        0x79 => 0x44,
        0x7A => 0x57,
        0x7B => 0x58,
        _ => return None,
    };

    Some(scancode)
}

fn is_clicker_hotkey(key: VirtualKey) -> bool {
    matches!(key.0, 0x75..=0x77)
}
