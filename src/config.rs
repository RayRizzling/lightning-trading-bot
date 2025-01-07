// src/config.rs

use dotenv::dotenv;
use std::{env, time::Duration};
use colored::Colorize;

use crate::utils::{get_timestamps::{
    format_timestamp, get_current_time_ms, get_time_n_days_ago_ms, get_time_n_minutes_ago_ms
}, log_bot_params::get_interval_from_range};

// Configuration for the bot's settings and signal parameters
pub struct BotConfig {
    pub api_url: String,                 // URL for the API endpoint (loaded from environment variables)
    pub range: String,                   // Time range for data (e.g., 1 minute, 1 hour)
    pub from: Option<i64>,               // Starting timestamp for data fetching (optional)
    pub to: Option<i64>,                 // Ending timestamp for data fetching (optional)
    pub formatted_from: String,          // Formatted 'from' timestamp for API calls
    pub formatted_to: String,            // Formatted 'to' timestamp for API calls
    pub ma_period: usize,                // Period for the moving average (MA) calculation
    pub ema_period: usize,               // Period for the exponential moving average (EMA) calculation
    pub bb_period: usize,                // Period for the Bollinger Bands calculation
    pub bb_std_dev_multiplier: f64,      // Multiplier for standard deviation in Bollinger Bands
    pub rsi_period: usize,               // Period for the relative strength index (RSI) calculation
    pub atr_period: usize,               // Period for the average true range (ATR) calculation
    pub trade_type: String,              // Defines the trade type: "running", "open", or "closed"
    pub include_price_data: bool,        // Whether to include price data (might slow down the bot)
    pub include_index_data: bool,        // Whether to include index data (might slow down the bot)
    pub interval: Duration,              // The interval for data fetching (calculated based on range)
    pub risk_per_trade_percent: f64,     // Risk handling for trade quantity
    pub risk_to_reward_ratio: f64,       // Risk handling for takeprofit
    pub risk_to_loss_ratio: f64,         // Risk handling for stoploss
    pub trade_gap_seconds: u64           // Min gap bewtween opening two trades in seconds
}

// Configuration for the signal weights and gap value
pub struct SignalSettings {
    pub bollinger_weight: f64,           // Weight for the Bollinger Bands signal
    pub rsi_weight: f64,                 // Weight for the RSI signal
    pub ma_ema_weight: f64,              // Weight for the MA/EMA signal
    pub atr_weight: f64,                 // Weight for the ATR signal
    pub gap_value: f64,                  // Gap value for triggering buy/sell signals based on indicator thresholds
}

// Loads the bot's configuration settings
pub async fn load_config() -> BotConfig {
    dotenv().ok(); // Load environment variables from .env file

    let trade_gap_seconds = 5; // min gap bewtween opening two trades

    // Fetch the API URL from the environment variables
    let api_url = env::var("LN_MAINNET_API_URL").expect("LN_MAINNET_API_URL not set");

    // Default time range for the data (1 minute in this case)
    let range = "1".to_string(); // Possible values: 1, 5, 10, 15, 30, 60, 120, 180, 240, 1D, 1W, 1M, 3M. Example "1" for each minute
    let from = Some(get_time_n_minutes_ago_ms(60)); // Default to 1 hour ago for 'from' timestamp
    let to = None; // Default to current time for 'to' timestamp

    // Determine the interval based on the selected range
    let interval = get_interval_from_range(&range).await;

    // Define the period for each indicator
    let ma_period = 14;                // Period for moving average (MA)
    let ema_period = 12;               // Period for exponential moving average (EMA)
    let bb_period = 12;                // Period for Bollinger Bands
    let bb_std_dev_multiplier = 2.0;   // Standard deviation multiplier for Bollinger Bands
    let rsi_period = 9;                // Period for relative strength index (RSI)
    let atr_period = 7;                // Period for average true range (ATR)

    // Define the trade type (can be "running", "open", or "closed")
    let trade_type = "running".to_string();

    // Optional data inclusion for price and index data
    let include_price_data = false;    // Set to true if price data should be included (may increase initialization time)
    let include_index_data = false;    // Set to true if index data should be included (may increase initialization time)

    // Format the 'from' and 'to' timestamps
    let formatted_from = format_timestamp(from.unwrap_or_else(|| get_time_n_days_ago_ms(14)));
    let formatted_to = format_timestamp(to.unwrap_or_else(get_current_time_ms));

    // Trade risk
    let risk_per_trade_percent = 0.01; // 1%
    let risk_to_reward_ratio = 0.8;
    let risk_to_loss_ratio = 0.75;

    // Return the full BotConfig struct with all settings
    BotConfig {
        api_url,
        range,
        from,
        to,
        formatted_from,
        formatted_to,
        ma_period,
        ema_period,
        bb_period,
        bb_std_dev_multiplier,
        rsi_period,
        atr_period,
        trade_type,
        include_price_data,
        include_index_data,
        interval,
        risk_per_trade_percent,
        risk_to_reward_ratio,
        risk_to_loss_ratio,
        trade_gap_seconds
    }
}

// Loads the signal settings, including weights and gap value
pub async fn load_signal_settings() -> SignalSettings {
    // Set the weights for the indicators
    let bollinger_weight = 0.25;  // Weight for the Bollinger Bands signal
    let rsi_weight = 0.30;        // Weight for the RSI signal
    let ma_ema_weight = 0.20;     // Weight for the MA/EMA signal
    let atr_weight = 0.25;        // Weight for the ATR signal
    let gap_value = 15.0;         // Gap value for triggering strong buy/sell signals

    // Check that the sum of weights equals 1.0 with a tolerance of 0.001
    let weight_sum: f64 = bollinger_weight + rsi_weight + ma_ema_weight + atr_weight;
    if (weight_sum - 1.0).abs() > 0.001 { // Allow a small margin for floating point precision errors
        println!("{}", format!("WARNING: The sum of weights does not equal 1.0! Sum: {}", weight_sum).yellow());
    }

    // Return the SignalSettings struct with the weights and gap value
    SignalSettings {
        bollinger_weight,
        rsi_weight,
        ma_ema_weight,
        atr_weight,
        gap_value,
    }
}
