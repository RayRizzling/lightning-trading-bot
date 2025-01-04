// src/futures/create_trade.rs

use reqwest::{Client, header::{HeaderMap, HeaderValue, CONTENT_TYPE}};
use serde::{Deserialize, Serialize};
use std::error::Error;
use crate::utils::get_headers::get_headers;

// Represents the parameters required to create a new trade.
#[derive(Serialize, Debug)]
pub struct CreateTradeParams {
    pub r#type: String,  // "m" for market, "l" for limit
    pub side: String,    // "b" for buy, "s" for sell
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin: Option<u64>,  // Optional: Margin amount (used if quantity is not provided) in Satoshis
    pub leverage: u64,   // Leverage to apply to the trade
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<u64>,  // Optional: Price for limit orders (required if type = "l")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,  // Optional: Quantity of the asset (used if margin is not provided) Quantity in USD (min tradable 1 USD)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub takeprofit: Option<u64>,  // Optional: Take-profit price for the trade
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stoploss: Option<u64>,  // Optional: Stop-loss price for the trade
}

// Represents the response from the create trade endpoint, detailing the created trade's parameters.
#[derive(Deserialize, Debug)]
#[allow(dead_code)] // Suppresses warnings for unused fields in the response struct.
pub struct TradeResponse {
    pub id: String,
    pub uid: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub side: String,
    pub opening_fee: u64,
    pub closing_fee: u64,
    pub maintenance_margin: u64,
    pub quantity: u64,
    pub margin: u64,
    pub leverage: u64,
    pub price: f64,
    pub liquidation: f64,
    pub stoploss: u64,
    pub takeprofit: u64,
    pub exit_price: Option<f64>,
    pub pl: u64,
    pub creation_ts: u64,
    pub market_filled_ts: Option<u64>,
    pub closed_ts: Option<u64>,
    pub open: bool,
    pub running: bool,
    pub canceled: bool,
    pub closed: bool,
    pub last_update_ts: u64,
    pub sum_carry_fees: u64,
    pub entry_price: Option<f64>,
    pub entry_margin: Option<u64>,
}

/// Creates a new trade on the server by sending the provided parameters.
/// 
/// # Arguments
/// - `api_url`: The base URL of the API endpoint.
/// - `params`: The parameters required to create the trade (including side, type, leverage, etc.).
/// 
/// # Returns
/// - `Ok(TradeResponse)`: If the trade was created successfully, returns the details of the created trade.
/// - `Err(Box<dyn Error>)`: If the request fails, returns an error message.
pub async fn create_trade(
    api_url: &str,
    params: CreateTradeParams
) -> Result<TradeResponse, Box<dyn Error>> {
    let params_json = serde_json::to_string(&params)?;
    println!("Request Body: {}", params_json);

    let mut headers: HeaderMap = get_headers("/v2/futures", "POST", Some(&params_json))?;
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    
    let client = Client::new();

    // Construct the URL for the 'create trade' API endpoint.
    let url = format!("{}/futures", api_url);

    // Send the POST request with the trade parameters in the body of the request.
    let response = client
        .post(&url)
        .headers(headers)
        .body(params_json)    
        .send()
        .await?;

    if response.status().is_success() {
        let trade: TradeResponse = response.json().await?;
        Ok(trade)
    } else {
        let error_message = format!("Error creating trade: {}", response.status());
        Err(error_message.into())
    }
}

/// Creates a limit buy order on the server.
/// This function constructs a `CreateTradeParams` struct for a limit buy order and calls `create_trade`.
/// 
/// # Arguments
/// - `api_url`: The base URL of the API endpoint.
/// - `leverage`: The leverage to apply to the trade.
/// - `price`: The limit price for the buy order.
/// - `quantity`: Optional quantity of the asset to buy.
/// - `takeprofit`: Optional take-profit price for the order.
/// - `stoploss`: Optional stop-loss price for the order.
/// 
/// # Returns
/// - `Result<TradeResponse, Box<dyn Error>>`: The response from the API call, or an error if the trade could not be created.
pub async fn create_limit_buy_order(
    api_url: &str,
    leverage: u64,
    price: u64,
    quantity: Option<u64>,
    takeprofit: Option<u64>,
    stoploss: Option<u64>
) -> Result<TradeResponse, Box<dyn Error>> {
    let params = CreateTradeParams {
        side: "b".to_string(),  // "b" indicates a buy order.
        r#type: "l".to_string(),  // "l" indicates a limit order.
        margin: None,
        leverage,
        price: Some(price), // Limit order requires a price.
        quantity,
        takeprofit,
        stoploss,
    };
    // Delegate the actual trade creation to the `create_trade` function.
    create_trade(api_url, params).await
}

