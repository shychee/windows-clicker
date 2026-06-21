#![cfg_attr(windows, windows_subsystem = "windows")]

#[cfg(windows)]
mod win32;

#[cfg(windows)]
fn main() {
    if let Err(err) = win32::run() {
        win32::show_error("Windows Clicker", &err);
    }
}

#[cfg(not(windows))]
fn main() {
    eprintln!("windows-clicker is a Windows GUI application. Run it on Windows.");
}
