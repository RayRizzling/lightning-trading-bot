// src/futures/close_trade.rs

use reqwest::{Client, header::HeaderMap}; 
use serde::{Deserialize, Serialize}; 
use std::error::Error; 
use crate::utils::get_headers::{get_headers, encode_query_params};

/// Represents the structure of the API response after attempting to close a trade.
/// This struct holds various details about the trade, including fees, margin, and status.
#[derive(Deserialize, Debug)]
#[allow(dead_code)] // Used to prevent warnings about unused fields during development.
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
    pub open: bool,            // Indicates whether the trade is open
    pub running: bool,         // Indicates whether the trade is still running
    pub canceled: bool,        // Indicates whether the trade was canceled
    pub closed: bool,          // Indicates whether the trade was closed
    pub last_update_ts: u64,   // Timestamp of the last update on the trade
    pub sum_carry_fees: f64,   // The total carry fees for the trade
    pub entry_price: Option<f64>, // Entry price, if available
    pub entry_margin: Option<f64>, // Entry margin, if available
}

/// Represents the query parameters required to close a trade via the API.
/// This struct ensures the `id` of the trade is passed in the request to identify which trade to cancel.
#[derive(Serialize)]
pub struct CloseTradeParams<'a> {
    pub id: &'a str,  // The trade ID to be closed
}

/// Asynchronously closes a trade by sending a DELETE request to the API.
/// It constructs the appropriate query parameters, sends the request, and processes the response.
/// 
/// # Arguments
/// - `api_url`: The base URL of the API endpoint to interact with.
/// - `trade_id`: The ID of the trade to close.
/// 
/// # Returns
/// - A `Result` that contains the `CloseTradeResponse` if successful, or an error if the request fails.
pub async fn _close_trade(
    api_url: &str,
    trade_id: &str,   // The ID of the trade to closse
) -> Result<CloseTradeResponse, Box<dyn Error>> {
    let params = CloseTradeParams { id: trade_id };

    let query_string = encode_query_params(&params).ok_or_else(|| {
        "Failed to encode query parameters".to_string()
    })?;

    let headers: HeaderMap = get_headers("/v2/futures", "DELETE", Some(&query_string))?;

    let url = format!("{}/futures?{}", api_url, query_string);

    let client = Client::new();
    let response = client
        .delete(&url)    
        .headers(headers) 
        .send()      
        .await?;

    if response.status().is_success() {
        let closed_trade: CloseTradeResponse = response.json().await?;
        Ok(closed_trade)
    } else {
        let error_message = format!(
            "Error canceling trade: {} - {}",
            response.status(),
            response.text().await?
        );
        Err(error_message.into())
    }
}
