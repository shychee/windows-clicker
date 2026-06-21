use windows_clicker::config::VirtualKey;
use windows_clicker::input::{
    clicker_hotkey_from_keyboard_event, hold_repeat_key_events, is_physical_keyboard_hook_event,
    is_physical_mouse_hook_event, keyboard_release_events, keyboard_scancode_from_virtual_key,
    tap_key_events, ClickerHotkey, KeyEventKind, LOW_LEVEL_KEYBOARD_INJECTED_FLAG,
    LOW_LEVEL_MOUSE_INJECTED_FLAG,
};

#[test]
fn hold_repeat_refreshes_the_key_and_leaves_it_down() {
    assert_eq!(
        hold_repeat_key_events(),
        [KeyEventKind::Up, KeyEventKind::Down]
    );
}

#[test]
fn tap_still_emits_down_and_up() {
    assert_eq!(tap_key_events(), [KeyEventKind::Down, KeyEventKind::Up]);
}

#[test]
fn keyboard_release_sends_final_key_up() {
    assert_eq!(keyboard_release_events(), [KeyEventKind::Up]);
}

#[test]
fn low_level_keyboard_hooks_ignore_injected_events() {
    assert!(is_physical_keyboard_hook_event(0));
    assert!(!is_physical_keyboard_hook_event(
        LOW_LEVEL_KEYBOARD_INJECTED_FLAG
    ));
    assert!(!is_physical_keyboard_hook_event(
        LOW_LEVEL_KEYBOARD_INJECTED_FLAG | 0x80
    ));
}

#[test]
fn low_level_mouse_hooks_ignore_injected_events() {
    assert!(is_physical_mouse_hook_event(0));
    assert!(!is_physical_mouse_hook_event(LOW_LEVEL_MOUSE_INJECTED_FLAG));
    assert!(!is_physical_mouse_hook_event(
        LOW_LEVEL_MOUSE_INJECTED_FLAG | 0x80
    ));
}

#[test]
fn clicker_hotkeys_fire_only_on_first_physical_keydown() {
    assert_eq!(
        clicker_hotkey_from_keyboard_event(0x75, true, false),
        Some(ClickerHotkey::Mouse)
    );
    assert_eq!(
        clicker_hotkey_from_keyboard_event(0x76, true, false),
        Some(ClickerHotkey::Keyboard)
    );
    assert_eq!(
        clicker_hotkey_from_keyboard_event(0x77, true, false),
        Some(ClickerHotkey::StopAll)
    );

    assert_eq!(clicker_hotkey_from_keyboard_event(0x76, true, true), None);
    assert_eq!(clicker_hotkey_from_keyboard_event(0x76, false, true), None);
    assert_eq!(clicker_hotkey_from_keyboard_event(0x41, true, false), None);
}

#[test]
fn maps_common_virtual_keys_to_keyboard_scancodes() {
    assert_eq!(
        keyboard_scancode_from_virtual_key(VirtualKey(0x4A)),
        Some(0x24)
    );
    assert_eq!(
        keyboard_scancode_from_virtual_key(VirtualKey(0x20)),
        Some(0x39)
    );
    assert_eq!(
        keyboard_scancode_from_virtual_key(VirtualKey(0x70)),
        Some(0x3B)
    );
    assert_eq!(keyboard_scancode_from_virtual_key(VirtualKey(0xFF)), None);
}
