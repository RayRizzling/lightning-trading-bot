// src/main.rs

use config::load_config;
use math::update_data::update_data;
use tokio::signal;
use tokio::sync::{Mutex, mpsc};
use utils::log_bot_params::{log_bot_params, log_spot_price, log_updated_indicators};
use std::env;
use std::sync::Arc;
use colored::Colorize;
use utils::connect_ws::ws_price_feed;
use crate::futures::get_ohlcs_history::OhlcHistoryEntry;
use math::get_indicators::update_price_indicators;
use math::init_bot_params::{init_bot_params, BotParams};
use utils::update_bot_indicators::update_indicators;
use math::get_signals::{get_signals, Signal, SignalData};
use std::io::{self, Write};

mod config;
mod utils;
mod futures;
mod math;

//use futures::create_trade::create_market_buy_order;
//use futures::close_trade::close_trade;
//use futures::close_all_trades::close_all_trades;

#[tokio::main]
async fn main() {
    let config = load_config().await;
    let bot_params: Arc<Mutex<BotParams>>;

    // init signals channels
    let (signal_tx, signal_rx) = mpsc::channel::<SignalData>(15);
    let signal_tx = Arc::new(Mutex::new(signal_tx));
    let signal_tx_clone1 = Arc::clone(&signal_tx);
    let (signal_result_tx, mut signal_result_rx) = mpsc::channel::<Signal>(15);

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

    // update bot params continuously
    if let Some(ref indicators) = &bot_params.lock().await.indicators {
        let bot_params_clone: Arc<Mutex<BotParams>> = Arc::clone(&bot_params);

        let ohlc_data = Arc::new(Mutex::new(indicators.ohlc_data.clone()));
        let ohlc_data_clone = Arc::clone(&ohlc_data);
        let (tx, mut rx) = mpsc::channel::<Vec<OhlcHistoryEntry>>(5);
    
        tokio::spawn(async move {
            if let Err(e) = update_data(&config.api_url, config.interval, ohlc_data_clone, &config.range, tx).await {
                eprintln!("Error in update_data task: {}", e);
            }
        });

        // Spawn task to process updated OHLC data for fresh indicators by interval
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
                update_indicators(&mut bot_params, ohlc_data, ma, ema, bollinger_bands, rsi, atr, price_ma, price_ema, price_bollinger_bands, price_rsi, index_ma, index_ema, index_bollinger_bands, index_rsi);
                
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

    // // Continuously process spot price data feed and send to signal channel
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

    tokio::spawn(async move {
        // get signal
        tokio::spawn(async move {
            get_signals(signal_rx, signal_result_tx).await;
        });
    
        // process signal
        while let Some(signal) = signal_result_rx.recv().await {
            println!(" - {}", signal.to_string());
            io::stdout().flush().unwrap();
        }
    });
    
    

    signal::ctrl_c().await.expect("failed to listen for event");
    println!("{}", "");
    println!("Ctrl+C received, bot shutdown...");

    let _ = shutdown_tx.send(()).await;

    // Wait for the WebSocket task to finish
    handle.await.expect("Error shutting down the trading bot.");
    println!("Bot stopped successfully.")
}


    //     // let leverage = 20;          // Leverage (i.e. 1 = 1x)
    //     // let quantity = Some(1);      // amoount in USD (100 = 1 USD)
    //     // //let margin = Some(1029); // (1100 Sats)
        
    //     // match create_market_buy_order(
    //     //     &api_url, 
    //     //     leverage, 
    //     //     quantity, 
    //     //     None,
    //     //     None,
    //     //     None,
    //     //     None
    //     // ).await {
    //     //     Ok(trade) => println!("Market buy order created: {:?}", trade),
    //     //     Err(e) => eprintln!("Error: {}", e),
    //     // }

    //     // match close_all_trades(&api_url).await {
    //     //     Ok(response) => {
    //     //         println!("Successfully closed trades: {:?}", response.trades);
    //     //     }
    //     //     Err(e) => {
    //     //         eprintln!("Error closing all trades: {}", e);
    //     //     }
    //     // }

    //     // let trade_id = "d0e7a81e-7bec-40d7-8e0f-9d57efb67f97"; // ID des zu stornierenden Trades

    //     // match close_trade(&api_url, trade_id).await {
    //     //     Ok(response) => {
    //     //         println!("Trade successfully canceled:");
    //     //         println!("{:#?}", response);
    //     //     }
    //     //     Err(e) => {
    //     //         eprintln!("Error canceling trade: {}", e);
    //     //     }
    //     // }
    // });