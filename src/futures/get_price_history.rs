// src/futures/get_price_history.rs

use reqwest::header::HeaderMap;
use crate::utils::get_headers::{encode_query_params, get_headers};
use crate::utils::get_timestamps::{format_timestamp, get_current_time_ms, get_time_n_days_ago_ms};
use serde::Deserialize;
use std::error::Error;
use std::io::Write;
use colored::Colorize;

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct PriceHistoryEntry {
    pub time: i64,    // The timestamp (in milliseconds) of the price entry
    pub value: f64,   // The price value at the corresponding time
}

// Fetches the price history from the API, allowing optional timestamps
pub async fn get_price_history(
    api_url: &str,
    from: Option<i64>, // Optional parameter for start timestamp
    to: Option<i64>,   // Optional parameter for end timestamp
    limit: Option<usize> // Optional parameter for result limit
) -> Result<Vec<PriceHistoryEntry>, Box<dyn Error>> {
    let to = to.unwrap_or_else(get_current_time_ms); // Default: now
    let from = from.unwrap_or_else(|| get_time_n_days_ago_ms(7)); // Default: 7 days ago
    let limit = limit.unwrap_or(1000); // Limit per request

    println!("{}", format!("Fetch price history from: {} - to: {}", format_timestamp(from), format_timestamp(to)).dimmed());

    let mut all_price_data: Vec<PriceHistoryEntry> = Vec::new();
    let current_from = from;
    let mut current_to = to;

    let mut total_time_span = 0i64;
    let mut request_count = 0usize;

    // Loop until we've fetched the full range
    while current_from < current_to {
        let params = serde_json::json!( {
            "from": current_from,
            "to": current_to,
            "limit": limit
        });
        let query_option = encode_query_params(&params);

        let headers: HeaderMap = get_headers("/v2/futures/history/price", "GET", query_option.as_deref())?;

        let client = reqwest::Client::new();
        let url = format!("{}/futures/history/price", api_url);

        let response = client
            .get(url)
            .headers(headers)
            .query(&params)
            .send()
            .await?;

        if response.status().is_success() {
            let body = response.text().await?;
            let price_history: Vec<PriceHistoryEntry> = serde_json::from_str(&body)?;

            // If no data is returned, break the loop
            if price_history.is_empty() {
                break;
            }

            // Add the fetched data to our collection
            all_price_data.extend(price_history.clone());

            // Get the oldest timestamp from the fetched data (last entry in the list)
            if let Some(last_entry) = price_history.last() {
                let last_time = last_entry.time;

                // Update the 'current_to' to the last time minus 1ms for the next API call
                current_to = last_time - 1;
            }

            // Only stop if the fetched range has covered enough time
            if current_to <= from {
                break;
            }

            let fetched_from = price_history.first().map(|e| e.time).unwrap_or(0);
            let fetched_to = price_history.last().map(|e| e.time).unwrap_or(0);

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

            // println!(
            //     "Fetched price history... from: {} to: {} - results length: {}",
            //     format_timestamp(fetched_from), format_timestamp(fetched_to), price_history.len()
            // );
            print!("\r{: <width$}", format!("...init price history: remaining ~ {} seconds", remaining_requests).dimmed(), width = 50);
            std::io::stdout().flush().unwrap();

        } else {
            let error: Box<dyn Error> = Box::new(response.error_for_status().unwrap_err());
            return Err(error);
        }

        // Sleep for a while to avoid hitting the API rate limits
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    println!("\r{: <width$}", "Price data retrieval complete.".green(), width = 50);
    Ok(all_price_data)
}
