use reqwest::header::HeaderMap;
use serde::Deserialize;
use std::error::Error;
use crate::utils::get_headers::get_headers;

/// Struct representing the user data received from the API.
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct User {
    pub uid: String,                   // Unique identifier for the user.
    pub role: String,                  // User's role (e.g., admin, user).
    pub balance: f64,                  // User's balance in the account.
    pub username: String,              // Username of the user.
    pub synthetic_usd_balance: f64,    // Synthetic USD balance of the user.
    pub email: Option<String>,         // Optional email address of the user.
    pub linkingpublickey: Option<String>,  // Optional linking public key.
    pub show_leaderboard: bool,        // Flag to show leaderboard.
    pub email_confirmed: bool,         // Flag indicating if email is confirmed.
    pub use_taproot_addresses: bool,   // Flag indicating if Taproot addresses are used.
    pub account_type: String,          // Account type (e.g., lnurl).
    pub auto_withdraw_enabled: bool,   // Flag indicating if auto withdrawal is enabled.
    pub auto_withdraw_lightning_address: Option<String>, // Optional Lightning address for auto withdrawal.
    pub nostr_pubkey: Option<String>,  // Optional Nostr public key.
    pub fee_tier: u32,                 // Fee tier.
    pub totp_enabled: bool,            // Flag indicating if TOTP is enabled.
    pub webauthn_enabled: bool,        // Flag indicating if WebAuthn is enabled.
}

/// Fetches the user data from the API.
/// 
/// This function sends a GET request to the `/v2/user` endpoint of the API with the provided
/// API key, secret, and passphrase. It also includes a generated signature to authenticate
/// the request and retrieves the user information.
///
/// # Parameters:
/// - `api_url`: The base URL of the API.
///
/// # Returns:
/// - `Ok(User)`: The user data retrieved from the API.
/// - `Err(Box<dyn Error>)`: Any error that occurs during the request or data processing.
pub async fn get_user(
    api_url: &str,
) -> Result<User, Box<dyn Error>> {
    let headers: HeaderMap = get_headers("/v2/user", "GET", None)?;

    let client = reqwest::Client::new();
    
    let response = client
        .get(format!("{}/user", api_url)) 
        .headers(headers)                     
        .send()           
        .await?;

    if response.status().is_success() {
        let user_data = response.json::<User>().await?;
        Ok(user_data)
    } else {
        let error: Box<dyn Error> = Box::new(response.error_for_status().unwrap_err());
        Err(error)
    }
}
