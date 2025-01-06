# ⚡ Lightning Trading Bot

## Overview

The Lightning Trading Bot is a high-performance Bitcoin Futures trading bot built in Rust, designed to interact with the LN Markets API. This bot leverages various market indicators such as Moving Averages (MA), Exponential Moving Averages (EMA), Bollinger Bands (BB), Relative Strength Index (RSI), and Average True Range (ATR) to analyze and execute trades on Bitcoin Futures markets.

Currently under active development, this bot is a proof-of-concept and is not yet suitable for production environments. It is intended for educational and experimental purposes only.

## Features

- 📈 **Real-Time Market Data**: Connects to LN Markets WebSocket to stream live market prices.
- ⚙️ **Customizable Trading Strategy**: Implements indicators like MA, EMA, BB, RSI, and ATR to calculate trading signals.
- 🛠️ **Stop-Loss and Take-Profit Calculation**: Dynamically calculates stop-loss and take-profit levels for each trade based on ATR and other parameters.
- 🧮 **Trade Quantity Calculation**: Automatically determines the optimal quantity for trades, taking account balance, leverage, and risk management into consideration.
- 🔄 **Signal Processing**: Evaluates buy, sell, hold, and strong buy/strong sell signals using a combination of price indicators.
- ⚖️ **Configurable Parameters**: Offers flexibility to adjust trading parameters, including technical indicators, leverage, stop-loss, take-profit settings, etc.
- 🚀 **Real-Time Trade Execution**: Executes trades based on generated signals using LN Markets API.
- 🔄 **Multi-Tasking**: Uses Tokio's async runtime to handle multiple tasks concurrently, including price data updates, indicator calculations, signal processing, and trade execution.

## Installation

### Prerequisites

- 🦀 **Rust**: Install Rust if you haven’t already. Follow the instructions [here](https://www.rust-lang.org/tools/install).
- ⚙️ **Cargo**: Cargo is the package manager and build tool for Rust. It is installed automatically with Rust.

### Step-by-Step Installation

1. **Clone the repository:**

    ```bash
    git clone https://github.com/RayRizzling/lightning-trading-bot.git
    cd lightning-trading-bot
    ```

2. **Set up your environment:**

    Create a `.env` file in the root directory of the project with the following content:

    ```env
    # Account API SECRETS
    LN_API_KEY=<> 
    LN_API_SECRET=<> 
    LN_API_PASSPHRASE=<> 

    # REST API URL (V2)
    LN_MAINNET_API_URL=https://api.lnmarkets.com/v2
    LN_TESTNET_API_URL=https://api.testnet.lnmarkets.com/v2

    # WEBSOCKET API ENDPOINTS
    LN_MAINNET_API_WS_ENDPOINT=wss://api.lnmarkets.com
    LN_TESTNET_API_WS_ENDPOINT=wss://api.testnet.lnmarkets.com

    # METHOD FOR WEBSOCKET PRICE FEED
    LN_PRICE_METHOD=v1/public/subscribe
    ```

3. **Install dependencies:**

    Run the following command to install dependencies:

    ```bash
    cargo build --release
    ```

4. **Run the bot:**

    After building the project, you can start the bot with:

    ```bash
    cargo run
    ```

    The bot will stream real-time price data, process signals, and execute trades based on the configured strategy.

## Configuration

The bot's behavior can be customized by modifying the `config.rs` file. You can set various parameters such as:

- ⏱️ **Trade Interval**: Interval for fetching market data and calculating indicators.
- 📊 **Technical Indicators**: Set the periods for MA, EMA, BB, RSI, and ATR.
- ⚙️ **Other Settings**: Configure other important parameters, including leverage, risk per trade, and risk-to-reward ratio.

Important: OHLC history data and live spot prices are used for signal derivation. The bot continuously updates parameters in real-time.

### Example Configuration

```rust
pub struct BotConfig {
    pub api_url: String,
    pub range: String,
    pub from: Option<i64>,
    pub to: Option<i64>,
    pub ma_period: usize,
    pub ema_period: usize,
    pub bb_period: usize,
    pub bb_std_dev_multiplier: f64,
    pub rsi_period: usize,
    pub atr_period: usize,
    pub trade_gap_seconds: u64,
    pub include_price_data: bool,
    pub include_index_data: bool,
    pub interval: Duration,
}
```
Modify these values in the `BotConfig` struct to adjust the bot’s trading parameters.

## Development Status

This bot is currently under active development. The core trading logic, including trade execution, stop-loss/take-profit calculations, and quantity optimization, is operational. Future improvements include enhanced error handling, better configuration options, and more robust trade management.

🚨 Disclaimer: This bot is not yet refined or suitable for production use. It is a raw development version, and you should use it at your own risk, especially with real funds.

## Contributing

🤝 Contributions are welcome! If you would like to improve the bot or add new features, please follow the steps below:

1. Fork the repository.
2. Create a new branch (`git checkout -b feature/your-feature`).
3. Make your changes and commit them (`git commit -am 'Add new feature'`).
4. Push to the branch (`git push origin feature/your-feature`).
5. Open a pull request.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.

## Acknowledgments

LN Markets API: The bot integrates with the LN Markets API for fetching price data and executing trades. For more information, please refer to the official documentation.
Rust Community: Special thanks to the Rust community for providing the tools and libraries that make this bot possible.

## Warning

⚠️ This bot is a work-in-progress and should be used with caution. It is intended for learning and experimentation, and you should not use it in a live environment with real funds unless you fully understand the code and its behavior.