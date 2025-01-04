// src/utils/get_headers.rs

use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use chrono::Utc;
use crate::utils::get_signature::generate_signature;
use std::error::Error;
use serde::Serialize;
use dotenv::dotenv;
use std::env;

const USER_AGENT_VALUE: &str = "0x41 Labs Rust Bot";

// Function to generate the required headers for API requests
///
/// # Parameters:
/// - `endpoint`: The endpoint for which the headers are being created.
/// - `method`: The HTTP method (e.g., "GET", "POST").
/// - 'data': Optional parameters for the request (params)
/// 
/// # Returns:
/// - `HeaderMap`: The generated headers for the API request.
///
pub fn get_headers(endpoint: &str, method: &str, data: Option<&str>,) -> Result<HeaderMap, Box<dyn Error>> {
    dotenv().ok();
    let api_key = env::var("LN_API_KEY").expect("LN_API_KEY not set");
    let api_secret = env::var("LN_API_SECRET").expect("LN_API_SECRET not set");
    let passphrase = env::var("LN_API_PASSPHRASE").expect("LN_API_PASSPHRASE not set");

    let timestamp = Utc::now().timestamp_millis();
    
    let mut headers = HeaderMap::new();
    headers.insert("LNM-ACCESS-KEY", HeaderValue::from_str(&api_key)?);
    headers.insert("LNM-ACCESS-PASSPHRASE", HeaderValue::from_str(&passphrase)?);
    headers.insert("LNM-ACCESS-TIMESTAMP", HeaderValue::from_str(&timestamp.to_string())?);

    // Generate the signature for the request
    let signature = generate_signature(&api_secret, timestamp, method, endpoint, data);
    headers.insert("LNM-ACCESS-SIGNATURE", HeaderValue::from_str(&signature)?);
    
    // Add the User-Agent header
    headers.insert(USER_AGENT, HeaderValue::from_static(USER_AGENT_VALUE));
    
    Ok(headers)
}

/// Encodes query parameters into a URL query string.
/// 
/// # Parameters
/// - `params`: A reference to a serializable structure containing the query parameters.
/// 
/// # Returns
/// - `Option<String>`: The encoded query string or `None` if the structure is empty.
pub fn encode_query_params<T: Serialize>(params: &T) -> Option<String> {
    let query_string = serde_json::to_value(params)
        .unwrap()
        .as_object()
        .unwrap()
        .iter()
        .filter_map(|(key, value)| {
            match value {
                serde_json::Value::String(s) => Some(format!("{}={}", encode_url_component(key), encode_url_component(s))),
                serde_json::Value::Number(n) => Some(format!("{}={}", encode_url_component(key), encode_url_component(&n.to_string()))),
                _ => None,
            }
        })
        .collect::<Vec<String>>()
        .join("&");

    if query_string.is_empty() {
        None
    } else {
        Some(query_string)
    }
}

/// Encodes a string for use in a URL query component (manual URL encoding).
fn encode_url_component(input: &str) -> String {
    input
        .chars()
        .map(|c| match c {
            // Encode reserved characters
            ' ' => "%20".to_string(),
            '!' => "%21".to_string(),
            '"' => "%22".to_string(),
            '#' => "%23".to_string(),
            '$' => "%24".to_string(),
            '%' => "%25".to_string(),
            '&' => "%26".to_string(),
            '\'' => "%27".to_string(),
            '(' => "%28".to_string(),
            ')' => "%29".to_string(),
            '*' => "%2A".to_string(),
            '+' => "%2B".to_string(),
            ',' => "%2C".to_string(),
            '/' => "%2F".to_string(),
            ':' => "%3A".to_string(),
            ';' => "%3B".to_string(),
            '=' => "%3D".to_string(),
            '?' => "%3F".to_string(),
            '@' => "%40".to_string(),
            '[' => "%5B".to_string(),
            ']' => "%5D".to_string(),
            _ if c.is_ascii_alphanumeric() => c.to_string(), // Leave alphanumerics as is
            _ => format!("%{:02X}", c as u8), // Encode everything else
        })
        .collect()
}
