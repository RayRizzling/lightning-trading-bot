# ⚡ Lightning Trading Bot

## Overview

The Lightning Trading Bot is a high-performance Bitcoin Futures trading bot built in Rust, designed to interact with the LN Markets API. This bot leverages various market indicators such as Moving Averages (MA), Exponential Moving Averages (EMA), Bollinger Bands (BB), Relative Strength Index (RSI), and Average True Range (ATR) to analyze and execute trades on Bitcoin Futures markets.

Currently under active development, this bot is a proof-of-concept and is not yet suitable for production environments. It is intended for educational and experimental purposes only.

## Features

- 📈 **Real-Time Market Data**: Connects to LN Markets WebSocket to stream live market prices.
- ⚙️ **Customizable Trading Strategy**: Implements indicators like MA, EMA, BB, RSI, and ATR to calculate trading signals.
- 🔄 **Signal Processing**: Evaluates buy, sell, hold, and strong buy/strong sell signals using a combination of price indicators.
- ⚖️ **Configurable Parameters**: Offers flexibility to adjust trading parameters, including technical indicators, leverage, stop-loss, take-profit settings, etc.
- 🔄 **Multi-Tasking**: Uses Tokio's async runtime to handle multiple tasks concurrently, including price data updates, indicator calculations, and signal processing.

- 💡 **Trade Execution (Planned)**: Real-time trade execution based on generated signals is a planned feature and will be added in future updates.

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

2. **Install dependencies:**

    Make sure to set up your `.env` file with your API credentials and other configuration parameters. Create a `.env` file in the root directory of the project with the following content:

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

    Then install the dependencies by running:

    ```bash
    cargo build --release
    ```

3. **Run the bot:**

    After building the project, you can start the bot with:

    ```bash
    cargo run
    ```

    The bot will start streaming real-time price data, process signals, and execute trades based on the configured strategy (last is planned).

## Configuration

The bot's behavior can be customized by modifying the `config.rs` file. You can set various parameters such as:

- ⏱️ **Trade Interval**: Interval for fetching market data and calculating indicators.
- 📊 **Technical Indicators**: Set the periods for MA, EMA, BB, RSI, and ATR.
- ⚙️ **Other Settings**: Configure other important parameters.

Important notice: OHLCs history data and live spot price is used for signal derivation. Price and index history not yet integrated in signal calulcations.

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
    pub trade_type: String,
    pub include_price_data: bool,
    pub include_index_data: bool,
    pub interval: Duration,
}
```
Modify these values in the BotConfig struct to adjust the bot’s trading parameters.

## Development Status

This bot is currently under active development. The core trading logic is operational, but there are additional features and improvements planned, including better error handling, enhanced configuration options, and more robust trade management.

🚨 Disclaimer: This bot is not yet refined or suitable for production use. It is a raw development version, and you should use it at your own risk, especially with real funds.

## Contributing

🤝 Contributions are welcome! If you would like to improve the bot or add new features, please follow the steps below:

Fork the repository.
Create a new branch (git checkout -b feature/your-feature).
Make your changes and commit them (git commit -am 'Add new feature').
Push to the branch (git push origin feature/your-feature).
Open a pull request.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.

## Acknowledgments

LN Markets API: The bot integrates with the LN Markets API for fetching price data and executing trades. For more information, please refer to the official documentation.
Rust Community: Special thanks to the Rust community for providing the tools and libraries that make this bot possible.

## Warning

⚠️ This bot is a work-in-progress and should be used with caution. It is intended for learning and experimentation, and you should not use it in a live environment with real funds unless you fully understand the code and its behavior.