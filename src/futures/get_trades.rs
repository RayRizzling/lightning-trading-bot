use reqwest::{Client, header::HeaderMap};
use serde::{Deserialize, Serialize};
use std::error::Error;
use crate::utils::get_headers::get_headers;
use crate::utils::get_headers::encode_query_params;

/// Represents a single trade entry fetched from the API.
/// This structure holds detailed information about a specific trade position, such as its type, side, fees, leverage, and status.
#[derive(Deserialize, Debug, Default)]
#[allow(dead_code)]
pub struct TradeEntry {
    pub uid: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub id: String,
    pub side: String,
    pub opening_fee: f64,
    pub closing_fee: f64,
    pub maintenance_margin: f64,
    pub quantity: f64,
    pub margin: f64,
    pub leverage: f64,
    pub price: f64,
    pub liquidation: f64,
    pub stoploss: f64,
    pub takeprofit: f64,
    pub pl: f64,
    pub creation_ts: u64,
    pub market_filled_ts: u64,
    pub open: bool,
    pub running: bool,
    pub canceled: bool,
    pub closed: bool,
    pub last_update_ts: u64,
    pub sum_carry_fees: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entry_price: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entry_margin: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exit_price: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub closed_ts: Option<u64>,
}

/// Represents the query parameters for fetching trades.
/// These parameters can be used to filter and limit the results returned by the API.
#[derive(Serialize)]
pub struct GetTradesParams<'a> {
    pub r#type: &'a str,  // The trade type filter (e.g., "open", "closed"). This field is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<u64>,         // Optional: Start timestamp for fetching trades.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<u64>,           // Optional: End timestamp for fetching trades.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,        // Optional: Limit the number of trades (default is 100, max 1000).
}

/// Default implementation for GetTradesParams, used if no specific query parameters are provided.
/// This will fetch 'open' trades by default.
impl<'a> Default for GetTradesParams<'a> {
    fn default() -> Self {
        Self {
            r#type: "open", // Default to fetching open trades.
            from: None,
            to: None,
            limit: None,
        }
    }
}

/// Fetches all trades for a user from the API, with optional query parameters.
/// 
/// # Arguments
/// - `api_url`: The base URL of the API endpoint.
/// - `params`: Optional query parameters for filtering trades (e.g., 'from', 'to', 'limit').
///
/// # Returns
/// - `Ok(Vec<TradeEntry>)`: A list of trade entries fetched from the API.
/// - `Err(Box<dyn Error>)`: An error message if the API request fails.
pub async fn get_trades(
    api_url: &str,
    params: Option<GetTradesParams<'_>> // Query parameters for fetching trades (optional).
) -> Result<Vec<TradeEntry>, Box<dyn Error>> {
    // If no parameters are provided, use default values for the query parameters.
    let params = params.unwrap_or_default();
    
    // Convert the query parameters into a query string.
    let query_option = encode_query_params(&params);
    
    // Generate the necessary headers for the API request, including authorization and other headers.
    let headers: HeaderMap = get_headers("/v2/futures", "GET", query_option.as_deref())?;
    
    let client = Client::new();
    
    let url = format!("{}/futures", api_url);
    
    let response = client
        .get(url)
        .headers(headers)
        .query(&params)
        .send()
        .await?;
    
    if response.status().is_success() {
        let trades: Vec<TradeEntry> = response.json().await?;
        Ok(trades)
    } else {
        let error_message = format!("Error fetching trades: {}", response.status());
        Err(error_message.into())
    }
}