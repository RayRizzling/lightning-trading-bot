// src/tests/get_timestamps_tests.rs

use trading_backend::utils::get_timestamps::{get_current_time_ms, get_time_n_days_ago_ms, format_timestamp};

#[test]
fn test_get_current_time_ms() {
    let current_time = get_current_time_ms();
    assert!(current_time > 0);
}

#[test]
fn test_get_time_n_days_ago_ms() {
    let n = 7;
    let time_n_days_ago = get_time_n_days_ago_ms(n);
    let current_time_ms = get_current_time_ms();

    // Ensure that the timestamp is in the past
    assert!(time_n_days_ago < current_time_ms);

    // Check that the difference is approximately 7 days (in milliseconds)
    // 86400 seconds/day * 1000 milliseconds/second * n days
    let expected_difference = 86400 * 1000 * n;
    let actual_difference = current_time_ms - time_n_days_ago;

    // Allow for a small margin of error (e.g., 1 second)
    assert!((actual_difference - expected_difference).abs() <= 1000);
}

#[test]
fn test_format_timestamp() {
    let timestamp = 1734541932000; // Unix timestamp for 18th December 2024, 17:12:12 GMT
    let formatted_time = format_timestamp(timestamp);
    assert_eq!(formatted_time, "18.12.2024 - 17:12:12");
}
