// src/math/price_indicators.rs

use crate::futures::get_ohlcs_history::OhlcHistoryEntry;

/// Calculates the moving average (MA) for a given period from price data.
/// 
/// # Parameters:
/// - `prices`: A vector of f64 values representing the price history.
/// - `period`: The number of periods for the moving average.
/// 
/// # Returns:
/// - An `Option<f64>` containing the moving average, or `None` if insufficient data.
pub fn calculate_moving_average(prices: &[f64], period: usize) -> Option<f64> {
    if prices.len() < period {
        return None;
    }
    Some(prices.iter().skip(prices.len() - period).sum::<f64>() / period as f64)
}


/// Calculates the exponential moving average (EMA) for a given period from price data.
/// 
/// # Parameters:
/// - `prices`: A vector of f64 values representing the price history.
/// - `period`: The number of periods for the EMA.
/// 
/// # Returns:
/// - An `Option<f64>` containing the EMA, or `None` if insufficient data.
pub fn calculate_exponential_moving_average(prices: &[f64], period: usize) -> Option<f64> {
    if prices.len() < period {
        return None;
    }

    let smoothing = 2.0 / (period as f64 + 1.0);
    let mut ema = prices[0..period].iter().copied().sum::<f64>() / period as f64;

    for &price in prices.iter().skip(period) {
        ema = (price - ema) * smoothing + ema;
    }

    Some(ema)
}


/// Calculates Bollinger Bands for a given period.
/// 
/// # Parameters:
/// - `prices`: A vector of f64 values representing the price history.
/// - `period`: The number of periods for the Bollinger Bands.
/// - `std_dev_multiplier`: The multiplier for the standard deviation (e.g., 2.0).
/// 
/// # Returns:
/// - A tuple of `Option<(f64, f64, f64)>` containing (Lower Band, Middle Band, Upper Band).
pub fn calculate_bollinger_bands(
    prices: &[f64],
    period: usize,
    std_dev_multiplier: f64,
) -> Option<(f64, f64, f64)> {
    if prices.len() < period {
        return None;
    }

    let window = &prices[prices.len() - period..];

    let middle_band = window.iter().sum::<f64>() / period as f64;

    let variance = window.iter().map(|price| (price - middle_band).powi(2)).sum::<f64>() / period as f64;

    let std_dev = variance.sqrt();

    let upper_band = middle_band + std_dev_multiplier * std_dev;
    let lower_band = middle_band - std_dev_multiplier * std_dev;

    // println!("LowerBAND: {} __ MIDDLE: {} __ UP: {}", lower_band, middle_band, upper_band);
    Some((lower_band, middle_band, upper_band))
}




/// Calculates the Relative Strength Index (RSI) for a given period.
/// 
/// # Parameters:
/// - `prices`: A vector of f64 values representing the price history.
/// - `period`: The number of periods for the RSI.
/// 
/// # Returns:
/// - An `Option<f64>` containing the RSI value, or `None` if insufficient data.
/// Calculates the Relative Strength Index (RSI) for a given period.
pub fn calculate_rsi(prices: &[f64], period: usize) -> Option<f64> {
    if prices.len() < period {
        return None;
    }

    let mut gains = vec![];
    let mut losses = vec![];

    for i in 1..prices.len() {
        let diff = prices[i] - prices[i - 1];
        if diff > 0.0 {
            gains.push(diff);
            losses.push(0.0);
        } else {
            gains.push(0.0);
            losses.push(-diff);
        }
    }

    let avg_gain = gains.iter().take(period).sum::<f64>() / period as f64;
    let avg_loss = losses.iter().take(period).sum::<f64>() / period as f64;

    let mut avg_gain = avg_gain;
    let mut avg_loss = avg_loss;

    for i in period..gains.len() {
        avg_gain = (avg_gain * (period as f64 - 1.0) + gains[i]) / period as f64;
        avg_loss = (avg_loss * (period as f64 - 1.0) + losses[i]) / period as f64;
    }

    if avg_loss == 0.0 {
        return Some(100.0);
    }

    let rs = avg_gain / avg_loss;
    let rsi = 100.0 - (100.0 / (1.0 + rs));

    Some(rsi)
}


/// Calculates the Average True Range (ATR) for a given period.
/// 
/// # Parameters:
/// - `highs`: A vector of f64 representing the high prices.
/// - `lows`: A vector of f64 representing the low prices.
/// - `closes`: A vector of f64 representing the closing prices.
/// - `period`: The number of periods for the ATR.
/// 
/// # Returns:
/// - An `Option<f64>` containing the ATR value, or `None` if insufficient data.
pub fn calculate_atr(
    highs: &[f64],
    lows: &[f64],
    closes: &[f64],
    period: usize,
) -> Option<f64> {
    if highs.len() < period || lows.len() < period || closes.len() < period {
        return None;
    }

    let mut true_ranges = Vec::new();

    for i in 1..highs.len() {
        let high_low = highs[i] - lows[i];
        let high_close = (highs[i] - closes[i - 1]).abs();
        let low_close = (lows[i] - closes[i - 1]).abs();

        true_ranges.push(high_low.max(high_close).max(low_close));
    }

    if true_ranges.len() < period {
        return None;
    }

    let atr = true_ranges.iter().take(period).sum::<f64>() / period as f64;
    Some(atr)
}


pub fn calculate_moving_average_ohlc(ohlcs: &[OhlcHistoryEntry], period: usize) -> Option<f64> {
    let closes: Vec<f64> = ohlcs.iter().map(|entry| entry.close).collect();
    calculate_moving_average(&closes, period)
}

pub fn calculate_exponential_moving_average_ohlc(
    ohlcs: &[OhlcHistoryEntry],
    period: usize,
) -> Option<f64> {
    let closes: Vec<f64> = ohlcs.iter().map(|entry| entry.close).collect();
    calculate_exponential_moving_average(&closes, period)
}

pub fn calculate_bollinger_bands_ohlc(
    ohlcs: &[OhlcHistoryEntry],
    period: usize,
    std_dev_multiplier: f64,
) -> Option<(f64, f64, f64)> {
    if ohlcs.len() < period {
        println!("Insufficient OHLC data: got {}, need {}", ohlcs.len(), period);
        return None;
    }

    let closes: Vec<f64> = ohlcs.iter().map(|entry| entry.close).collect();

    if closes.iter().any(|&price| !price.is_finite() || price < 0.0) {
        println!("Invalid close prices detected: {:?}", closes);
        return None;
    }

    calculate_bollinger_bands(&closes, period, std_dev_multiplier)
}

pub fn calculate_rsi_ohlc(ohlcs: &[OhlcHistoryEntry], period: usize) -> Option<f64> {
    let closes: Vec<f64> = ohlcs.iter().map(|entry| entry.close).collect();
    calculate_rsi(&closes, period)
}
