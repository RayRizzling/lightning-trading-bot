// src/utils/log_bot_params.rs

use colored::Colorize;
use crate::{math::init_bot_params::BotParams, utils::get_timestamps::format_timestamp};
use tokio::time::Duration;
use std::io::{self, Write};

use super::{calculate_trade::TradeParams, connect_ws::PriceData};

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
                println!("{}", "");

                let formatted_creation_ts = format_timestamp(trade.creation_ts as i64);
                let formatted_last_update_ts = format_timestamp(trade.last_update_ts as i64);
                let formatted_closed_ts = trade.closed_ts.map(|ts| format_timestamp(ts as i64)).unwrap_or("N/A".to_string());
                let side_display = match trade.side.as_str() {
                    "s" => "Short".red().to_string(),
                    "b" => "Long".green().to_string(),
                    _ => trade.side.clone(),
                };
                let type_display = match trade.type_.as_str() {
                    "m" => "Market Order".to_string(),
                    "l" => "Limit Order".to_string(),
                    _ => trade.type_.clone(),
                };

                let opening_fee_usd = (trade.opening_fee / 100000000.0) * trade.entry_price.unwrap_or(0.0);
                let closing_fee_usd = (trade.closing_fee / 100000000.0) * trade.entry_price.unwrap_or(0.0);
                let margin_usd = (trade.margin / 100000000.0) * trade.entry_price.unwrap_or(0.0);
                let maintenance_margin_usd = (trade.maintenance_margin / 100000000.0) * trade.entry_price.unwrap_or(0.0);
                let sum_carry_fees_usd = (trade.sum_carry_fees / 100000000.0) * trade.entry_price.unwrap_or(0.0);
                let pl_usd = (trade.pl / 100000000.0) * trade.entry_price.unwrap_or(0.0);

                let potential_close_result_sats = trade.pl - (trade.opening_fee + trade.closing_fee);
                let potential_close_result_usd = pl_usd - (opening_fee_usd + closing_fee_usd);
                
                let potential_close_result_display = if potential_close_result_usd > 0.0 {
                    format!(
                        "Potential Close Result: +${:.2} (+{} sats)",
                        potential_close_result_usd,
                        potential_close_result_sats
                    )
                    .green()
                } else if potential_close_result_usd < 0.0 {
                    format!(
                        "Potential Close Result: -${:.2} (-{} sats)",
                        potential_close_result_usd.abs(),
                        potential_close_result_sats.abs()
                    )
                    .red()
                } else {
                    format!(
                        "Potential Close Result: ${:.2} ({} sats)",
                        potential_close_result_usd,
                        potential_close_result_sats
                    )
                    .white()
                };

                println!("{}", format!("Type: {}", type_display).blue());
                println!("{}", format!("Side: {}", side_display));
                println!("{}", format!("Quantity: $ {}", trade.quantity).blue());
                // orange color used (not available in Colorized)
                println!("{}", format!("\x1b[38;5;214mMargin: {} sats (~${:.2})\x1b[0m", trade.margin, margin_usd));
                println!("{}", potential_close_result_display);
                println!("{}", format!("Entry Price: ${:.2}", trade.entry_price.unwrap_or(0.0)).blue());
                println!(
                    "{}",
                    if trade.stoploss == 0.0 {
                        "Stop Loss: not set".magenta()
                    } else {
                        format!("Stop Loss: ${:.2}", trade.stoploss).blue()
                    }
                );
                println!(
                    "{}",
                    if trade.takeprofit == 0.0 {
                        "Take Profit: not set".magenta()
                    } else {
                        format!("Take Profit: ${:.2}", trade.takeprofit).blue()
                    }
                );
                println!("{}", format!("Exit Price: ${:.2}", trade.exit_price.unwrap_or(0.0)).blue());
                println!("{}", format!("Liquidation Price: ${:.2}", trade.liquidation).blue());
                println!("{}", format!("Opening Fee: {} sats (~${:.2})", trade.opening_fee, opening_fee_usd).blue());
                println!("{}", format!("Closing Fee: {} sats (~${:.2})", trade.closing_fee, closing_fee_usd).blue());
                println!("{}", format!("Maintenance Margin: {} sats (~${:.2})", trade.maintenance_margin, maintenance_margin_usd).blue());
                println!("{}", format!("P&L: {} sats (~${:.2})", trade.pl, pl_usd).blue());
                println!("{}", format!("Sum Carry Fees: {} sats (~${:.2})", trade.sum_carry_fees, sum_carry_fees_usd).blue());
                println!("{}", format!("Entry Margin: {:.0} sats", trade.entry_margin.unwrap_or(0.0)).blue());
                println!("{}", format!("Creation: {}", formatted_creation_ts).blue());
                println!("{}", format!("Last Update: {}", formatted_last_update_ts).blue());
                println!("{}", format!("Closed: {}", formatted_closed_ts).blue());
                if trade.open {
                    println!("{}", format!("Open").blue());
                }
                
                if trade.running {
                    println!("{}", format!("Running").green());
                }
                
                if trade.canceled {
                    println!("{}", format!("Canceled").red());
                }
                
                if trade.closed {
                    println!("{}", format!("Closed").red());
                }
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

        if let Some(ma) = indicators.ma {
            println!("{}", format!("Moving Average (MA): {}", ma).green());
        }

        if let Some(ema) = indicators.ema {
            println!("{}", format!("Exponential Moving Average (EMA): {}", ema).green());
        }

        if let Some((upper, middle, lower)) = indicators.bollinger_bands {
            println!(
                "{}",
                format!("Bollinger Bands - Upper: {}, Middle: {}, Lower: {}", upper, middle, lower).green()
            );
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
            println!(
                "{}",
                format!(
                    "Indicator Bollinger Bands - Upper: {}, Middle: {}, Lower: {}",
                    i_upper, i_middle, i_lower
                )
                .green()
            );
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

        if let Some((ohlc_upper, ohlc_middle, ohlc_lower)) = indicators.ohlc_bollinger_bands {
            println!(
                "{}",
                format!(
                    "OHLC Bollinger Bands - Upper: {}, Middle: {}, Lower: {}",
                    ohlc_upper, ohlc_middle, ohlc_lower
                )
                .green()
            );
        }

        if let Some(ohlc_rsi) = indicators.ohlc_rsi {
            println!("{}", format!("OHLC RSI: {}", ohlc_rsi).green());
        }
        println!("{}", "");
    } else {
        println!("{}", "");
        println!("{}", "No updated Indicators available.".red());
        println!("{}", "");
    }
}

