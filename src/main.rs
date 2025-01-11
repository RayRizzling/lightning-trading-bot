// src/main.rs

use config::load_config;
use utils::update_history_data::update_history_data;
use tokio::signal;
use tokio::sync::{Mutex, mpsc};
use utils::log_bot_params::{log_bot_params, log_spot_price, log_updated_indicators};
use utils::process_signals::process_signals;
use std::env;
use std::sync::Arc;
use colored::Colorize;
use utils::connect_ws::ws_price_feed;
use crate::futures::get_ohlcs_history::OhlcHistoryEntry;
use math::get_indicators::update_price_indicators;
use utils::init_bot_params::{init_bot_params, BotParams};
use utils::set_updated_indicators::set_updated_indicators;
use math::get_signals::{get_signals, SignalData, SignalResponse};

mod config;
mod utils;
mod futures;
mod math;

#[tokio::main]
async fn main() {
    let config = load_config().await;
    let api_url = config.api_url.clone();
    let bot_params: Arc<Mutex<BotParams>>;

    // init signals channels
    let (signal_tx, signal_rx) = mpsc::channel::<SignalData>(15);
    let signal_tx = Arc::new(Mutex::new(signal_tx));
    let signal_tx_clone1 = Arc::clone(&signal_tx);
    let (signal_result_tx, signal_result_rx) = mpsc::channel::<SignalResponse>(15);

    // init bot params
    match init_bot_params(
        &config.api_url,
        &config.range,
        config.from,
        config.to,
        config.ma_period,
        config.ema_period,
        config.bb_period,
        config.bb_std_dev_multiplier,
        config.rsi_period,
        config.atr_period,
        &config.trade_type,
        config.include_price_data,
        config.include_index_data
    ).await {
        Ok(initialized_bot_params) => {
            bot_params = Arc::new(Mutex::new(initialized_bot_params));
            println!("\n{} Bot Params Initialization {}\n", "===" .bold(), "===");
            
            log_bot_params(&*bot_params.lock().await, &config.trade_type, config.formatted_from, config.formatted_to);
    
            println!("{}", "===" .bold());

            // add bot_params.indicators to signals channel
            let bot_params_locked = bot_params.lock().await;
            if let Some(indicators) = &bot_params_locked.indicators {
                let signal_data = SignalData {
                    price_data: None,
                    indicators: Some(indicators.clone()),
                };
    
                let signal_tx_locked = signal_tx.lock().await;
                signal_tx_locked.send(signal_data).await.unwrap();
            }
        }
        Err(e) => {
            eprintln!("{}", format!("Error initializing Bot Params: {}", e).red());
            return;
        }
    }

    // update bot params (history data and derived indicators) continuously
    if let Some(ref indicators) = &bot_params.lock().await.indicators {
        let bot_params_clone: Arc<Mutex<BotParams>> = Arc::clone(&bot_params);

        let ohlc_data = Arc::new(Mutex::new(indicators.ohlc_data.clone()));
        let ohlc_data_clone = Arc::clone(&ohlc_data);
        let (tx, mut rx) = mpsc::channel::<Vec<OhlcHistoryEntry>>(5);
    
        // task to update ohlc data on interval (index and price history data not integrated in v0.1.0)
        tokio::spawn(async move {
            if let Err(e) = update_history_data(&config.api_url, config.interval, ohlc_data_clone, &config.range, tx).await {
                eprintln!("Error in update_data task: {}", e);
            }
        });

        // task to process updated OHLC data for fresh indicators by interval
        tokio::spawn(async move {
            while let Some(ohlc_data) = rx.recv().await {
                let (ma, ema, bollinger_bands, rsi, atr, price_ma, price_ema, price_bollinger_bands, price_rsi, index_ma, index_ema, index_bollinger_bands, index_rsi) =
                    update_price_indicators(
                        &ohlc_data,
                        config.ma_period,
                        config.ema_period,
                        config.bb_period,
                        config.bb_std_dev_multiplier,
                        config.rsi_period,
                        config.atr_period,
                        None,
                        None,
                    );

                let mut bot_params = bot_params_clone.lock().await;
                set_updated_indicators(&mut bot_params, ohlc_data, ma, ema, bollinger_bands, rsi, atr, price_ma, price_ema, price_bollinger_bands, price_rsi, index_ma, index_ema, index_bollinger_bands, index_rsi);
                
                log_updated_indicators(&bot_params);

                // add indicators to signal channel
                let signal_data = SignalData {
                    price_data: None,
                    indicators: Some(bot_params.indicators.clone().unwrap()),
                };
                let signal_tx_locked = signal_tx.lock().await;
                signal_tx_locked.send(signal_data).await.unwrap();
            }
        });
    } else {
        eprintln!("Indicators not initialized.");
    }
    
    // channel for price data
    let (price_tx, mut price_rx) = mpsc::channel(10);
    // channel for shutdown signal
    let (shutdown_tx, shutdown_rx) = mpsc::channel(1);

    // Start the WebSocket task
    let handle = tokio::spawn(async move {
        let ws_endpoint = env::var("LN_MAINNET_API_WS_ENDPOINT").expect("WebSocket Endpoint Not Found");
        let method = env::var("LN_PRICE_METHOD").expect("Price Method for Price Feed Not Found");
        if let Err(e) = ws_price_feed(shutdown_rx, &ws_endpoint, &method, price_tx).await {
            eprintln!("Error: {}", e);
        }
    });

    // Continuously process spot price data feed and send to signal channel
    tokio::spawn({
        async move {
            while let Some(price_data) = price_rx.recv().await {
                
                log_spot_price(&price_data).await;
    
                let signal_data = SignalData {
                    price_data: Some(price_data.clone()),
                    indicators: None,
                };
    
                // Lock the Mutex and send the data
                let signal_tx_lock = signal_tx_clone1.lock().await;
                if let Err(e) = signal_tx_lock.send(signal_data).await {
                    eprintln!("Error sending signal data: {}", e);
                }
            }
        }
    });

    // get signal
    tokio::spawn(async move {
        get_signals(signal_rx, signal_result_tx).await;
    });

    // process signal (log signal & create trade)
    tokio::spawn({
        let bot_params = Arc::clone(&bot_params);
        let api_url = Arc::clone(&api_url).to_string().into();
        async move {
            process_signals(
                signal_result_rx,
                api_url,
                bot_params,
                config.trade_gap_seconds,
                config.risk_per_trade_percent,
                config.risk_to_reward_ratio,
                config.risk_to_loss_ratio,
            )
            .await;
        }
    });
    
    // TO DO: revalidate running trades on interval

    signal::ctrl_c().await.expect("failed to listen for shutdown event");
    println!("{}", "");
    println!("Ctrl+C received, bot shutdown...");

    let _ = shutdown_tx.send(()).await;

    // Wait for the WebSocket task to finish
    handle.await.expect("Error shutting down the trading bot.");
    println!("Bot stopped successfully.")
}
