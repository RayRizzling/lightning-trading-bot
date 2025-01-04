// src/futures/ticker.rs

use reqwest::header::HeaderMap;
use crate::utils::get_headers::get_headers;
use serde::Deserialize;
use std::error::Error;

// Struct to represent the data structure of the futures ticker response.
// The #[serde(rename_all = "camelCase")] attribute ensures that field names in the struct 
// are automatically converted from snake_case to camelCase to match the JSON response format.
/// Represents the data structure of the futures ticker response.
///
/// # Fields:
/// - `index`: The index value of the futures ticker.
/// - `last_price`: The last price at which the futures contract was traded.
/// - `ask_price`: The current ask price (the price at which sellers are willing to sell).
/// - `bid_price`: The current bid price (the price at which buyers are willing to buy).
/// - `carry_fee_rate`: The carry fee rate for the futures contract.
/// - `carry_fee_timestamp`: The timestamp when the carry fee rate was last updated.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct FuturesTicker {
    pub index: f64,              // The index value of the futures ticker.
    pub last_price: f64,         // The last traded price of the futures contract.
    pub ask_price: f64,          // The current ask price (price sellers are willing to sell).
    pub bid_price: f64,          // The current bid price (price buyers are willing to buy).
    pub carry_fee_rate: f64,     // The carry fee rate for the futures contract.
    pub carry_fee_timestamp: i64, // The timestamp representing the last update of the carry fee rate.
}

// Async function to fetch the futures ticker data from the API.
// This function makes an HTTP GET request to the `/futures/ticker` endpoint and processes the response.
/// Fetches the futures ticker data from the API endpoint.
///
/// # Parameters:
/// - `api_url`: The base URL of the API (e.g., "https://api.example.com").
///
/// # Returns:
/// - A `Result` containing the `FuturesTicker` struct if successful, or an error if the request fails.
pub async fn get_futures_ticker(
    api_url: &str,
) -> Result<FuturesTicker, Box<dyn Error>> {
    // Generate the necessary headers for the API request using the provided credentials.
    // The get_headers function handles the creation of headers like authentication and API signature.
    let headers: HeaderMap = get_headers("/v2/futures/ticker", "GET", None)?;

    // Create a new HTTP client to make requests.
    let client = reqwest::Client::new();

    // Send the GET request to the /futures/ticker endpoint of the API.
    let response = client
        .get(format!("{}/futures/ticker", api_url))  // Construct the full URL with the base API URL and endpoint.
        .headers(headers)  // Attach the necessary headers for authentication.
        .send()  // Send the request to the API.
        .await?;  // Await the response asynchronously.

    // Check if the response status code indicates success (200 OK).
    if response.status().is_success() {
        // If successful, retrieve the response body as text (JSON string).
        let body = response.text().await?;

        // Deserialize the JSON response body into the FuturesTicker struct.
        // The serde_json::from_str function converts the JSON string into the struct.
        let ticker: FuturesTicker = serde_json::from_str(&body)?;

        // Return the deserialized FuturesTicker struct.
        Ok(ticker)
    } else {
        // If the response is not successful, capture the error and return it.
        // The error is wrapped in a Box to handle different types of errors uniformly.
        let error: Box<dyn Error> = Box::new(response.error_for_status().unwrap_err());
        Err(error)
    }
}
