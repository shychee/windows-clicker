use crate::config::ClickerConfig;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClickerSnapshot {
    pub config: ClickerConfig,
    pub mouse_armed: bool,
    pub keyboard_armed: bool,
}

#[derive(Debug, Clone)]
pub struct ClickerRuntime {
    config: ClickerConfig,
    mouse_armed: bool,
    keyboard_armed: bool,
}

impl ClickerRuntime {
    pub fn new(config: ClickerConfig) -> Self {
        Self {
            config,
            mouse_armed: false,
            keyboard_armed: false,
        }
    }

    pub fn snapshot(&self) -> ClickerSnapshot {
        ClickerSnapshot {
            config: self.config.clone(),
            mouse_armed: self.mouse_armed,
            keyboard_armed: self.keyboard_armed,
        }
    }

    pub fn toggle_mouse(&mut self) -> bool {
        self.mouse_armed = !self.mouse_armed;
        self.mouse_armed
    }

    pub fn toggle_keyboard(&mut self) -> bool {
        self.keyboard_armed = !self.keyboard_armed;
        self.keyboard_armed
    }

    pub fn stop_all(&mut self) {
        self.mouse_armed = false;
        self.keyboard_armed = false;
    }

    pub fn set_config(&mut self, config: ClickerConfig) {
        self.config = config;
    }
}
