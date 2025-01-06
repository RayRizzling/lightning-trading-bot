// src/futures/create_trade.rs

use crate::futures::create_trade::{create_market_buy_order, create_market_sell_order};
use crate::futures::ticker::get_futures_ticker;
use crate::futures::get_trades::{get_trades, GetTradesParams};
use crate::math::get_stoploss_takeprofit::calculate_stoploss_takeprofit;
use crate::math::get_trade_quantity::calculate_trade_quantity;
use crate::utils::calculate_trade::calculate_trade_params;
use crate::utils::get_user::get_user;
use crate::utils::log_bot_params::log_forecast_trade;
use std::sync::Arc;
use tokio::sync::Mutex;

use super::get_indicators::Indicators;
use super::get_signals::Signal;
use super::init_bot_params::BotParams;

pub enum CreateTradeResult {
    TradeCreated,
    NoTradeCreated(String)
}

// Function to create a trade based on the signal
pub async fn create_trade_from_signal(
    signal: Signal,
    api_url: &str,
    bot_params: Arc<Mutex<BotParams>>,
    indicators: Option<Indicators>,
    leverage: Option<u64>,
    risk_per_trade_percent: f64,
    risk_to_reward_ratio: f64,
    risk_to_loss_ratio: f64
) -> Result<CreateTradeResult, String> {

    let leverage = leverage.unwrap_or(20);
    let bot_params = bot_params.lock().await;
    let max_trades = bot_params.market_data.as_ref().unwrap().limits.count.max;
    //let max_trades = 25;

    // Fetch running trades and check if trade count limit is reached
    let trade_params = GetTradesParams {
        r#type: "running",
        from: None,
        to: None,
        limit: None,
    };
    let active_trades = match get_trades(api_url, Some(trade_params)).await {
        Ok(trades) => trades,
        Err(e) => {
            return Ok(CreateTradeResult::NoTradeCreated(format!(
                "Error fetching active trades: {}",
                e
            )))
        }
    };
    if active_trades.len() >= max_trades as usize {
        return Ok(CreateTradeResult::NoTradeCreated(
            "Trade limit reached".to_string(),
        ));
    }

    // Fetch user and futures ticker data
    let user_data = match get_user(api_url).await {
        Ok(user) => user,
        Err(e) => return Err(format!("Error fetching user data: {}", e)),
    };
    let ticker = match get_futures_ticker(api_url).await {
        Ok(ticker) => ticker,
        Err(e) => return Err(format!("Error fetching futures ticker: {}", e)),
    };

    // set entry_price and exit_price
    let (entry_p, trade_type) = match signal {
        Signal::Buy | Signal::StrongBuy => (ticker.ask_price, "b"),
        Signal::Sell | Signal::StrongSell => (ticker.bid_price, "s"),
        Signal::Hold => {
            return Ok(CreateTradeResult::NoTradeCreated(
                "Hold signal received on create_trade".to_string(),
            ));
        }
        _ => {
            return Ok(CreateTradeResult::NoTradeCreated(
                "No valid trading signal".to_string(),
            ));
        }
    };
    
    // futures market data (eg for fees)
    let futures_market = bot_params
        .market_data
        .as_ref()
        .ok_or("Market data is not available")?;

    // calculate quantity for trade
    let quantity = match calculate_trade_quantity(
        user_data.balance as u64,
        entry_p,
        risk_per_trade_percent,
        max_trades,
        leverage as f64,
        indicators.as_ref().and_then(|i| i.atr),
        futures_market,
    ) {
        Ok(final_quantity) => Some(final_quantity as u64),
        Err(e) => return Err(format!("Error calculating trade quantity: {}", e)),
    };
    if quantity.is_none() {
        return Err("Quantity calculation failed".to_string());
    }   

    // calculate takeprofit and stoploss for trade
    let (takeprofit, stoploss) = match calculate_stoploss_takeprofit(
        entry_p,
        indicators.as_ref().and_then(|i| i.atr).unwrap(),
        leverage as f64,
        trade_type == "b",
        risk_to_reward_ratio,
        risk_to_loss_ratio
    ) {
        Ok((takeprofit, stoploss)) => (Some(takeprofit as u64), Some(stoploss as u64)),
        Err(e) => return Err(format!("Error calculating stoploss/takeprofit: {}", e)),
    };

    let trade_params = calculate_trade_params(trade_type, entry_p, leverage, quantity.map(|q| q as f64).unwrap_or(1.0), futures_market)
        .map_err(|e| format!("Error calculating trade parameters: {}", e))?;

    log_forecast_trade(
        entry_p,
        takeprofit,
        stoploss,
        &trade_params
    );

    // Execute trade based on the signal
    match signal {
        Signal::Buy | Signal::StrongBuy => {
            if user_data.balance <= trade_params.margin_sats {
                return Ok(CreateTradeResult::NoTradeCreated(
                    "Insufficient balance for creating a trade".to_string(),
                ));
            }
            create_market_buy_order(
                api_url,
                leverage,
                quantity,
                None,
                takeprofit,
                stoploss,
            )
            .await
            .map_err(|e| format!("Error creating buy order: {}", e))?;

            return Ok(CreateTradeResult::TradeCreated);
        }
        Signal::Sell | Signal::StrongSell => {
            if user_data.balance <= trade_params.margin_sats {
                return Ok(CreateTradeResult::NoTradeCreated(
                    "Insufficient balance for creating a trade".to_string(),
                ));
            }
            create_market_sell_order(
                api_url,
                leverage,
                quantity,
                None,
                takeprofit,
                stoploss,
            )
            .await
            .map_err(|e| format!("Error creating sell order: {}", e))?;

            return Ok(CreateTradeResult::TradeCreated);
        }
        Signal::Hold => {
            return Ok(CreateTradeResult::NoTradeCreated(
                "Hold signal received on create_trade".to_string(),
            ));
        }
        Signal::Undefined => {
            return Ok(CreateTradeResult::NoTradeCreated(format!(
                "{:?} signal received on create_trade",
                signal
            )));
        }
    }
}
