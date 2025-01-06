// src/futures/close_all_trades.rs

use reqwest::{Client, header::HeaderMap};
use serde::Deserialize;
use std::error::Error;
use crate::utils::get_headers::get_headers;

// Represents the structure of the API response after attempting to close all trades.
// This struct holds various details about each trade, including fees, margin, and status.
#[derive(Deserialize, Debug)]
#[allow(dead_code)] // Used to prevent warnings about unused fields during development.
pub struct CloseAllTradesResponse {
    pub trades: Vec<CloseTradeResponse>,  // A list of trades that were closed
}

// Represents the structure of the response from closing a single trade.
// This is the same struct used in `close_trade.rs`.
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct CloseTradeResponse {
    pub uid: String,           // Unique identifier for the trade
    #[serde(rename = "type")]
    pub type_: String,         // Type of the trade (e.g., "buy", "sell")
    pub id: String,            // The trade ID
    pub side: String,          // The side of the trade ("buy" or "sell")
    pub opening_fee: f64,      // The fee incurred for opening the trade
    pub closing_fee: f64,      // The fee incurred for closing the trade
    pub maintenance_margin: f64, // Maintenance margin required for the trade
    pub quantity: f64,         // The quantity involved in the trade
    pub margin: f64,           // The margin used for the trade
    pub leverage: f64,         // The leverage applied to the trade
    pub price: f64,            // The price at which the trade was executed
    pub liquidation: f64,      // Liquidation price of the trade
    pub pl: f64,               // Profit or loss from the trade
    pub creation_ts: u64,      // Timestamp when the trade was created
    pub market_filled_ts: u64, // Timestamp when the trade was filled in the market
    pub closed_ts: Option<u64>, // Timestamp when the trade was closed
    pub open: bool,            // Indicates whether the trade is open
    pub running: bool,         // Indicates whether the trade is still running
    pub canceled: bool,        // Indicates whether the trade was canceled
    pub closed: bool,          // Indicates whether the trade was closed
    pub last_update_ts: u64,   // Timestamp of the last update on the trade
    pub sum_carry_fees: f64,   // The total carry fees for the trade
    pub entry_price: Option<f64>, // Entry price, if available
    pub entry_margin: Option<f64>, // Entry margin, if available
}

/// Asynchronously closes all running trades by sending a DELETE request to the API.
/// It does not require any parameters, as it will close all trades automatically.
///
/// # Arguments
/// - `api_url`: The base URL of the API endpoint to interact with.
///
/// # Returns
/// - A `Result` that contains the `CloseAllTradesResponse` if successful, or an error if the request fails.
pub async fn _close_all_trades(
    api_url: &str
) -> Result<CloseAllTradesResponse, Box<dyn Error>> {
    // Construct the URL for closing all trades.
    let url = format!("{}/futures/all/close", api_url);

    let headers: HeaderMap = get_headers("/v2/futures/all/close", "DELETE", None)?;

    let client = Client::new();
    let response = client
        .delete(&url)
        .headers(headers)
        .send()
        .await?;

    if response.status().is_success() {
        let close_all_trades_response: CloseAllTradesResponse = response.json().await?;
        Ok(close_all_trades_response)
    } else {
        let error_message = format!(
            "Error closing all trades: {} - {}",
            response.status(),
            response.text().await?
        );
        Err(error_message.into())
    }
}
