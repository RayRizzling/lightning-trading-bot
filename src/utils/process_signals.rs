// src/utils/process_signals.rs

use tokio::sync::mpsc::Receiver;
use tokio::time::Duration;
use std::io::{self, Write};
use std::sync::Arc;
use colored::Colorize;
use crate::math::create_trade_from_signal::{create_trade_from_signal, CreateTradeResult};
use crate::utils::init_bot_params::BotParams;
use crate::math::get_signals::SignalResponse;

pub async fn process_signals(
    mut signal_result_rx: Receiver<SignalResponse>,
    api_url: Arc<str>,
    bot_params: Arc<tokio::sync::Mutex<BotParams>>,
    trade_gap_seconds: u64,
    risk_per_trade_percent: f64,
    risk_to_reward_ratio: f64,
    risk_to_loss_ratio: f64,
) {
    let mut last_trade_time = tokio::time::Instant::now();

    while let Some(signal_response) = signal_result_rx.recv().await {
        let signal = signal_response.signal;
        let indicators = signal_response.indicators;

        // Log the signal
        println!(" - {}", signal.to_string());
        io::stdout().flush().unwrap();

        // Check if enough time has passed for a new trade
        if last_trade_time.elapsed() >= Duration::new(trade_gap_seconds, 0) {
            last_trade_time = tokio::time::Instant::now();

            let bot_params = Arc::clone(&bot_params);
            let api_url = Arc::clone(&api_url);

            tokio::spawn(async move {
                match create_trade_from_signal(
                    signal,
                    &api_url,
                    bot_params,
                    indicators,
                    None,
                    risk_per_trade_percent,
                    risk_to_reward_ratio,
                    risk_to_loss_ratio,
                )
                .await
                {
                    Ok(CreateTradeResult::TradeCreated) => {
                        println!(
                            "{}",
                            format!(
                                "Trade successfully created for signal: {}",
                                signal.to_string()
                            )
                            .green()
                        );
                    }
                    Ok(CreateTradeResult::NoTradeCreated(reason)) => {
                        println!("{}", format!("No trade created: {}", reason).yellow());
                    }
                    Err(e) => {
                        eprintln!("{}", format!("Error creating trade: {}", e).red());
                    }
                }
            });
        } else {
            println!("{}", "...skipped.".cyan());
        }
    }
}
