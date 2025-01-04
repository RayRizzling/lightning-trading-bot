// tests/price_indicators.rs

use trading_backend::math::price_indicators::{
    calculate_moving_average, calculate_exponential_moving_average,
    calculate_bollinger_bands, calculate_rsi, calculate_atr,
};

#[cfg(test)]
mod tests {
    use super::*;
    
    // Test for Simple Moving Average (SMA)
    #[test]
    fn test_moving_average() {
        let prices = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let period = 3;
        let result = calculate_moving_average(&prices, period);
        
        // Expected value: (3 + 4 + 5) / 3 = 4.0
        let expected = (3.0 + 4.0 + 5.0) / 3.0;
        assert_eq!(result, Some(expected));
    }

    // Test for Simple Moving Average (SMA) with insufficient data
    #[test]
    fn test_moving_average_short_data() {
        let prices = vec![1.0, 2.0];
        let period = 3;
        let result = calculate_moving_average(&prices, period);
        assert_eq!(result, None); // Not enough data to calculate SMA
    }

    // Test for Exponential Moving Average (EMA)
    #[test]
    fn test_ema() {
        let prices = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let period = 3;
        let result = calculate_exponential_moving_average(&prices, period);
        
        // Expected value should be calculated using the EMA formula
        let expected_ema = 4.0;  // Example expected value
        assert_eq!(result, Some(expected_ema));
    }

    // Test for Bollinger Bands
    #[test]
    fn test_bollinger_bands() {
        let prices = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let period = 3;
        let std_dev_multiplier = 2.0;
        let result = calculate_bollinger_bands(&prices, period, std_dev_multiplier);
    
        // Erwartete Werte für Bollinger-Bänder
        let expected_middle_band = (3.0 + 4.0 + 5.0) / 3.0; // Durchschnitt der letzten 3 Preise
        let expected_std_dev = ((3.0_f64 - expected_middle_band).powi(2)
            + (4.0_f64 - expected_middle_band).powi(2)
            + (5.0_f64 - expected_middle_band).powi(2))
            / 3.0_f64;
    
        let expected_std_dev = expected_std_dev.sqrt();
    
        let expected_upper_band = expected_middle_band + 2.0 * expected_std_dev;
        let expected_lower_band = expected_middle_band - 2.0 * expected_std_dev;
    
        assert!(result.is_some());
    
        let (lower, middle, upper) = result.unwrap();
    
        // Überprüfe die berechneten Werte
        assert!((middle - expected_middle_band).abs() < 1e-4);
        assert!((upper - expected_upper_band).abs() < 1e-4);
        assert!((lower - expected_lower_band).abs() < 1e-4);
    }
    

    // Test for Relative Strength Index (RSI)
    #[test]
    fn test_rsi() {
        let prices = vec![44.0, 44.1, 44.2, 44.3, 44.0, 43.8, 44.0];
        let period = 3;
        let result = calculate_rsi(&prices, period);
        
        let expected_rsi = 52.0;
        assert!(result.is_some());
        assert!((result.unwrap() - expected_rsi).abs() < 1e-4);
    }
    
    // Test for Average True Range (ATR)
    #[test]
    fn test_atr() {
        let highs = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let lows = vec![0.5, 1.5, 2.5, 3.5, 4.5];
        let closes = vec![0.75, 1.75, 2.75, 3.75, 4.75];
        let period = 3;
        let result = calculate_atr(&highs, &lows, &closes, period);
    
        let expected_atr = 1.25;
        assert!(result.is_some());
        assert!((result.unwrap() - expected_atr).abs() < 1e-4);
    }
}
