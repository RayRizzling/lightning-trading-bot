// src/math/get_signals.rs

use crate::{config::load_signal_settings, utils::connect_ws::PriceData};
use tokio::sync::mpsc;
use super::get_indicators::Indicators;
use colored::Colorize;

#[derive(Clone)]
#[allow(dead_code)]
pub struct SignalData {
    pub price_data: Option<PriceData>,
    pub indicators: Option<Indicators>,
}

#[derive(Debug, Clone, Copy)]
pub enum Signal {
    StrongSell,
    Sell,
    Hold,
    Buy,
    StrongBuy,
    Undefined,
}

impl Signal {
    pub fn to_string(&self) -> String {
        match self {
            Signal::StrongSell => "Signal: Strong Sell 🚫".red().to_string(),
            Signal::Sell => "Signal: Sell 🚩".red().to_string(),
            Signal::Hold => "Signal: Hold".white().to_string(),
            Signal::Buy => "Signal: Buy ✅".green().to_string(),
            Signal::StrongBuy => "Signal: Strong Buy 💚".green().to_string(),
            Signal::Undefined => "Signal: Undefined".yellow().to_string(),
        }
    }
}

pub async fn get_signals(mut rx: mpsc::Receiver<SignalData>, tx: mpsc::Sender<Signal>) {
    let mut last_signal: Option<SignalData> = None;

    while let Some(signal_data) = rx.recv().await {
        let mut updated_signal = signal_data.clone();

        if updated_signal.price_data.is_none() {
            if let Some(last) = &last_signal {
                updated_signal.price_data = last.price_data.clone();
            }
        }

        if updated_signal.indicators.is_none() {
            if let Some(last) = &last_signal {
                updated_signal.indicators = last.indicators.clone();
            }
        }

        last_signal = Some(updated_signal.clone());

        if let (Some(price_data), Some(indicators)) = (updated_signal.price_data, updated_signal.indicators) {
            let signal_value = calculate_ohlc_with_price_signal(&price_data, &indicators).await;
            let signal = match signal_value {
                -2 => Signal::StrongSell,
                -1 => Signal::Sell,
                0  => Signal::Hold,
                1  => Signal::Buy,
                2  => Signal::StrongBuy,
                _  => Signal::Undefined,
            };

            if tx.send(signal).await.is_err() {
                eprintln!("Error sending signal");
            }
        }
    }
}

