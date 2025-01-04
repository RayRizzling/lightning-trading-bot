// src/futures/get_index_history.rs

use reqwest::header::HeaderMap;
use crate::utils::get_headers::{encode_query_params, get_headers};
use crate::utils::get_timestamps::{format_timestamp, get_current_time_ms, get_time_n_days_ago_ms};
use serde::Deserialize;
use std::error::Error;
use std::io::Write;
use colored::Colorize;

// Struct to represent a single entry in the index history, containing the timestamp and value
#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct IndexHistoryEntry {
    pub time: i64,    // The timestamp (in milliseconds) of the index entry
    pub value: f64,   // The index value at the corresponding timestamp
}

// Function to retrieve the index history from the API.
pub async fn get_index_history(
    api_url: &str,
    from: Option<i64>, // Optional parameter for start timestamp
    to: Option<i64>,   // Optional parameter for end timestamp
    limit: Option<usize> // Optional parameter for result limit
) -> Result<Vec<IndexHistoryEntry>, Box<dyn Error>> {
    let to = to.unwrap_or_else(get_current_time_ms); // Default: now
    let from = from.unwrap_or_else(|| get_time_n_days_ago_ms(7)); // Default: 7 days ago
    let limit = limit.unwrap_or(1000);

    println!("{}", format!("Fetch index history from: {} - to: {}", format_timestamp(from), format_timestamp(to)).dimmed());

    let mut all_index_data: Vec<IndexHistoryEntry> = Vec::new();
    let current_from = from;
    let mut current_to = to;

    let mut total_time_span = 0i64;
    let mut request_count = 0usize;

    // Loop until we've fetched the full range
    while current_from < current_to {
        let params = serde_json::json!({
            "from": current_from,
            "to": current_to,
            "limit": limit
        });
        let query_option = encode_query_params(&params);

        let headers: HeaderMap = get_headers("/v2/futures/history/index", "GET", query_option.as_deref())?;

        let client = reqwest::Client::new();
        let url = format!("{}/futures/history/index", api_url);

        let response = client
            .get(url)
            .headers(headers)
            .query(&params)
            .send()
            .await?;

        if response.status().is_success() {
            let body = response.text().await?;
            let index_history: Vec<IndexHistoryEntry> = serde_json::from_str(&body)?;

            // If no data is returned, break the loop
            if index_history.is_empty() {
                break;
            }

            // Add the fetched data to our collection
            all_index_data.extend(index_history.clone());

            // Get the oldest timestamp from the fetched data (last entry in the list)
            if let Some(last_entry) = index_history.last() {
                let last_time = last_entry.time;

                // Update the 'current_to' to the last time minus 1ms for the next API call
                current_to = last_time - 1;
            }

            // Only stop if the fetched range has covered enough time
            if current_to <= from {
                break;
            }

            let fetched_from = index_history.first().map(|e| e.time).unwrap_or(0);
            let fetched_to = index_history.last().map(|e| e.time).unwrap_or(0);

            let remaining_time = current_to - current_from;
            let current_time_span = fetched_from - fetched_to;

            total_time_span += current_time_span;
            request_count += 1;

            let avg_time_span = if request_count > 0 {
                total_time_span as f64 / request_count as f64
            } else {
                0.0
            };

            let remaining_requests = if avg_time_span > 0.0 {
                (remaining_time as f64 / avg_time_span).ceil() as usize
            } else {
                0
            };

            print!("\r{: <width$}", format!("...init index history: remaining ~ {} seconds", remaining_requests).dimmed(), width = 50);
            std::io::stdout().flush().unwrap();

        } else {
            let error: Box<dyn Error> = Box::new(response.error_for_status().unwrap_err());
            return Err(error);
        }

        // Sleep for a while to avoid hitting the API rate limits
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    println!("\r{: <width$}", "Index data retrieval complete.".green(), width = 50);
    Ok(all_index_data)
}
