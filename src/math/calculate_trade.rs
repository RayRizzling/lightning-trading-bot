// src/utils/calculate_trade.rs

use crate::futures::get_market::FuturesMarket;

pub struct TradeParams {
    pub margin_sats: f64,
    pub liquidation_price: f64,
    pub trade_quantity: f64,
    pub maintenance_margin: f64,
}

pub fn calculate_trade_params(
    trade_type: &str,
    entry_price: f64,
    leverage: u64,
    trade_quantity: f64,
    market_data: &FuturesMarket,
) -> Result<TradeParams, String> {
    // (Trade) Margin in BTC
    let margin_raw = trade_quantity / (entry_price * leverage as f64);
    let margin = (margin_raw * 100_000_000.0).floor() / 100_000_000.0;

    // (Trade) Margin in Satoshis
    let margin_sats = margin * 100_000_000.0;

    let trading_fee_rate = market_data
        .fees
        .trading
        .tiers
        .iter()
        .rev()
        .find(|tier| margin_sats as u64 >= tier.min_volume)
        .map(|tier| tier.fees)
        .ok_or("No matching fee tier found")?;

    // Liquidation Price
    let liquidation_price = match trade_type {
        "b" => {
            // for Buy (Long): Liquidation Price = 1 / (1 / Entry Price + Trade Margin / Quantity)
            let inverse_liquidation = (1.0 / entry_price) + (margin / trade_quantity);
            1.0 / inverse_liquidation
        }
        "s" => {
            // for Sell (Short): Liquidation Price = 1 / (1 / Entry Price - Trade Margin / Quantity)
            let inverse_liquidation = (1.0 / entry_price) - (margin / trade_quantity);
            1.0 / inverse_liquidation
        }
        _ => return Err("Invalid trade type, expected 'b' for Buy or 's' for Sell".to_string()),
    };

    let opening_fee_reserved = (trade_quantity / entry_price) * trading_fee_rate;
    let closing_fee_reserved = (trade_quantity / liquidation_price) * trading_fee_rate;
    let maintenance_margin_raw = opening_fee_reserved + closing_fee_reserved;
    let maintenance_margin = (maintenance_margin_raw * 100_000_000.0).floor(); // sats value

    Ok(TradeParams {
        margin_sats,
        liquidation_price,
        trade_quantity,
        maintenance_margin,
    })
}
