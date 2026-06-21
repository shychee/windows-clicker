use crate::config::ClickerConfig;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClickerSnapshot {
    pub config: ClickerConfig,
    pub mouse_running: bool,
    pub keyboard_running: bool,
}

#[derive(Debug, Clone)]
pub struct ClickerRuntime {
    config: ClickerConfig,
    mouse_running: bool,
    keyboard_running: bool,
}

impl ClickerRuntime {
    pub fn new(config: ClickerConfig) -> Self {
        Self {
            config,
            mouse_running: false,
            keyboard_running: false,
        }
    }

    pub fn snapshot(&self) -> ClickerSnapshot {
        ClickerSnapshot {
            config: self.config.clone(),
            mouse_running: self.mouse_running,
            keyboard_running: self.keyboard_running,
        }
    }

    pub fn toggle_mouse(&mut self) -> bool {
        self.mouse_running = !self.mouse_running;
        self.mouse_running
    }

    pub fn toggle_keyboard(&mut self) -> bool {
        self.keyboard_running = !self.keyboard_running;
        self.keyboard_running
    }

    pub fn stop_all(&mut self) {
        self.mouse_running = false;
        self.keyboard_running = false;
    }

    pub fn set_config(&mut self, config: ClickerConfig) {
        self.config = config;
    }
}
