// src/math/init_bot_params.rs

use crate::utils::get_user::{get_user, User};
use crate::futures::ticker::{get_futures_ticker, FuturesTicker};
use crate::futures::get_market::{get_market, FuturesMarket};
use crate::math::get_indicators::get_indicators;
use crate::futures::get_trades::{get_trades, GetTradesParams, TradeEntry};

use super::get_indicators::Indicators;

/// Struct to hold all initialized parameters.
#[allow(dead_code)]
pub struct BotParams {
    pub user_data: Option<User>,
    pub ticker_data: Option<FuturesTicker>,
    pub market_data: Option<FuturesMarket>,
    pub indicators: Option<Indicators>,
    pub trades: Option<Vec<TradeEntry>>,
}

/// Initialize bot parameters by fetching user data, market data, ticker data,
/// indicators, and trade data.
///
/// # Parameters:
/// - `api_url`: The API base URL.
/// - `range`: The range parameter for the OHLC data (e.g., "1D" for daily).
/// - `ma_period`, `ema_period`, `bb_period`, `rsi_period`, `atr_period`: Indicator parameters.
/// - `bb_std_dev_multiplier`: Multiplier for Bollinger Bands.
/// - `trade_type`: Type of trades to fetch (e.g., "running", "open", "closed").
///
/// # Returns:
/// - A `BotParams` struct containing the initialized values.
pub async fn init_bot_params(
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
    trade_type: &str,
    include_price_data: bool, // Flag for including price data
    include_index_data: bool, // Flag for including index data
) -> Result<BotParams, Box<dyn std::error::Error>> {
    // Initialize user data
    let user_data = match get_user(api_url).await {
        Ok(user) => Some(user),
        Err(e) => {
            eprintln!("Error fetching user data: {}", e);
            None
        }
    };

    // Initialize ticker data
    let ticker_data = match get_futures_ticker(api_url).await {
        Ok(ticker) => Some(ticker),
        Err(e) => {
            eprintln!("Error fetching futures ticker: {}", e);
            None
        }
    };

    // Initialize market data
    let market_data = match get_market(api_url).await {
        Ok(market) => Some(market),
        Err(e) => {
            eprintln!("Error fetching market data: {}", e);
            None
        }
    };

    // Initialize indicators
    let indicators = match get_indicators(
        api_url,
        range,
        from,
        to,
        ma_period,
        ema_period,
        bb_period,
        bb_std_dev_multiplier,
        rsi_period,
        atr_period,
        include_price_data,
        include_index_data
    ).await {
        Ok(indicators) => Some(indicators),
        Err(e) => {
            eprintln!("Error fetching indicators: {}", e);
            None
        }
    };

    // Initialize trades
    let trade_params = GetTradesParams {
        r#type: trade_type,
        from: None,
        to: None,
        limit: None,
    };

    let trades = match get_trades(api_url, Some(trade_params)).await {
        Ok(trades) => Some(trades),
        Err(e) => {
            eprintln!("Error fetching trades: {}", e);
            None
        }
    };

    Ok(BotParams {
        user_data,
        ticker_data,
        market_data,
        indicators,
        trades,
    })
}