/// Creates a limit sell order on the server.
/// This function constructs a `CreateTradeParams` struct for a limit sell order and calls `create_trade`.
/// 
/// # Arguments
/// - `api_url`: The base URL of the API endpoint.
/// - `leverage`: The leverage to apply to the trade.
/// - `price`: The limit price for the sell order.
/// - `quantity`: Optional quantity of the asset to sell.
/// - `takeprofit`: Optional take-profit price for the order.
/// - `stoploss`: Optional stop-loss price for the order.
/// 
/// # Returns
/// - `Result<TradeResponse, Box<dyn Error>>`: The response from the API call, or an error if the trade could not be created.
pub async fn create_limit_sell_order(
    api_url: &str,
    leverage: u64,
    price: u64,
    quantity: Option<u64>,
    takeprofit: Option<u64>,
    stoploss: Option<u64>
) -> Result<TradeResponse, Box<dyn Error>> {
    let params = CreateTradeParams {
        side: "s".to_string(),  // "s" indicates a sell order.
        r#type: "l".to_string(),  // "l" indicates a limit order.
        margin: None,
        leverage,
        price: Some(price),
        quantity,
        takeprofit,
        stoploss,
    };
    // Delegate the actual trade creation to the `create_trade` function.
    create_trade(api_url, params).await
}

/// Creates a market buy order on the server.
/// This function constructs a `CreateTradeParams` struct for a market buy order and calls `create_trade`.
/// 
/// # Arguments
/// - `api_url`: The base URL of the API endpoint.
/// - `leverage`: The leverage to apply to the trade.
/// - `quantity`: Optional quantity of the asset to buy.
/// - `margin`: Optional margin to apply to the trade.
/// - `takeprofit`: Optional take-profit price for the order.
/// - `stoploss`: Optional stop-loss price for the order.
/// 
/// # Returns
/// - `Result<TradeResponse, Box<dyn Error>>`: The response from the API call, or an error if the trade could not be created.
pub async fn create_market_buy_order(
    api_url: &str,
    leverage: u64,
    quantity: Option<u64>,
    margin: Option<u64>,
    price: Option<u64>,
    takeprofit: Option<u64>,
    stoploss: Option<u64>
) -> Result<TradeResponse, Box<dyn Error>> {
    let params = CreateTradeParams {
        side: "b".to_string(),  // "b" indicates a buy order.
        r#type: "m".to_string(),  // "m" indicates a market order.
        margin,
        leverage,
        price, // No price for market orders.
        quantity,
        takeprofit,
        stoploss,
    };
    // Delegate the actual trade creation to the `create_trade` function.
    create_trade(api_url, params).await
}

/// Creates a market sell order on the server.
/// This function constructs a `CreateTradeParams` struct for a market sell order and calls `create_trade`.
/// 
/// # Arguments
/// - `api_url`: The base URL of the API endpoint.
/// - `leverage`: The leverage to apply to the trade.
/// - `quantity`: Optional quantity of the asset to sell.
/// - `margin`: Optional margin to apply to the trade.
/// - `takeprofit`: Optional take-profit price for the order.
/// - `stoploss`: Optional stop-loss price for the order.
/// 
/// # Returns
/// - `Result<TradeResponse, Box<dyn Error>>`: The response from the API call, or an error if the trade could not be created.
pub async fn create_market_sell_order(
    api_url: &str,
    leverage: u64,
    quantity: Option<u64>,
    margin: Option<u64>,
    takeprofit: Option<u64>,
    stoploss: Option<u64>
) -> Result<TradeResponse, Box<dyn Error>> {
    let params = CreateTradeParams {
        side: "s".to_string(),  // "s" indicates a sell order.
        r#type: "m".to_string(),  // "m" indicates a market order.
        margin,
        leverage,
        price: None, // No price for market orders.
        quantity,
        takeprofit,
        stoploss,
    };
    // Delegate the actual trade creation to the `create_trade` function.
    create_trade(api_url, params).await
}
