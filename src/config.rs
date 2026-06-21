pub const MIN_INTERVAL_MS: u64 = 25;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VirtualKey(pub u16);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClickerConfig {
    pub mouse_button: MouseButton,
    pub mouse_interval_ms: u64,
    pub keyboard_key: VirtualKey,
    pub keyboard_interval_ms: u64,
}

impl ClickerConfig {
    pub fn new(
        mouse_button: MouseButton,
        mouse_interval_ms: u64,
        keyboard_key_name: &str,
        keyboard_interval_ms: u64,
    ) -> Result<Self, String> {
        let mouse_interval_ms = validate_named_interval_ms("mouse interval", mouse_interval_ms)?;
        let keyboard_key = parse_virtual_key(keyboard_key_name)?;
        let keyboard_interval_ms =
            validate_named_interval_ms("keyboard interval", keyboard_interval_ms)?;

        Ok(Self {
            mouse_button,
            mouse_interval_ms,
            keyboard_key,
            keyboard_interval_ms,
        })
    }
}

pub fn validate_interval_ms(interval_ms: u64) -> Result<u64, String> {
    validate_named_interval_ms("interval", interval_ms)
}

fn validate_named_interval_ms(name: &str, interval_ms: u64) -> Result<u64, String> {
    if interval_ms < MIN_INTERVAL_MS {
        return Err(format!("{name} must be at least {MIN_INTERVAL_MS} ms"));
    }

    Ok(interval_ms)
}

pub fn parse_virtual_key(raw: &str) -> Result<VirtualKey, String> {
    let key = raw.trim();
    if key.is_empty() {
        return Err("keyboard key is required".to_string());
    }

    let upper = key.to_ascii_uppercase();
    if upper.len() == 1 {
        let byte = upper.as_bytes()[0];
        if byte.is_ascii_uppercase() || byte.is_ascii_digit() {
            return Ok(VirtualKey(byte as u16));
        }
    }

    match upper.as_str() {
        "SPACE" => Ok(VirtualKey(0x20)),
        "ENTER" | "RETURN" => Ok(VirtualKey(0x0D)),
        "ESC" | "ESCAPE" => Ok(VirtualKey(0x1B)),
        "TAB" => Ok(VirtualKey(0x09)),
        "BACKSPACE" | "BKSP" => Ok(VirtualKey(0x08)),
        "LEFT" | "LEFTARROW" => Ok(VirtualKey(0x25)),
        "UP" | "UPARROW" => Ok(VirtualKey(0x26)),
        "RIGHT" | "RIGHTARROW" => Ok(VirtualKey(0x27)),
        "DOWN" | "DOWNARROW" => Ok(VirtualKey(0x28)),
        "SHIFT" => Ok(VirtualKey(0x10)),
        "CTRL" | "CONTROL" => Ok(VirtualKey(0x11)),
        "ALT" => Ok(VirtualKey(0x12)),
        "CAPSLOCK" | "CAPS" => Ok(VirtualKey(0x14)),
        "F1" => Ok(VirtualKey(0x70)),
        "F2" => Ok(VirtualKey(0x71)),
        "F3" => Ok(VirtualKey(0x72)),
        "F4" => Ok(VirtualKey(0x73)),
        "F5" => Ok(VirtualKey(0x74)),
        "F6" => Ok(VirtualKey(0x75)),
        "F7" => Ok(VirtualKey(0x76)),
        "F8" => Ok(VirtualKey(0x77)),
        "F9" => Ok(VirtualKey(0x78)),
        "F10" => Ok(VirtualKey(0x79)),
        "F11" => Ok(VirtualKey(0x7A)),
        "F12" => Ok(VirtualKey(0x7B)),
        _ => Err(format!("unsupported keyboard key: {key}")),
    }
}
