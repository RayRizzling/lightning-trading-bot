// src/utils/get_timestamps.rs

use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{TimeZone, Utc};

// Helper function to get the current timestamp in milliseconds
pub fn get_current_time_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}

pub fn get_time_n_days_ago_ms(days: i64) -> i64 {
    get_current_time_ms() - (days * 24 * 60 * 60 * 1000)
}

pub fn _get_time_n_minutes_ago_ms(minutes: i64) -> i64 {
    get_current_time_ms() - (minutes * 60 * 1000)
}

pub fn format_timestamp(timestamp: i64) -> String {
    let naive_datetime_opt = Utc.timestamp_opt(timestamp / 1000, (timestamp % 1000) as u32 * 1_000_000);

    match naive_datetime_opt.single() {
        Some(naive_datetime) => {
            naive_datetime.format("%d.%m.%Y - %H:%M:%S").to_string()
        }
        None => {
            "Invalid timestamp".to_string()
        }
    }
}
