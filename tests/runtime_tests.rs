use windows_clicker::config::{ClickerConfig, MouseButton, VirtualKey};
use windows_clicker::runtime::ClickerRuntime;

#[test]
fn starts_with_both_clickers_stopped() {
    let runtime = ClickerRuntime::new(sample_config());

    let snapshot = runtime.snapshot();

    assert!(!snapshot.mouse_armed);
    assert!(!snapshot.keyboard_armed);
    assert_eq!(snapshot.config.mouse_button, MouseButton::Left);
}

#[test]
fn toggles_mouse_and_keyboard_armed_independently() {
    let mut runtime = ClickerRuntime::new(sample_config());

    assert!(runtime.toggle_mouse());
    assert!(runtime.snapshot().mouse_armed);
    assert!(!runtime.snapshot().keyboard_armed);

    assert!(runtime.toggle_keyboard());
    assert!(runtime.snapshot().mouse_armed);
    assert!(runtime.snapshot().keyboard_armed);

    assert!(!runtime.toggle_mouse());
    assert!(!runtime.snapshot().mouse_armed);
    assert!(runtime.snapshot().keyboard_armed);
}

#[test]
fn stop_all_turns_off_both_clickers() {
    let mut runtime = ClickerRuntime::new(sample_config());
    runtime.toggle_mouse();
    runtime.toggle_keyboard();

    runtime.stop_all();

    let snapshot = runtime.snapshot();
    assert!(!snapshot.mouse_armed);
    assert!(!snapshot.keyboard_armed);
}

#[test]
fn updating_config_preserves_running_state() {
    let mut runtime = ClickerRuntime::new(sample_config());
    runtime.toggle_mouse();

    let next = ClickerConfig::new(MouseButton::Right, 80, "space", 90).unwrap();
    runtime.set_config(next);

    let snapshot = runtime.snapshot();
    assert!(snapshot.mouse_armed);
    assert!(!snapshot.keyboard_armed);
    assert_eq!(snapshot.config.mouse_button, MouseButton::Right);
    assert_eq!(snapshot.config.keyboard_key, VirtualKey(0x20));
}

fn sample_config() -> ClickerConfig {
    ClickerConfig::new(MouseButton::Left, 50, "q", 75).unwrap()
}