pub fn log_forecast_trade(
    entry_p: f64,
    takeprofit: Option<u64>,
    stoploss: Option<u64>,
    trade_params: &TradeParams
) {
    println!(
        "{} {}",
        "Trade Forecast".bold().underline().green(),
        "Parameters:".yellow()
    );

    // Log Margin, Maintenance Margin, and Liquidation Price with Colorized Output
    println!(
        "{} {} Sats ({} + {})",
        "Margin + Maintenance Margin:".cyan(),
        (trade_params.margin_sats + trade_params.maintenance_margin).to_string().bold(),
        trade_params.margin_sats.to_string().blue(),
        trade_params.maintenance_margin.to_string().blue()
    );

    println!(
        "{}: {}",
        "Liquidation Price:".red(),
        trade_params.liquidation_price.to_string().bold()
    );

    println!(
        "{}: {} at entry: {}$",
        "Quantity:".magenta(),
        trade_params.trade_quantity.to_string().bold(),
        entry_p.to_string().green()
    );

    // Take Profit and Stop Loss with colored options
    match (takeprofit, stoploss) {
        (Some(tp), Some(sl)) => {
            println!(
                "{}: {}$ --- {}: {}$",
                "Take Profit:".green(),
                tp.to_string().bold(),
                "Stop Loss:".red(),
                sl.to_string().red(),
            );
        }
        _ => {
            println!(
                "{} {}$",
                "No Take Profit or Stop Loss defined.".yellow(),
                entry_p.to_string().green()
            );
        }
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