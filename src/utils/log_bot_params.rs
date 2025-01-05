// src/utils/log_bot_params.rs

use colored::Colorize;
use crate::{math::init_bot_params::BotParams, utils::get_timestamps::format_timestamp};
use tokio::time::Duration;
use std::io::{self, Write};

use super::connect_ws::PriceData;

pub fn log_bot_params(bot_params: &BotParams, trade_type: &str, formatted_from: String, formatted_to: String) {
    println!("{}", format!("From: {} - To: {}", formatted_from, formatted_to).dimmed());

    // -------------------------- User Data --------------------------
    if let Some(user_data) = &bot_params.user_data {
        println!("{}", "\n--- User Data ---".green());
        println!("{}", format!("UID: {}", user_data.uid).blue());
        println!("{}", format!("Username: {}", user_data.username).blue());
        println!("{}", format!("Role: {}", user_data.role).blue());
        if let Some(ticker_data) = &bot_params.ticker_data {
            let balance_in_usd = user_data.balance / 100000000.0 * ticker_data.last_price;
            let balance_in_usd_formatted = format!("{:.2}", balance_in_usd);
            println!(
                "{}",
                format!("Balance: {} sats (~ {} USD)", user_data.balance, balance_in_usd_formatted).blue()
            );
        }
        println!("{}", format!("Account Type: {}", user_data.account_type).blue());
        if let Some(email) = &user_data.email {
            if email.is_empty() {
                println!("{}", "No email registered".yellow());
            } else {
                println!("{}", format!("Email: {}", email).blue());
            }
        }
    } else {
        println!("{}", "No user available.".yellow());
    }

    // ------------------------ Futures Ticker Data ------------------------
    if let Some(ticker_data) = &bot_params.ticker_data {
        println!("{}", "\n--- Futures Ticker Data ---".green());
        println!("{}", format!("Last Price: ${}", ticker_data.last_price).blue());
        println!("{}", format!("Ask Price: ${}", ticker_data.ask_price).blue());
        println!("{}", format!("Bid Price: ${}", ticker_data.bid_price).blue());
        println!("{}", format!("Carry Fee Rate: {}", ticker_data.carry_fee_rate).blue());
        println!("{}", format!("Carry Fee Timestamp: {}", ticker_data.carry_fee_timestamp).blue());
    } else {
        println!("{}", "No Futures Ticker available.".yellow());
    }

    // ----------------------- Futures Market Data ------------------------
    if let Some(market_data) = &bot_params.market_data {
        println!("{}", "\n--- Futures Market Data ---".green());
        println!("{}", format!("Active: {}", market_data.active).blue());
        println!("{}", format!("Quantity Min: {}", market_data.limits.quantity.min).blue());
        println!("{}", format!("Quantity Max: {}", market_data.limits.quantity.max).blue());
        println!("{}", format!("Leverage Min: {}", market_data.limits.leverage.min).blue());
        println!("{}", format!("Leverage Max: {}", market_data.limits.leverage.max).blue());
        println!("{}", format!("Max Trades: {}", market_data.limits.count.max).blue());

        // Gebühren
        println!("{}", "\nTrading Fees:".green());
        for tier in &market_data.fees.trading.tiers {
            println!("{}", format!("Volume Min: {} - Fee: {}", tier.min_volume, tier.fees).blue());
        }
        println!("{}", format!("Carry Fee Min: {}", market_data.fees.carry.min).blue());
    } else {
        println!("{}", "No Futures Market Data available.".yellow());
    }

    // --------------------------- Indicators ---------------------------
    if let Some(indicators) = &bot_params.indicators {
        println!("{}", "\n--- Initial Indicators ---".green());
        
        // Moving Averages
        if let Some(ma) = indicators.ma {
            println!("{}", format!("Moving Average (MA): {}", ma).blue());
        }
        
        if let Some(ema) = indicators.ema {
            println!("{}", format!("Exponential Moving Average (EMA): {}", ema).blue());
        }

        // Bollinger Bands
        if let Some((lower, middle, upper)) = indicators.bollinger_bands {
            println!("{}", format!("Bollinger Bands - Upper: {}, Middle: {}, Lower: {}", upper, middle, lower).blue());
        }

        // Relative Strength Index (RSI)
        if let Some(rsi) = indicators.rsi {
            println!("{}", format!("Relative Strength Index (RSI): {}", rsi).blue());
        }

        // Indicator-MA, EMA, Bollinger Bands, RSI
        if let Some(i_ma) = indicators.i_ma {
            println!("{}", format!("Indicator MA: {}", i_ma).blue());
        }
        
        if let Some(i_ema) = indicators.i_ema {
            println!("{}", format!("Indicator EMA: {}", i_ema).blue());
        }
        
        if let Some((i_lower, i_middle, i_upper)) = indicators.i_bollinger_bands {
            println!("{}", format!("Indicator Bollinger Bands - Upper: {}, Middle: {}, Lower: {}", i_upper, i_middle, i_lower).blue());
        }

        if let Some(i_rsi) = indicators.i_rsi {
            println!("{}", format!("Indicator RSI: {}", i_rsi).blue());
        }

        // ATR (Average True Range)
        if let Some(atr) = indicators.atr {
            println!("{}", format!("Average True Range (ATR): {}", atr).blue());
        }

        // OHLC Indicators
        if let Some(ohlc_ma) = indicators.ohlc_ma {
            println!("{}", format!("OHLC MA: {}", ohlc_ma).blue());
        }

        if let Some(ohlc_ema) = indicators.ohlc_ema {
            println!("{}", format!("OHLC EMA: {}", ohlc_ema).blue());
        }

        if let Some((ohlc_lower, ohlc_middle, ohlc_upper)) = indicators.ohlc_bollinger_bands {
            println!("{}", format!("OHLC Bollinger Bands - Upper: {}, Middle: {}, Lower: {}", ohlc_upper, ohlc_middle, ohlc_lower).blue());
        }

        if let Some(ohlc_rsi) = indicators.ohlc_rsi {
            println!("{}", format!("OHLC RSI: {}", ohlc_rsi).blue());
        }
    } else {
        println!("{}", "No Indicators available.".yellow());
    }

    // --------------------------- Trades ---------------------------
    if let Some(trades) = &bot_params.trades {
        if trades.is_empty() {
            println!("No {} Futures Trades available.", trade_type);
        } else {
            println!("{}", "\n--- Trades ---".green());
            for trade in trades {
                println!("{}", format!("Trade: {:?}", trade).blue());
            }
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
        "\r{} --- {} {} {} at {}",
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
        println!("{}", "Updated Indicators:".blue());

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
        } else {
            println!("{}", "Moving Average (MA): Not Available".yellow());
        }

        if let Some(ema) = indicators.ema {
            println!("{}", format!("Exponential Moving Average (EMA): {}", ema).green());
        } else {
            println!("{}", "Exponential Moving Average (EMA): Not Available".yellow());
        }

        if let Some((upper, middle, lower)) = indicators.bollinger_bands {
            println!("{}", format!("Bollinger Bands - Upper: {}, Middle: {}, Lower: {}", upper, middle, lower).green());
        } else {
            println!("{}", "Bollinger Bands: Not Available".yellow());
        }

        if let Some(rsi) = indicators.rsi {
            println!("{}", format!("Relative Strength Index (RSI): {}", rsi).green());
        } else {
            println!("{}", "Relative Strength Index (RSI): Not Available".yellow());
        }

        if let Some(i_ma) = indicators.i_ma {
            println!("{}", format!("Indicator MA: {}", i_ma).green());
        } else {
            println!("{}", "Indicator MA: Not Available".yellow());
        }

        if let Some(i_ema) = indicators.i_ema {
            println!("{}", format!("Indicator EMA: {}", i_ema).green());
        } else {
            println!("{}", "Indicator EMA: Not Available".yellow());
        }

        if let Some((i_upper, i_middle, i_lower)) = indicators.i_bollinger_bands {
            println!("{}", format!("Indicator Bollinger Bands - Upper: {}, Middle: {}, Lower: {}", i_upper, i_middle, i_lower).green());
        } else {
            println!("{}", "Indicator Bollinger Bands: Not Available".yellow());
        }

        if let Some(i_rsi) = indicators.i_rsi {
            println!("{}", format!("Indicator RSI: {}", i_rsi).green());
        } else {
            println!("{}", "Indicator RSI: Not Available".yellow());
        }

        if let Some(atr) = indicators.atr {
            println!("{}", format!("Average True Range (ATR): {}", atr).green());
        } else {
            println!("{}", "Average True Range (ATR): Not Available".yellow());
        }

        if let Some(ohlc_ma) = indicators.ohlc_ma {
            println!("{}", format!("OHLC MA: {}", ohlc_ma).green());
        } else {
            println!("{}", "OHLC MA: Not Available".yellow());
        }

        if let Some(ohlc_ema) = indicators.ohlc_ema {
            println!("{}", format!("OHLC EMA: {}", ohlc_ema).green());
        } else {
            println!("{}", "OHLC EMA: Not Available".yellow());
        }

        if let Some((ohlc_upper, ohlc_middle, ohlc_lower)) = indicators.ohlc_bollinger_bands {
            println!("{}", format!("OHLC Bollinger Bands - Upper: {}, Middle: {}, Lower: {}", ohlc_upper, ohlc_middle, ohlc_lower).green());
        } else {
            println!("{}", "OHLC Bollinger Bands: Not Available".yellow());
        }

        if let Some(ohlc_rsi) = indicators.ohlc_rsi {
            println!("{}", format!("OHLC RSI: {}", ohlc_rsi).green());
        } else {
            println!("{}", "OHLC RSI: Not Available".yellow());
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