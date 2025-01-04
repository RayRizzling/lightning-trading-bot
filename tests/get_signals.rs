// src/tests/get_signals.rs

use trading_backend::utils::connect_ws::PriceData;
use trading_backend::math::get_indicators::Indicators;
use trading_backend::math::get_signals::calculate_ohlc_with_price_signal;

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn create_mock_price_data(last_price: f64) -> PriceData {
        PriceData {
            last_price,
            last_tick_direction: "UP".to_string(),
            time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
        }
    }

    fn create_mock_indicators(
        ohlc_bollinger_bands: Option<(f64, f64, f64)>,
        ohlc_rsi: Option<f64>,
        ohlc_ma: Option<f64>,
        ohlc_ema: Option<f64>,
        atr: Option<f64>,
    ) -> Indicators {
        Indicators {
            ohlc_data: vec![],             // Empty OHLC data
            price_data: vec![],            // Empty price data
            index_price_data: vec![],      // Empty index data
            ma: None,
            ema: None,
            bollinger_bands: None,
            rsi: None,
            i_ma: None,
            i_ema: None,
            i_bollinger_bands: None,
            i_rsi: None,
            atr,
            ohlc_ma,
            ohlc_ema,
            ohlc_bollinger_bands,
            ohlc_rsi,
        }
    }

    #[tokio::test]
    async fn test_strong_buy() {
        let price_data = create_mock_price_data(95_000.0); // Very low price
        let indicators = create_mock_indicators(
            Some((97_000.0, 98_000.0, 99_000.0)), // Bollinger Bands
            Some(10.0),                           // RSI (oversold)
            Some(98_500.0),                       // MA
            Some(98_800.0),                       // EMA
            Some(30.0),                           // ATR (high volatility)
        );

        let signal = calculate_ohlc_with_price_signal(&price_data, &indicators).await;
        assert_eq!(signal, 2); // Strong Buy
    }

    #[tokio::test]
    async fn test_strong_sell() {
        let price_data = create_mock_price_data(105_000.0); // Very high price
        let indicators = create_mock_indicators(
            Some((97_000.0, 98_000.0, 99_000.0)), // Bollinger Bands
            Some(90.0),                           // RSI (overbought)
            Some(98_500.0),                       // MA
            Some(98_800.0),                       // EMA
            Some(30.0),                           // ATR (high volatility)
        );

        let signal = calculate_ohlc_with_price_signal(&price_data, &indicators).await;
        assert_eq!(signal, -2); // Strong Sell
    }

    #[tokio::test]
    async fn test_buy() {
        let price_data = create_mock_price_data(97_500.0); // Slightly low price
        let indicators = create_mock_indicators(
            Some((97_000.0, 98_000.0, 99_000.0)), // Bollinger Bands
            Some(40.0),                           // RSI (neutral-slightly oversold)
            Some(98_000.0),                       // MA
            Some(98_500.0),                       // EMA
            Some(15.0),                           // ATR (moderate volatility)
        );

        let signal = calculate_ohlc_with_price_signal(&price_data, &indicators).await;
        assert_eq!(signal, 1); // Buy
    }

    #[tokio::test]
    async fn test_sell() {
        let price_data = create_mock_price_data(99_500.0); // Slightly high price
        let indicators = create_mock_indicators(
            Some((97_000.0, 98_000.0, 99_000.0)), // Bollinger Bands
            Some(60.0),                           // RSI (neutral-slightly overbought)
            Some(98_000.0),                       // MA
            Some(98_000.0),                       // EMA
            Some(20.0),                           // ATR (moderate volatility)
        );

        let signal = calculate_ohlc_with_price_signal(&price_data, &indicators).await;
        assert_eq!(signal, -1); // Sell
    }

    #[tokio::test]
    async fn test_hold() {
        let price_data = create_mock_price_data(98_000.0); // Neutral price
        let indicators = create_mock_indicators(
            Some((97_000.0, 98_000.0, 99_000.0)), // Bollinger Bands
            Some(50.0),                           // RSI (neutral)
            Some(98_000.0),                       // MA
            Some(98_000.0),                       // EMA
            Some(10.0),                           // ATR (low volatility)
        );

        let signal = calculate_ohlc_with_price_signal(&price_data, &indicators).await;
        assert_eq!(signal, 0); // Hold
    }

    #[tokio::test]
    async fn test_invalid_data() {
        let price_data = create_mock_price_data(-1.0); // Invalid price
        let indicators = create_mock_indicators(
            Some((-1.0, -1.0, -1.0)), // Invalid Bollinger Bands
            Some(-1.0),               // Invalid RSI
            Some(-1.0),               // Invalid MA
            Some(-1.0),               // Invalid EMA
            Some(-1.0),               // Invalid ATR
        );

        let signal = calculate_ohlc_with_price_signal(&price_data, &indicators).await;
        assert_eq!(signal, 0); // Hold, invalid data should not affect signals
    }

    #[tokio::test]
    async fn test_empty_data() {
        let price_data = create_mock_price_data(98_000.0); // Neutral price
        let indicators = create_mock_indicators(
            None, // No Bollinger Bands
            None, // No RSI
            None, // No MA
            None, // No EMA
            None, // No ATR
        );

        let signal = calculate_ohlc_with_price_signal(&price_data, &indicators).await;
        assert_eq!(signal, 0); // Hold, no indicators available
    }
}
