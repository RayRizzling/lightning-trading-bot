// src/math/get_trade_quantity.rs

use crate::futures::get_market::FuturesMarket;

/// Calculates the maximum trade quantity based on user balance, risk per trade, leverage,
/// volatility (via ATR), and market limits. The function considers the user's balance in satoshis
/// and converts it to USD, then calculates the trade quantity considering the leverage and volatility.
/// The function ensures that the resulting quantity is within the market's allowed limits for quantity.
///
/// # Parameters
/// - `balance_sats`: User's balance in satoshis (1 BTC = 100,000,000 satoshis).
/// - `entry_price`: The entry price for 1 Bitcoin in USD.
/// - `risk_per_trade_percent`: The risk percentage per trade (e.g., 0.01 for 1%).
/// - `max_trades`: Maximum number of simultaneous trades allowed.
/// - `leverage`: Leverage used in the trade (e.g., 10x).
/// - `atr`: Average True Range (ATR) value to adjust for volatility (optional).
/// - `market_data`: The market data containing minimum and maximum trade quantity limits.
///
/// # Returns
/// The calculated trade quantity considering all the factors, limited by the market's min and max quantity.
/// # Errors
/// Returns an error if ATR is not available (ATR is required for the trade).
pub fn calculate_trade_quantity(
    balance_sats: u64,      // User's balance in satoshis
    entry_price: f64,       // Entry price for 1 Bitcoin in USD
    risk_per_trade_percent: f64, // Risk per trade as a percentage (e.g., 0.01 for 1%)
    max_trades: u64,        // Maximum number of simultaneous trades
    leverage: f64,          // Leverage factor (e.g., 10x)
    atr: Option<f64>,       // Average True Range (ATR) value for volatility (optional)
    market_data: &FuturesMarket, // Market data for minimum and maximum quantity limits
) -> Result<f64, String> { // Return Result with Ok for success and Err for failure
    // Check if ATR is available, if not return an error
    let volatility_factor = match atr {
        Some(atr_value) if atr_value > 0.0 => 1.0 / atr_value,  // If ATR is greater than 0, use it
        _ => return Err("ATR is required for the trade.".to_string()),  // Return error if ATR is missing
    };

    // Convert the balance from satoshis to USD
    let balance_usd = (balance_sats as f64) * entry_price / 100_000_000.0;

    // Calculate the maximum quantity per trade based on the user's balance and risk
    let max_quantity_per_trade = (balance_usd * risk_per_trade_percent) / max_trades as f64;

    // Adjust the quantity by considering the leverage
    let leverage_adjusted_quantity = max_quantity_per_trade * leverage;

    // Adjust the quantity considering both leverage and volatility
    let adjusted_quantity = leverage_adjusted_quantity * volatility_factor;

    // Ensure the final quantity is within the market's limits
    let final_quantity = adjusted_quantity
        .min(market_data.limits.quantity.max as f64)  // Maximum allowed quantity
        .max(market_data.limits.quantity.min as f64); // Minimum allowed quantity

    Ok(final_quantity)
}
