use reqwest::{Client, header::HeaderMap};
use serde::Deserialize;
use std::error::Error;
use crate::utils::get_headers::get_headers;

/// Struct to represent the market data response from the API
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct FuturesMarket {
    pub active: bool,
    pub limits: Limits,
    pub fees: Fees,
}

/// Sub-struct for limits
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Limits {
    pub quantity: MinMax,
    pub leverage: MinMax,
    pub count: CountLimit,
}

/// Struct for min and max values
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct MinMax {
    pub min: u64,
    pub max: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade: Option<u64>,
}

/// Struct for count limits
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct CountLimit {
    pub max: u64,
}

/// Struct for fees
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Fees {
    pub carry: CarryFee,
    pub trading: TradingFees,
}

/// Carry fees structure
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct CarryFee {
    pub min: f64,
    pub hours: Vec<u8>,
}

/// Trading fees structure
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct TradingFees {
    pub tiers: Vec<Tier>,
}

/// Struct for individual fee tiers
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct Tier {
    pub min_volume: u64,
    pub fees: f64,
}

/// Fetches futures market details from the API
///
/// # Arguments
/// - `api_url`: The base URL of the API endpoint.
/// 
/// # Returns
/// - `Ok(FuturesMarket)` if the request succeeds and data is parsed.
/// - `Err(Box<dyn Error>)` if the request fails or parsing fails.
pub async fn get_market(api_url: &str) -> Result<FuturesMarket, Box<dyn Error>> {
    // Generate the required headers for the API request
    let headers: HeaderMap = get_headers("/v2/futures/market", "GET", None)?;

    // Create a new HTTP client
    let client = Client::new();

    // Construct the full URL
    let url = format!("{}{}", api_url, "/futures/market");

    // Send the GET request with headers
    let response = client
        .get(&url)
        .headers(headers)
        .send()
        .await?;

    // Check for a successful response and deserialize JSON
    if response.status().is_success() {
        let response_text = response.text().await?;
        let market_data: FuturesMarket = serde_json::from_str(&response_text)?;
        Ok(market_data)
    } else {
        let error_message = format!("Failed to fetch market data: {}", response.status());
        Err(error_message.into())
    }
}
