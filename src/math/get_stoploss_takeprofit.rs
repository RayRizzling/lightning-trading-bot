// src/math/get_stoploss_takeprofit.rs

/// Calculates the stoploss and takeprofit levels for short-term trades
/// based on entry price, ATR, leverage, and risk per trade. 
/// The risk amount is calculated based on the user's balance and risk percentage.
/// 
/// # Parameters
/// - `entry_price`: The entry price of the trade.
/// - `atr_value`: The ATR value (Average True Range) for volatility.
/// - `leverage`: The leverage used for the trade.
/// - `is_buy`: Whether the trade is a buy (true) or sell (false) trade.
/// - `risk_to_reward_ratio`: Desired risk-to-reward ratio (e.g., 1:2).
/// 
/// # Returns
/// - `stoploss`: The calculated stoploss price.
/// - `takeprofit`: The calculated takeprofit price.
/// 
/// # Errors
/// - Returns an error if ATR value is invalid (<= 0.0).
pub fn calculate_stoploss_takeprofit(
    entry_price: f64,
    atr_value: f64,
    leverage: f64,
    is_buy: bool,
    risk_to_reward_ratio: f64,
    risk_to_loss_ratio: f64
) -> Result<(f64, f64), String> {
    if atr_value <= 0.0 {
        return Err("ATR value must be greater than 0.".to_string());
    }

    // Distance based on ATR value and leverage
    let distance = atr_value * leverage;

    // Calculate the Stop-Loss distance using the Risk-to-Loss Ratio
    let stoploss_distance = distance * risk_to_loss_ratio;

    // Calculate the Take-Profit distance using the Risk-to-Reward Ratio
    let takeprofit_distance = distance * risk_to_reward_ratio;


    // Calculate the Stop-Loss and Take-Profit prices
    let stoploss = if is_buy {
        entry_price - stoploss_distance // For buy, Stop-Loss is below the entry price
    } else {
        entry_price + stoploss_distance // For sell, Stop-Loss is above the entry price
    };

    let takeprofit = if is_buy {
        entry_price + takeprofit_distance // For buy, Take-Profit is above the entry price
    } else {
        entry_price - takeprofit_distance // For sell, Take-Profit is below the entry price
    };

    Ok((takeprofit, stoploss))
}