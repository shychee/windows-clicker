use windows_clicker::config::{
    parse_virtual_key, validate_interval_ms, ClickerConfig, MouseButton, VirtualKey,
    MIN_INTERVAL_MS,
};

#[test]
fn interval_below_minimum_is_rejected() {
    let err = validate_interval_ms(MIN_INTERVAL_MS - 1).unwrap_err();

    assert_eq!(
        err,
        format!("interval must be at least {MIN_INTERVAL_MS} ms")
    );
}

#[test]
fn interval_at_minimum_is_valid() {
    assert_eq!(
        validate_interval_ms(MIN_INTERVAL_MS).unwrap(),
        MIN_INTERVAL_MS
    );
}

#[test]
fn parses_letters_digits_and_named_keys() {
    assert_eq!(parse_virtual_key("a").unwrap(), VirtualKey(0x41));
    assert_eq!(parse_virtual_key("Z").unwrap(), VirtualKey(0x5A));
    assert_eq!(parse_virtual_key("7").unwrap(), VirtualKey(0x37));
    assert_eq!(parse_virtual_key("space").unwrap(), VirtualKey(0x20));
    assert_eq!(parse_virtual_key("Enter").unwrap(), VirtualKey(0x0D));
    assert_eq!(parse_virtual_key("f12").unwrap(), VirtualKey(0x7B));
    assert_eq!(parse_virtual_key("left").unwrap(), VirtualKey(0x25));
}

#[test]
fn rejects_empty_or_unknown_key_names() {
    assert_eq!(
        parse_virtual_key("").unwrap_err(),
        "keyboard key is required"
    );
    assert_eq!(
        parse_virtual_key("not-a-key").unwrap_err(),
        "unsupported keyboard key: not-a-key"
    );
}

#[test]
fn config_constructor_validates_intervals_and_keys() {
    let config = ClickerConfig::new(MouseButton::Left, 50, "q", 75).unwrap();

    assert_eq!(config.mouse_button, MouseButton::Left);
    assert_eq!(config.mouse_interval_ms, 50);
    assert_eq!(config.keyboard_key, VirtualKey(0x51));
    assert_eq!(config.keyboard_interval_ms, 75);
}

#[test]
fn config_constructor_rejects_bad_values() {
    let err = ClickerConfig::new(MouseButton::Right, 1, "q", 75).unwrap_err();

    assert_eq!(
        err,
        format!("mouse interval must be at least {MIN_INTERVAL_MS} ms")
    );

    let err = ClickerConfig::new(MouseButton::Middle, 50, "", 75).unwrap_err();

    assert_eq!(err, "keyboard key is required");
}
