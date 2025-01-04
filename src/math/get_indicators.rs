// src/math/get_indicators.rs

use crate::{futures::{get_index_history::{get_index_history, IndexHistoryEntry}, get_ohlcs_history::{get_ohlcs_history, GetOhlcsParams, OhlcHistoryEntry}, get_price_history::{get_price_history, PriceHistoryEntry}}, utils::get_timestamps::format_timestamp};
use crate::math::price_indicators::{
    calculate_moving_average, calculate_exponential_moving_average,
    calculate_bollinger_bands, calculate_rsi, calculate_atr,
};
use chrono::{Utc, Duration};
use colored::Colorize;

use super::price_indicators::{calculate_bollinger_bands_ohlc, calculate_exponential_moving_average_ohlc, calculate_moving_average_ohlc, calculate_rsi_ohlc};

/// Represents the calculated indicators for a trading session.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Indicators {
    pub ohlc_data: Vec<OhlcHistoryEntry>,
    pub price_data: Vec<PriceHistoryEntry>,
    pub index_price_data: Vec<IndexHistoryEntry>, 
    pub ma: Option<f64>,
    pub ema: Option<f64>,
    pub bollinger_bands: Option<(f64, f64, f64)>,
    pub rsi: Option<f64>,
    pub i_ma: Option<f64>,
    pub i_ema: Option<f64>,
    pub i_bollinger_bands: Option<(f64, f64, f64)>,
    pub i_rsi: Option<f64>,
    pub atr: Option<f64>,
    pub ohlc_ma: Option<f64>,
    pub ohlc_ema: Option<f64>,
    pub ohlc_bollinger_bands: Option<(f64, f64, f64)>,
    pub ohlc_rsi: Option<f64>,
}

