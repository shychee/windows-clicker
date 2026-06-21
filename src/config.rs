pub const MIN_INTERVAL_MS: u64 = 25;
pub const MAX_CLICKS_PER_SECOND: u64 = 40;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpeedPreset {
    OnePerSecond,
    TwoPerSecond,
    FivePerSecond,
    TenPerSecond,
    TwentyPerSecond,
    ThirtyPerSecond,
}

impl SpeedPreset {
    pub const ALL: [SpeedPreset; 6] = [
        SpeedPreset::OnePerSecond,
        SpeedPreset::TwoPerSecond,
        SpeedPreset::FivePerSecond,
        SpeedPreset::TenPerSecond,
        SpeedPreset::TwentyPerSecond,
        SpeedPreset::ThirtyPerSecond,
    ];

    pub fn clicks_per_second(self) -> u64 {
        match self {
            SpeedPreset::OnePerSecond => 1,
            SpeedPreset::TwoPerSecond => 2,
            SpeedPreset::FivePerSecond => 5,
            SpeedPreset::TenPerSecond => 10,
            SpeedPreset::TwentyPerSecond => 20,
            SpeedPreset::ThirtyPerSecond => 30,
        }
    }

    pub fn interval_ms(self) -> u64 {
        interval_from_clicks_per_second(self.clicks_per_second())
            .expect("speed presets stay within supported range")
    }

    pub fn label_en(self) -> &'static str {
        match self {
            SpeedPreset::OnePerSecond => "1 / sec",
            SpeedPreset::TwoPerSecond => "2 / sec",
            SpeedPreset::FivePerSecond => "5 / sec",
            SpeedPreset::TenPerSecond => "10 / sec",
            SpeedPreset::TwentyPerSecond => "20 / sec",
            SpeedPreset::ThirtyPerSecond => "30 / sec",
        }
    }

    pub fn label_zh(self) -> &'static str {
        match self {
            SpeedPreset::OnePerSecond => "每秒 1 次",
            SpeedPreset::TwoPerSecond => "每秒 2 次",
            SpeedPreset::FivePerSecond => "每秒 5 次",
            SpeedPreset::TenPerSecond => "每秒 10 次",
            SpeedPreset::TwentyPerSecond => "每秒 20 次",
            SpeedPreset::ThirtyPerSecond => "每秒 30 次",
        }
    }
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

pub fn interval_from_clicks_per_second(clicks_per_second: u64) -> Result<u64, String> {
    if !(1..=MAX_CLICKS_PER_SECOND).contains(&clicks_per_second) {
        return Err(format!(
            "clicks per second must be between 1 and {MAX_CLICKS_PER_SECOND}"
        ));
    }

    validate_interval_ms(1000 / clicks_per_second)
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

pub fn virtual_key_display_name(key: VirtualKey) -> Option<&'static str> {
    match key.0 {
        0x30 => Some("0"),
        0x31 => Some("1"),
        0x32 => Some("2"),
        0x33 => Some("3"),
        0x34 => Some("4"),
        0x35 => Some("5"),
        0x36 => Some("6"),
        0x37 => Some("7"),
        0x38 => Some("8"),
        0x39 => Some("9"),
        0x41 => Some("A"),
        0x42 => Some("B"),
        0x43 => Some("C"),
        0x44 => Some("D"),
        0x45 => Some("E"),
        0x46 => Some("F"),
        0x47 => Some("G"),
        0x48 => Some("H"),
        0x49 => Some("I"),
        0x4A => Some("J"),
        0x4B => Some("K"),
        0x4C => Some("L"),
        0x4D => Some("M"),
        0x4E => Some("N"),
        0x4F => Some("O"),
        0x50 => Some("P"),
        0x51 => Some("Q"),
        0x52 => Some("R"),
        0x53 => Some("S"),
        0x54 => Some("T"),
        0x55 => Some("U"),
        0x56 => Some("V"),
        0x57 => Some("W"),
        0x58 => Some("X"),
        0x59 => Some("Y"),
        0x5A => Some("Z"),
        0x08 => Some("Backspace"),
        0x09 => Some("Tab"),
        0x0D => Some("Enter"),
        0x10 => Some("Shift"),
        0x11 => Some("Ctrl"),
        0x12 => Some("Alt"),
        0x14 => Some("CapsLock"),
        0x1B => Some("Esc"),
        0x20 => Some("Space"),
        0x25 => Some("Left"),
        0x26 => Some("Up"),
        0x27 => Some("Right"),
        0x28 => Some("Down"),
        0x70 => Some("F1"),
        0x71 => Some("F2"),
        0x72 => Some("F3"),
        0x73 => Some("F4"),
        0x74 => Some("F5"),
        0x75 => Some("F6"),
        0x76 => Some("F7"),
        0x77 => Some("F8"),
        0x78 => Some("F9"),
        0x79 => Some("F10"),
        0x7A => Some("F11"),
        0x7B => Some("F12"),
        _ => None,
    }
}
