use windows_clicker::config::{interval_from_clicks_per_second, SpeedPreset};

#[test]
fn converts_clicks_per_second_to_interval_ms() {
    assert_eq!(interval_from_clicks_per_second(1).unwrap(), 1000);
    assert_eq!(interval_from_clicks_per_second(2).unwrap(), 500);
    assert_eq!(interval_from_clicks_per_second(5).unwrap(), 200);
    assert_eq!(interval_from_clicks_per_second(10).unwrap(), 100);
    assert_eq!(interval_from_clicks_per_second(20).unwrap(), 50);
    assert_eq!(interval_from_clicks_per_second(30).unwrap(), 33);
}

#[test]
fn rejects_zero_or_too_fast_clicks_per_second() {
    assert_eq!(
        interval_from_clicks_per_second(0).unwrap_err(),
        "clicks per second must be between 1 and 40"
    );
    assert_eq!(
        interval_from_clicks_per_second(41).unwrap_err(),
        "clicks per second must be between 1 and 40"
    );
}

#[test]
fn preset_labels_are_user_friendly() {
    assert_eq!(SpeedPreset::TenPerSecond.label_en(), "10 / sec");
    assert_eq!(SpeedPreset::TenPerSecond.label_zh(), "每秒 10 次");
    assert_eq!(SpeedPreset::TenPerSecond.interval_ms(), 100);
    assert_eq!(SpeedPreset::ThirtyPerSecond.label_en(), "30 / sec");
    assert_eq!(SpeedPreset::ThirtyPerSecond.label_zh(), "每秒 30 次");
}