/// Fetches price, OHLC, and optional index data, then calculates trading indicators.
///
/// # Parameters:
/// - `api_url`: The API base URL.
/// - `range`: The range parameter for the OHLC data (e.g., "1D" for daily).
/// - `from`: The start timestamp for the data.
/// - `to`: The end timestamp for the data.
/// - `ma_period`: Period for the Moving Average.
/// - `ema_period`: Period for the Exponential Moving Average.
/// - `bb_period`: Period for the Bollinger Bands.
/// - `rsi_period`: Period for the Relative Strength Index.
/// - `atr_period`: Period for the Average True Range.
/// - `include_price_data`: Flag to include price data (default: true).
/// - `include_index_data`: Flag to include index price data (default: true).
/// 
/// # Returns:
/// - An `Indicators` struct containing the calculated values.
pub async fn get_indicators(
    api_url: &str,
    range: &str,
    from: Option<i64>,
    to: Option<i64>,
    ma_period: usize,
    ema_period: usize,
    bb_period: usize,
    bb_std_dev_multiplier: f64,
    rsi_period: usize,
    atr_period: usize,
    include_price_data: bool, // Flag for including price data
    include_index_data: bool, // Flag for including index data
) -> Result<Indicators, Box<dyn std::error::Error>> {
    // Prepare query parameters for OHLC history
    let now = Utc::now().timestamp();
    let default_from = (now - Duration::days(1).num_seconds()) * 1000; // ms
    let from = from.unwrap_or(default_from);
    let to = to.unwrap_or(now * 1000); // ms

    // Prepare query parameters for OHLC history
    let ohlc_params = GetOhlcsParams {
        range,
        from,
        to,
        limit: Some(1000),
        debug: true // with debug for inital indicators
    };

    println!("{}", ""); // For spacing
    println!("{}", "Init 1/3: OHLCs Data".dimmed());

    // Fetch OHLC history data for ATR calculation
    let ohlc_data = get_ohlcs_history(api_url, ohlc_params).await?;

    let ohlc_from_log = format_timestamp(ohlc_data.last().map(|e| e.time).unwrap_or(0));
    let ohlc_to_log = format_timestamp(ohlc_data.first().map(|e| e.time).unwrap_or(0));
    println!("{}", format!("Results: {}", ohlc_data.len()).dimmed());
    println!("{}", format!("first/from: {}", ohlc_to_log).dimmed());
    println!("{}", format!("last/to: {}", ohlc_from_log).dimmed());
    
    // Extract OHLC values for ATR
    let highs: Vec<f64> = ohlc_data.iter().map(|entry| entry.high).collect();
    let lows: Vec<f64> = ohlc_data.iter().map(|entry| entry.low).collect();
    let closes: Vec<f64> = ohlc_data.iter().map(|entry| entry.close).collect();
    
    let price_data = if include_price_data {
        println!("{}", "Init 2/3: Price Data".dimmed());

        // Fetch price history data for MA, EMA, RSI, and Bollinger Bands
        let price_data = get_price_history(api_url, Some(from), Some(to), None).await?;

        let from_log = format_timestamp(price_data.first().map(|e| e.time).unwrap_or(0));
        let to_log = format_timestamp(price_data.last().map(|e| e.time).unwrap_or(0));

        println!("{}", format!("Results: {}", price_data.len()).dimmed());
        println!("{}", format!("first/from: {}", to_log).dimmed());
        println!("{}", format!("last/to: {}", from_log).dimmed());

        Some(price_data)
    } else {
        None
    };

    let index_price_data = if include_index_data {
        println!("{}", "Init 3/3: Index Data".dimmed());

        // Fetch index price history data for MA, EMA, RSI, and Bollinger Bands
        let index_price_data = get_index_history(api_url, Some(from), Some(to), None).await?;

        let index_from_log = format_timestamp(index_price_data.first().map(|e| e.time).unwrap_or(0));
        let index_to_log = format_timestamp(index_price_data.last().map(|e| e.time).unwrap_or(0));

        println!("{}", format!("Fetched index price data length: {}", index_price_data.len()).dimmed());
        println!("{}", format!("first/from: {}", index_to_log).dimmed());
        println!("{}", format!("last/to: {}", index_from_log).dimmed());

        Some(index_price_data)
    } else {
        None
    };

    // Calculate indicators
    let ma = if let Some(ref price_data) = price_data {
        calculate_moving_average(&price_data.iter().map(|entry| entry.value).collect::<Vec<f64>>(), ma_period)
    } else {
        None
    };
    
    let ema = if let Some(ref price_data) = price_data {
        calculate_exponential_moving_average(&price_data.iter().map(|entry| entry.value).collect::<Vec<f64>>(), ema_period)
    } else {
        None
    };
    
    let bollinger_bands = if let Some(ref price_data) = price_data {
        calculate_bollinger_bands(&price_data.iter().map(|entry| entry.value).collect::<Vec<f64>>(), bb_period, bb_std_dev_multiplier)
    } else {
        None
    };
    
    let rsi = if let Some(ref price_data) = price_data {
        calculate_rsi(&price_data.iter().map(|entry| entry.value).collect::<Vec<f64>>(), rsi_period)
    } else {
        None
    };

    let i_ma = if let Some(ref index_data) = index_price_data {
        calculate_moving_average(&index_data.iter().map(|entry| entry.value).collect::<Vec<f64>>(), ma_period)
    } else {
        None
    };
    
    let i_ema = if let Some(ref index_data) = index_price_data {
        calculate_exponential_moving_average(&index_data.iter().map(|entry| entry.value).collect::<Vec<f64>>(), ema_period)
    } else {
        None
    };

    let i_bollinger_bands = if let Some(ref index_data) = index_price_data {
        calculate_bollinger_bands(&index_data.iter().map(|entry| entry.value).collect::<Vec<f64>>(), bb_period, bb_std_dev_multiplier)
    } else {
        None
    };

    let i_rsi = if let Some(ref index_data) = index_price_data {
        calculate_rsi(&index_data.iter().map(|entry| entry.value).collect::<Vec<f64>>(), rsi_period)
    } else {
        None
    };

    let atr = calculate_atr(&highs, &lows, &closes, atr_period);

    let ohlc_ma = calculate_moving_average_ohlc(&ohlc_data, ma_period);
    let ohlc_ema = calculate_exponential_moving_average_ohlc(&ohlc_data, ema_period);
    let ohlc_bollinger_bands = calculate_bollinger_bands_ohlc(&ohlc_data, bb_period, bb_std_dev_multiplier);
    let ohlc_rsi = calculate_rsi_ohlc(&ohlc_data, rsi_period);

    Ok(Indicators {
        ohlc_data,
        price_data: price_data.unwrap_or_default(),
        index_price_data: index_price_data.unwrap_or_default(),
        ma,
        ema,
        bollinger_bands,
        rsi,
        i_ma,
        i_ema,
        i_bollinger_bands,
        i_rsi,
        atr,
        ohlc_ma,
        ohlc_ema,
        ohlc_bollinger_bands,
        ohlc_rsi
    })
}