pub async fn calculate_ohlc_with_price_signal(price_data: &PriceData, indicators: &Indicators) -> i32 {
    let mut signal = 0.0;

    if price_data.last_price <= 0.0 {
        println!("Invalid price_data.last_price: {}", price_data.last_price);
        return 0; // Hold Signal
    }
    
    // Ungültige Indikatoren prüfen
    if let Some(bollinger_bands) = indicators.ohlc_bollinger_bands {
        let (lower, middle, upper) = bollinger_bands;
        if lower < 0.0 || middle < 0.0 || upper < 0.0 {
            println!("Invalid Bollinger Bands values: {:?}", bollinger_bands);
            return 0; // Hold Signal
        }
    }
    
    if let Some(rsi) = indicators.ohlc_rsi {
        if rsi < 0.0 || rsi > 100.0 {
            println!("Invalid RSI value: {}", rsi);
            return 0; // Hold Signal
        }
    }
    
    if let Some(ma) = indicators.ohlc_ma {
        if ma < 0.0 {
            println!("Invalid MA value: {}", ma);
            return 0; // Hold Signal
        }
    }
    
    if let Some(ema) = indicators.ohlc_ema {
        if ema < 0.0 {
            println!("Invalid EMA value: {}", ema);
            return 0; // Hold Signal
        }
    }
    
    if let Some(atr) = indicators.atr {
        if atr < 0.0 {
            println!("Invalid ATR value: {}", atr);
            return 0; // Hold Signal
        }
    }

    let settings = load_signal_settings().await;

    // Bollinger Bands Check
    if let Some(bollinger_bands) = indicators.ohlc_bollinger_bands {
        let (lower, _, upper) = bollinger_bands;
        if price_data.last_price > upper + settings.gap_value {  // Strong Sell Condition
            signal += (settings.bollinger_weight * -2.0) as f64;  // Strong Sell Signal
        } else if price_data.last_price < lower - settings.gap_value {  // Strong Buy Condition
            signal += (settings.bollinger_weight * 2.0) as f64;  // Strong Buy Signal
        } else if price_data.last_price > upper {
            signal += (settings.bollinger_weight * -1.0) as f64;  // Sell Signal
        } else if price_data.last_price < lower {
            signal += (settings.bollinger_weight * 1.0) as f64;  // Buy Signal
        }
    }

    //println!("Signal after Bollinger: {}", signal);

    // RSI Check
    if let Some(rsi) = indicators.ohlc_rsi {
        if rsi > 80.0 {  // Strong Sell
            signal += (settings.rsi_weight * -2.0) as f64;  // Strong Sell Signal
        } else if rsi > 70.0 {  // Normal Sell
            signal += (settings.rsi_weight * -1.0) as f64;  // Sell Signal
        } else if rsi < 20.0 {  // Strong Buy
            signal += (settings.rsi_weight * 2.0) as f64;  // Strong Buy Signal
        } else if rsi < 30.0 {  // Normal Buy
            signal += (settings.rsi_weight * 1.0) as f64;  // Buy Signal
        }
    }

    //println!("Signal after RSI: {}", signal);

    // MA Check
    if let Some(ma) = indicators.ohlc_ma {
        if price_data.last_price > ma + settings.gap_value { 
            signal += (settings.ma_ema_weight * -2.0) as f64;  // Strong Sell Signal
        } else if price_data.last_price > ma {
            signal += (settings.ma_ema_weight * -1.0) as f64;  // Sell Signal
        } else if price_data.last_price < ma - settings.gap_value {
            signal += (settings.ma_ema_weight * 2.0) as f64;  // Strong Buy Signal
        } else if price_data.last_price < ma {
            signal += (settings.ma_ema_weight * 1.0) as f64;  // Buy Signal
        } else {
            signal += 0.0;  // Hold Signal
        }
    }

    //println!("Signal after MA: {}", signal);

    // EMA Check
    if let Some(ema) = indicators.ohlc_ema {
        if price_data.last_price > ema + settings.gap_value {  
            signal += (settings.ma_ema_weight * -2.0) as f64;  // Strong Sell Signal
        } else if price_data.last_price > ema {
            signal += (settings.ma_ema_weight * -1.0) as f64;  // Sell Signal
        } else if price_data.last_price < ema - settings.gap_value {  
            signal += (settings.ma_ema_weight * 2.0) as f64;  // Strong Buy Signal
        } else if price_data.last_price < ema {
            signal += (settings.ma_ema_weight * 1.0) as f64;  // Buy Signal
        } else {
            signal += 0.0;  // Hold Signal
        }
    }

    //println!("Signal after EMA: {}", signal);


    // ATR Check
    if let Some(atr) = indicators.atr {
        let high_volatility_threshold = price_data.last_price * 0.005; // 0.5% of the spot price
        let strong_buy_threshold = high_volatility_threshold * 1.5; // e.g., 1.5x volatility
        let strong_sell_threshold = high_volatility_threshold * 1.75; // e.g., 1.75x volatility
    
        // Strong Sell Condition
        if atr > strong_sell_threshold && price_data.last_price > atr + strong_sell_threshold {
            signal += (settings.atr_weight * -2.0) as f64;
        }
        // Strong Buy Condition
        else if atr > strong_sell_threshold && price_data.last_price < atr - strong_buy_threshold {
            signal += (settings.atr_weight * 2.0) as f64;
        }
        // Sell Condition
        else if atr > high_volatility_threshold && price_data.last_price > atr {
            signal += (settings.atr_weight * -1.0) as f64;
        }
        // Buy Condition
        else if atr > high_volatility_threshold && price_data.last_price < atr {
            signal += (settings.atr_weight * 1.0) as f64;
        }
        // Hold Condition
        else {
            signal += 0.0; // Hold signal
        }
    }
    
    println!("Signal after ATR (final): {}", signal);

    // Final Signal Determination
    if signal >= 1.55 {  // Strong Buy Condition
        return 2;  // Strong Buy Signal
    } else if signal > 0.2 {
        return 1;  // Buy Signal
    } else if signal <= -1.55 {  // Strong Sell Condition
        return -2;  // Strong Sell Signal
    } else if signal < -0.2 {
        return -1;  // Sell Signal
    } else {
        return 0;  // Hold Signal
    }
}
