// src/futures/get_ohlc_history.rs

use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};
use std::io::Write;
use colored::Colorize;
use tokio::time::Duration;

use crate::utils::get_headers::get_headers;
use crate::utils::get_headers::encode_query_params;
use crate::utils::get_timestamps::format_timestamp;
use crate::utils::get_timestamps::get_current_time_ms;
use crate::utils::get_timestamps::get_time_n_days_ago_ms;

/// Represents a single OHLC entry, containing the timestamp and the open, high, low, and close values.
#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct OhlcHistoryEntry {
    pub time: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

/// Represents the query parameters for fetching OHLC history.
#[derive(Serialize, Debug)]
pub struct GetOhlcsParams<'a> {
    pub range: &'a str,  // The range (e.g., "1D", "1H")
    pub from: i64,       // The start timestamp
    pub to: i64,         // The end timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>, // Optional: Maximum number of entries (default: 100)
    pub debug: bool,
}

impl<'a> Default for GetOhlcsParams<'a> {
    fn default() -> Self {
        // Get the current timestamp from the utility function
        let current_timestamp = get_current_time_ms();

        // 7 days ago using the utility function
        let seven_days_ago = get_time_n_days_ago_ms(7);

        // Return the default parameters
        Self {
            range: "1D", // Default to 1 day range
            from: seven_days_ago,
            to: current_timestamp,
            limit: Some(100), // Default limit to 100
            debug: true
        }
    }
}

pub async fn get_ohlcs_history(
    api_url: &str,
    params: GetOhlcsParams<'_>,
) -> Result<Vec<OhlcHistoryEntry>, Box<dyn std::error::Error>> {

    let mut all_ohlc_data: Vec<OhlcHistoryEntry> = Vec::new();
    let mut current_from = params.from;
    let current_to = params.to;
    let limit = params.limit.unwrap_or(1000);

    let mut total_time_span = 0i64;
    let mut request_count = 0usize;

    if params.debug {
        println!("{}", format!("Fetch OHLC history from: {} - to: {}", format_timestamp(current_from), format_timestamp(current_to)).dimmed());
    }

    let client = reqwest::Client::new();

    while current_from < current_to {
        let params = GetOhlcsParams {
            range: params.range,
            from: current_from,
            to: current_to,
            limit: Some(limit),
            debug: true
        };
        
        let query_option = encode_query_params(&params);
        let headers: HeaderMap = get_headers("/v2/futures/ohlcs", "GET", query_option.as_deref())?;

        let url = format!("{}/futures/ohlcs?{}", api_url, query_option.unwrap_or_default());

        let response = client
            .get(url)
            .headers(headers)
            .send()
            .await?;

        if response.status().is_success() {
            let response_text = response.text().await?;
            let ohlc_history: Vec<OhlcHistoryEntry> = serde_json::from_str(&response_text)?;

            if ohlc_history.is_empty() {
                break;
            }

            all_ohlc_data.extend(ohlc_history.clone());

            if let Some(last_entry) = ohlc_history.last() {
                current_from = last_entry.time + 1;
            }


            let fetched_from = ohlc_history.first().map(|e| e.time).unwrap_or(0);
            let fetched_to = ohlc_history.last().map(|e| e.time).unwrap_or(0);
            let remaining_time = current_to - current_from;
            let current_time_span = fetched_to - fetched_from;

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
   
            if params.debug {
                print!("\r{: <width$}", format!("...init OHLCs history: remaining ~ {} seconds", remaining_requests).dimmed(), width = 50);
                std::io::stdout().flush().unwrap();
            }
        } else {
            let error_message = format!(
                "Failed to fetch OHLC history: {} - {:?}",
                response.status(),
                response.text().await?
            );
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                error_message
            )));
        }

        if current_to <= params.from {
            break;
        }

        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    if params.debug {
        println!("\r{: <width$}", "OHLCs data retrieval complete.".green(), width = 50);
    }
    Ok(all_ohlc_data)
}
