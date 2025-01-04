// src/utils/update_bot_indicators.rs

use crate::{futures::get_ohlcs_history::OhlcHistoryEntry, math::init_bot_params::BotParams};

pub fn update_indicators(
    bot_params: &mut BotParams,
    ohlc_data: Vec<OhlcHistoryEntry>,
    ma: Option<f64>,
    ema: Option<f64>,
    bollinger_bands: Option<(f64, f64, f64)>,
    rsi: Option<f64>,
    atr: Option<f64>,
    price_ma: Option<f64>,
    price_ema: Option<f64>,
    price_bollinger_bands: Option<(f64, f64, f64)>,
    price_rsi: Option<f64>,
    index_ma: Option<f64>,
    index_ema: Option<f64>,
    index_bollinger_bands: Option<(f64, f64, f64)>,
    index_rsi: Option<f64>,
) {
    // Update OHLC data and indicators in bot_params
    if let Some(ref mut indicators) = bot_params.indicators {

        indicators.ohlc_data = ohlc_data;

        indicators.ohlc_ma = ma;
        indicators.ohlc_ema = ema;
        indicators.ohlc_bollinger_bands = bollinger_bands;
        indicators.ohlc_rsi = rsi;
        indicators.atr = atr;

        // Update price indicators only if values are not None
        if price_ma.is_some() || price_ema.is_some() || price_bollinger_bands.is_some() || price_rsi.is_some() {
            if let Some(price_ma) = price_ma {
                indicators.ma = Some(price_ma);
            }
            if let Some(price_ema) = price_ema {
                indicators.ema = Some(price_ema);
            }
            if let Some(price_bollinger_bands) = price_bollinger_bands {
                indicators.bollinger_bands = Some(price_bollinger_bands);
            }
            if let Some(price_rsi) = price_rsi {
                indicators.rsi = Some(price_rsi);
            }
        }

        // Update index indicators only if values are not None
        if index_ma.is_some() || index_ema.is_some() || index_bollinger_bands.is_some() || index_rsi.is_some() {
            if let Some(index_ma) = index_ma {
                indicators.i_ma = Some(index_ma);
            }
            if let Some(index_ema) = index_ema {
                indicators.i_ema = Some(index_ema);
            }
            if let Some(index_bollinger_bands) = index_bollinger_bands {
                indicators.i_bollinger_bands = Some(index_bollinger_bands);
            }
            if let Some(index_rsi) = index_rsi {
                indicators.i_rsi = Some(index_rsi);
            }
        }
    } else {
        eprintln!("No indicators available in bot_params.");
    }
}
