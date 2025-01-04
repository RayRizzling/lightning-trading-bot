// src/math/update_data.rs

//TO DO: add logic for update price and index data (history) on option

use std::time::{SystemTime, UNIX_EPOCH};
use std::{sync::Arc, error::Error};
use tokio::sync::{Mutex, mpsc};
use tokio::time::{self, Duration};
use crate::futures::get_ohlcs_history::{get_ohlcs_history, GetOhlcsParams, OhlcHistoryEntry};
use crate::utils::get_timestamps::get_current_time_ms;

pub async fn update_data(
    api_url: &str,
    interval: Duration,
    ohlc_data: Arc<Mutex<Vec<OhlcHistoryEntry>>>,
    range: &str,
    tx: mpsc::Sender<Vec<OhlcHistoryEntry>>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let initial_delay = calculate_initial_delay(interval);
    tokio::time::sleep(initial_delay).await;
    let mut interval_timer = time::interval(interval);

    loop {
        interval_timer.tick().await;

        let mut ohlc_data_lock = ohlc_data.lock().await;

        let data_length = ohlc_data_lock.len();

        let from = ohlc_data_lock.last().map(|entry| entry.time).unwrap_or(0);

        let ohlc_params = GetOhlcsParams {
            range,
            from,
            to: get_current_time_ms(),
            limit: Some(1000),
            debug: false
        };

        match get_ohlcs_history(api_url, ohlc_params).await {
            Ok(mut new_data) => {
                new_data.retain(|entry| entry.time > from);

                if !new_data.is_empty() {
                    ohlc_data_lock.extend(new_data);

                    if ohlc_data_lock.len() > data_length {
                        ohlc_data_lock.reverse();
                        ohlc_data_lock.truncate(data_length);
                        ohlc_data_lock.reverse();
                    }

                    let tx_clone = tx.clone();
                    let ohlc_data_clone = ohlc_data.clone();

                    tokio::spawn(async move {
                        let ohlc_data_lock = ohlc_data_clone.lock().await;
                        if let Err(e) = tx_clone.send(ohlc_data_lock.clone()).await {
                            eprintln!("Error sending updated OHLC data: {}", e);
                        }
                    });
                }
            }
            Err(e) => {
                let error_message = format!("Error updating OHLC data: {}", e);
                eprintln!("{}", error_message);
            }
        }
    }
}

fn calculate_initial_delay(interval: Duration) -> Duration {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("System time should be valid");
    let now_secs = now.as_secs();

    let interval_secs = interval.as_secs();

    let next_aligned_time = match interval_secs {
        60 | 180 | 300 | 600 | 900 | 1800 | 2700 | 3600 => {
            ((now_secs / interval_secs) + 1) * interval_secs
        }
        86400 => {
            let days_since_epoch = now_secs / 86400;
            (days_since_epoch + 1) * 86400
        }
        2592000 => {
            let months_since_epoch = now_secs / 2592000;
            (months_since_epoch + 1) * 2592000
        }
        7776000 => {
            let quarters_since_epoch = now_secs / 7776000;
            (quarters_since_epoch + 1) * 7776000
        }
        _ => {
            ((now_secs / interval_secs) + 1) * interval_secs
        }
    };

    Duration::from_secs(next_aligned_time - now_secs) + Duration::from_millis(1000)
}
