// src/utils/log_bot_params.rs

use colored::Colorize;
use crate::{math::init_bot_params::BotParams, utils::get_timestamps::format_timestamp};
use tokio::time::Duration;
use std::io::{self, Write};

use super::connect_ws::PriceData;

pub fn log_bot_params(bot_params: &BotParams, trade_type: &str, formatted_from: String, formatted_to: String) {
    println!("{}", format!("From: {} - To: {}", formatted_from, formatted_to).dimmed());

    // User data
    if let Some(user_data) = &bot_params.user_data {
        println!("{}", "User Data:".green());
        println!("{:?}", user_data);
    } else {
        println!("{}", "No user available.".yellow());
    }

    // Ticker data
    if let Some(ticker_data) = &bot_params.ticker_data {
        println!("{}", "Futures Ticker Data:".green());
        println!("{:?}", ticker_data);
    } else {
        println!("{}", "No Futures Ticker available.".yellow());
    }

    // Market data
    if let Some(market_data) = &bot_params.market_data {
        println!("{}", "Futures Market Data:".green());
        println!("{:?}", market_data);
    } else {
        println!("{}", "No Futures Market Data available.".yellow());
    }

    // Indicator data
    if let Some(indicators) = &bot_params.indicators {
        // Anzeige der MA, EMA, Bollinger Bänder, RSI, etc.
        if let Some(ma) = indicators.ma {
            println!("{}", format!("Moving Average (MA): {}", ma).green());
        }
        
        if let Some(ema) = indicators.ema {
            println!("{}", format!("Exponential Moving Average (EMA): {}", ema).green());
        }
        
        if let Some((lower, middle, upper)) = indicators.bollinger_bands {
            println!("{}", format!("Bollinger Bands - Upper: {}, Middle: {}, Lower: {}", upper, middle, lower).green());
        }

        if let Some(rsi) = indicators.rsi {
            println!("{}", format!("Relative Strength Index (RSI): {}", rsi).green());
        }

        if let Some(i_ma) = indicators.i_ma {
            println!("{}", format!("Indicator MA: {}", i_ma).green());
        }

        if let Some(i_ema) = indicators.i_ema {
            println!("{}", format!("Indicator EMA: {}", i_ema).green());
        }

        if let Some((i_lower, i_middle, i_upper)) = indicators.i_bollinger_bands {
            println!("{}", format!("Indicator Bollinger Bands - Upper: {}, Middle: {}, Lower: {}", i_upper, i_middle, i_lower).green());
        }

        if let Some(i_rsi) = indicators.i_rsi {
            println!("{}", format!("Indicator RSI: {}", i_rsi).green());
        }

        if let Some(atr) = indicators.atr {
            println!("{}", format!("Average True Range (ATR): {}", atr).green());
        }

        if let Some(ohlc_ma) = indicators.ohlc_ma {
            println!("{}", format!("OHLC MA: {}", ohlc_ma).green());
        }

        if let Some(ohlc_ema) = indicators.ohlc_ema {
            println!("{}", format!("OHLC EMA: {}", ohlc_ema).green());
        }

        if let Some((ohlc_lower, ohlc_middle, ohlc_upper)) = indicators.ohlc_bollinger_bands {
            println!("{}", format!("OHLC Bollinger Bands - Upper: {}, Middle: {}, Lower: {}", ohlc_upper, ohlc_middle, ohlc_lower).green());
        }

        if let Some(ohlc_rsi) = indicators.ohlc_rsi {
            println!("{}", format!("OHLC RSI: {}", ohlc_rsi).green());
        }
    } else {
        println!("{}", "No Indicators available.".yellow());
    }

    // Trades data
    if let Some(trades) = &bot_params.trades {
        if trades.is_empty() {
            println!("No {} Futures Trades available.", trade_type);
        } else {
            println!("{}: {:?}", trade_type, trades);
        }
    } else {
        eprintln!("{}", "Error: Futures Trades not found.".red());
    }
}

pub async fn log_spot_price(price_data: &PriceData) {
    let price = format!("{:.2}", price_data.last_price);
    let timestamp = format_timestamp(price_data.time).bright_white();

    let (price_colored, arrow) = match price_data.last_tick_direction.as_str() {
        "PlusTick" => (price.green(), "↑".bold().green()),
        "ZeroPlusTick" => (price.bright_green(), "↑".bright_green()),
        "MinusTick" => (price.red(), "↓".bold().red()),
        "ZeroMinusTick" => (price.bright_red(), "↓".bright_red()),
        _ => (price.white(), "↔".white()),
    };

    print!(
        "\r{} --- Last Price: {} {} {} at {}",
        "Live Spot".bold().underline(),
        "Price:".bold(),
        price_colored,
        arrow,
        timestamp
    );
    io::stdout().flush().unwrap();
}

