// src/utils/connect_ws.rs

use serde::Deserialize;
use serde_json::json;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{StreamExt, SinkExt};
use tokio::sync::{mpsc, Mutex};
use uuid::Uuid;
use tokio::time::{Instant, Duration};
use std::sync::Arc;
use colored::*; // Für farbige Ausgaben

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PriceData {
    pub last_price: f64,
    pub last_tick_direction: String,
    pub time: i64,
}

/// Establishes a WebSocket connection to the price feed and handles the reception of price updates.
///
pub async fn ws_price_feed(
    mut shutdown_rx: mpsc::Receiver<()>,
    ws_endpoint: &str,
    method: &str,
    price_tx: mpsc::Sender<PriceData>, // Channel to transmit price data
) -> Result<(), Box<dyn std::error::Error>> {
    let price_tx = Arc::new(Mutex::new(price_tx)); // Wrap the sender in Arc<Mutex>

    loop {
        let (ws_stream, _) = match connect_async(ws_endpoint).await {
            Ok(ws) => ws,
            Err(_e) => {
                eprintln!("{}", "Error connecting to WebSocket. Retrying...".red());
                tokio::time::sleep(Duration::from_secs(5)).await;
                continue;
            }
        };

        println!("Connected to {}", ws_endpoint.purple());
        let (mut write, mut read) = ws_stream.split(); // Split the WebSocket stream into read and write parts

        // Subscribe to the price channel
        let channel = "futures:btc_usd:last-price";
        let subscription_request = json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": [channel],
            "id": Uuid::new_v4().to_string(), // Generate a unique ID for the subscription
        });

        // Send the subscription request
        if write.send(Message::Text(subscription_request.to_string())).await.is_err() {
            eprintln!("{}", "Error subscribing to channel.".red());
            continue; // Falls das Senden der Nachricht fehlschlägt, Verbindung erneut aufbauen
        }
        println!("Subscribed to: {}", channel.blue());

        // Track the time of the last received message
        let last_received = Arc::new(Mutex::new(Instant::now()));

        // Spawn a task to handle incoming messages
        tokio::spawn({
            let last_received = last_received.clone(); // Clone for use in the async block
            let price_tx = price_tx.clone(); // Clone the Arc<Mutex<Sender<PriceData>>>
            async move {
                while let Some(message) = read.next().await {
                    match message {
                        Ok(Message::Text(text)) => {
                            // Parse the received message
                            if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&text) {
                                if let Some(params) = parsed.get("params") {
                                    if let Some(data) = params.get("data") {
                                        if let Ok(price_data) = serde_json::from_value::<PriceData>(data.clone()) {
                                            let price_tx = price_tx.lock().await;
                                            if price_tx.send(price_data).await.is_err() {
                                                eprintln!("Failed to send price data.");
                                                break;
                                            }
                                        }
                                    }
                                }
                            }

                            // Update the time of the last received message
                            let mut last_received = last_received.lock().await;
                            *last_received = Instant::now();
                        }
                        Err(e) => {
                            eprintln!("Error receiving message: {}", e);
                            break;
                        }
                        _ => {}
                    }
                }
            }
        });

        // Heartbeat mechanism and shutdown handling
        let mut interval = tokio::time::interval(Duration::from_secs(5)); // Check every 5 seconds
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    // If 5 seconds have passed without receiving a message, send a ping
                    let last_received = last_received.lock().await;
                    if Instant::now().duration_since(*last_received) >= Duration::from_secs(5) {
                        if let Err(_e) = write.send(Message::Ping(vec![])).await {
                            // Log error in red and bold when ping fails
                            eprintln!("{}", "WEBSOCKET CONNECTION: LOST".red().bold());
                            break;
                        }
                        // Do not log anything if ping is successful
                    }
                }
                _ = shutdown_rx.recv() => {
                    // Handle shutdown signal
                    println!("Closing WebSocket connection...");
                    match write.send(Message::Close(None)).await {
                        Ok(_) => println!("WebSocket connection closed successfully."),
                        Err(e) => eprintln!("Error closing WebSocket connection: {}", e),
                    }
                    break; // Exit the inner loop after closing the WebSocket
                }
            }
        }

        // Here we exit the outer loop once the shutdown signal is received
        println!("Price feed stopped.");
        break; // Exit the outer loop to stop reconnecting
    }

    Ok(())
}