// Function to update indicators with OHLCs data
pub fn update_price_indicators(
    ohlc_data: &[OhlcHistoryEntry],
    ma_period: usize,
    ema_period: usize,
    bb_period: usize,
    bb_std_dev_multiplier: f64,
    rsi_period: usize,
    atr_period: usize,
    price_data: Option<&[PriceHistoryEntry]>,
    index_data: Option<&[IndexHistoryEntry]>,
) -> (
    Option<f64>, // MA (OHLC)
    Option<f64>, // EMA (OHLC)
    Option<(f64, f64, f64)>, // Bollinger Bands (OHLC)
    Option<f64>, // RSI (OHLC)
    Option<f64>, // ATR (OHLC)
    
    Option<f64>, // MA (Price)
    Option<f64>, // EMA (Price)
    Option<(f64, f64, f64)>, // Bollinger Bands (Price)
    Option<f64>, // RSI (Price)

    Option<f64>, // MA (Index)
    Option<f64>, // EMA (Index)
    Option<(f64, f64, f64)>, // Bollinger Bands (Index)
    Option<f64>, // RSI (Index)
) {
    // OHLC indicators (already in place)
    let highs: Vec<f64> = ohlc_data.iter().map(|entry| entry.high).collect();
    let lows: Vec<f64> = ohlc_data.iter().map(|entry| entry.low).collect();
    let closes: Vec<f64> = ohlc_data.iter().map(|entry| entry.close).collect();

    let ma = calculate_moving_average(&closes, ma_period);
    let ema = calculate_exponential_moving_average(&closes, ema_period);
    let bollinger_bands = calculate_bollinger_bands(&closes, bb_period, bb_std_dev_multiplier);
    let rsi = calculate_rsi(&closes, rsi_period);
    let atr = calculate_atr(&highs, &lows, &closes, atr_period);

    // Price data indicators (if available)
    let (price_ma, price_ema, price_bollinger_bands, price_rsi) = if let Some(price_data) = price_data {
        let price_closes: Vec<f64> = price_data.iter().map(|entry| entry.value).collect();
        (
            calculate_moving_average(&price_closes, ma_period),
            calculate_exponential_moving_average(&price_closes, ema_period),
            calculate_bollinger_bands(&price_closes, bb_period, bb_std_dev_multiplier),
            calculate_rsi(&price_closes, rsi_period),
        )
    } else {
        (None, None, None, None)
    };

    // Index data indicators (if available)
    let (index_ma, index_ema, index_bollinger_bands, index_rsi) = if let Some(index_data) = index_data {
        let index_closes: Vec<f64> = index_data.iter().map(|entry| entry.value).collect();
        (
            calculate_moving_average(&index_closes, ma_period),
            calculate_exponential_moving_average(&index_closes, ema_period),
            calculate_bollinger_bands(&index_closes, bb_period, bb_std_dev_multiplier),
            calculate_rsi(&index_closes, rsi_period),
        )
    } else {
        (None, None, None, None)
    };

    (
        ma, // MA (OHLC)
        ema, // EMA (OHLC)
        bollinger_bands, // Bollinger Bands (OHLC)
        rsi, // RSI (OHLC)
        atr, // ATR (OHLC)

        price_ma, // MA (Price)
        price_ema, // EMA (Price)
        price_bollinger_bands, // Bollinger Bands (Price)
        price_rsi, // RSI (Price)

        index_ma, // MA (Index)
        index_ema, // EMA (Index)
        index_bollinger_bands, // Bollinger Bands (Index)
        index_rsi, // RSI (Index)
    )
}