pub fn log_updated_indicators(bot_params: &BotParams) {
    if let Some(indicators) = &bot_params.indicators {

        println!("{}", "");

        // let ohlc_data = &indicators.ohlc_data;
        // if let Some(first_entry) = ohlc_data.first() {
        //     let first_time = first_entry.time;
        //     println!("{}", format!("First OHLC Time: {}", format_timestamp(first_time)).yellow());
        // }
        // if let Some(last_entry) = ohlc_data.last() {
        //     let last_time = last_entry.time;
        //     println!("{}", format!("Last OHLC Time: {}", format_timestamp(last_time)).yellow());
        // }
        
        if let Some(ma) = indicators.ma {
            println!("{}", format!("Moving Average (MA): {}", ma).green());
        }

        if let Some(ema) = indicators.ema {
            println!("{}", format!("Exponential Moving Average (EMA): {}", ema).green());
        }

        if let Some((upper, middle, lower)) = indicators.bollinger_bands {
            println!("{}", format!("Bollinger Bands - Upper: {}, Middle: {}, Lower: {}", upper, middle, lower).green());
        }

        if let Some(rsi) = indicators.rsi {
            println!("{}", format!("Relative Strength Index (RSI): {}", rsi).green());
        }

        if let Some(i_ma) = indicators.i_ma {
            println!("{}", format!("Indicator MA: {}", i_ma).green());
        }

        if let Some(i_ema) = indicators.i_ema {
            println!("{}", format!("Indicator EMA: {}", i_ema).green());
        }

        if let Some((i_upper, i_middle, i_lower)) = indicators.i_bollinger_bands {
            println!("{}", format!("Indicator Bollinger Bands - Upper: {}, Middle: {}, Lower: {}", i_upper, i_middle, i_lower).green());
        }

        if let Some(i_rsi) = indicators.i_rsi {
            println!("{}", format!("Indicator RSI: {}", i_rsi).green());
        }

        if let Some(atr) = indicators.atr {
            println!("{}", format!("Average True Range (ATR): {}", atr).green());
        }

        if let Some(ohlc_ma) = indicators.ohlc_ma {
            println!("{}", format!("Indicator MA: {}", ohlc_ma).green());
        }

        if let Some(ohlc_ema) = indicators.ohlc_ema {
            println!("{}", format!("Indicator EMA: {}", ohlc_ema).green());
        }

        if let Some((ohlc_upper, ohlc_middle, ohlc_lower)) = indicators.ohlc_bollinger_bands {
            println!("{}", format!("Indicator Bollinger Bands - Upper: {}, Middle: {}, Lower: {}", ohlc_upper, ohlc_middle, ohlc_lower).green());
        }

        if let Some(ohlc_rsi) = indicators.ohlc_rsi {
            println!("{}", format!("Indicator RSI: {}", ohlc_rsi).green());
        }
    } else {
        println!("{}", "No Indicators available.".yellow());
    }
}

pub async fn get_interval_from_range(range: &str) -> Duration {
    match range {
        "1" => Duration::from_secs(60), // 1 min
        "3" => Duration::from_secs(3 * 60), // 3 mins
        "5" => Duration::from_secs(5 * 60), // 5 mins
        "10" => Duration::from_secs(10 * 60), // 10 mins
        "15" => Duration::from_secs(15 * 60), // 15 mins
        "30" => Duration::from_secs(30 * 60), // 30 mins
        "45" => Duration::from_secs(45 * 60), // 45 mins
        "60" => Duration::from_secs(60 * 60), // 1 hour
        "120" => Duration::from_secs(2 * 60 * 60), // 2 hours
        "180" => Duration::from_secs(3 * 60 * 60), // 3 hours
        "240" => Duration::from_secs(4 * 60 * 60), // 4 hours
        "1D" => Duration::from_secs(24 * 60 * 60), // 1 day
        "1W" => Duration::from_secs(7 * 24 * 60 * 60), // 1 week
        "1M" => Duration::from_secs(30 * 24 * 60 * 60), // 1 month
        "3M" => Duration::from_secs(90 * 24 * 60 * 60), // 3 month
        _ => Duration::from_secs(60), // fallback: 1 min
    }
}