
<div align="center">

# 🚀 binance-rs-plus

[![Fork Origin](https://img.shields.io/badge/Origin-wisespace__io/binance__rs-6f42c1?style=flat&logo=git)](https://github.com/wisespace-io/binance-rs-plus)

> 📢 **Derivative Notice**: This project is derived from [binance-rs](https://github.com/wisespace-io/binance-rs-plus),  
> with acknowledgments to the original author [@wisespace-io](https://github.com/wisespace-io) for their contribution

</div>

Unofficial Rust Library for the [Binance API](https://github.com/binance/binance-spot-api-docs) and [Binance Futures API](https://binance-docs.github.io/apidocs/futures/en/#general-info).

[![English](https://img.shields.io/badge/Language-English-blue?style=flat-square)](README.md)
[![中文](https://img.shields.io/badge/语言-中文-red?style=flat-square)](README-CN.md)
[![Crates.io](https://img.shields.io/crates/v/binance-rs-plus.svg)](https://crates.io/crates/binance-rs-plus)
[![Build Status](https://travis-ci.org/wisespace-io/binance-rs-plus.png?branch=master)](https://travis-ci.org/wisespace-io/binance-rs-plus)
[![CI](https://github.com/wisespace-io/binance-rs-plus/workflows/Rust/badge.svg)](https://github.com/wisespace-io/binance-rs-plus/actions?query=workflow%3ARust)
[![MIT licensed](https://img.shields.io/badge/License-MIT-blue.svg)](./LICENSE-MIT)
[![Apache-2.0 licensed](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](./LICENSE-APACHE)
[![Forked from](https://img.shields.io/badge/Forked%20from-wisespace--io/binance--rs-blue?style=flat-square&logo=github)](https://github.com/wisespace-io/binance-rs-plus)

[Documentation on docs.rs](https://docs.rs/crate/binance-rs-plus/)

## Risk Warning

It is a personal project, use at your own risk. I will not be responsible for your investment losses.
Cryptocurrency investment is subject to high market risk.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
binance-rs-plus = "0.1.0" # Or the latest version from crates.io
# For using the git version:
# binance-rs-plus = { git = "https://github.com/Praying/binance-rs-plus" }
tokio = { version = "1", features = ["full"] }
anyhow = "1.0"
```

## Rust >= 1.84.0 (Edition 2024)

```sh
rustup update stable
```

## Table of Contents
- [SPOT API](#spot-api)
  - [MARKET DATA (SPOT)](#market-data-spot)
  - [ACCOUNT (SPOT)](#account-spot)
  - [USER STREAM (SPOT)](#user-stream-spot)
  - [WEBSOCKETS (SPOT)](#websockets-spot)
    - [USER STREAM DATA (SPOT)](#user-stream-data-spot)
    - [TRADES (SPOT)](#trades-spot)
    - [KLINE (SPOT)](#kline-spot)
    - [MULTIPLE STREAMS (SPOT)](#multiple-streams-spot)
- [FUTURES API](#futures-api)
  - [GENERAL (FUTURES)](#general-futures)
  - [MARKET DATA (FUTURES)](#market-data-futures)
  - [USER STREAM (FUTURES)](#user-stream-futures)
  - [WEBSOCKETS (FUTURES)](#websockets-futures)
- [ERROR HANDLING](#error-handling)
- [TESTNET AND API CLUSTERS](#testnet-and-api-clusters)
- [OTHER EXCHANGES](#other-exchanges)

## SPOT API

### MARKET DATA (SPOT)

```rust
use binance_rs_plus::api::*;
use binance_rs_plus::model::*;
use binance_rs_plus::market::*;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let market: Market = Binance::new(None, None);

    // Order book at default depth
    match market.get_depth("BNBETH").await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Order book at depth 500
    match market.get_custom_depth("BNBETH", 500).await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Latest price for ALL symbols
    match market.get_all_prices().await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }

    // Latest price for ONE symbol
    match market.get_price("BNBETH").await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }

    // Current average price for ONE symbol
    match market.get_average_price("BNBETH").await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }

    // Best price/qty on the order book for ALL symbols
    match market.get_all_book_tickers().await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }

    // Best price/qty on the order book for ONE symbol
    match market.get_book_ticker("BNBETH").await {
        Ok(answer) => println!(
            "Bid Price: {}, Ask Price: {}",
            answer.bid_price, answer.ask_price
        ),
        Err(e) => println!("Error: {:?}", e),
    }

    // 24hr ticker price change statistics
    match market.get_24h_price_stats("BNBETH").await {
        Ok(answer) => println!(
            "Open Price: {}, Higher Price: {}, Lower Price: {:?}",
            answer.open_price, answer.high_price, answer.low_price
        ),
        Err(e) => println!("Error: {:?}", e),
    }

    // last 10 5min klines (candlesticks) for a symbol:
    match market.get_klines("BNBETH", "5m", 10, None, None).await {
        Ok(klines) => {   
            match klines {
                binance_rs_plus::model::KlineSummaries::AllKlineSummaries(klines) => {
                    if let Some(kline) = klines.first() { // Example: Print first kline
                        println!(
                            "Open: {}, High: {}, Low: {}",
                            kline.open, kline.high, kline.low
                        );
                    }
                }
            }
        },
        Err(e) => println!("Error: {}", e),
    }
    Ok(())
}
```

### ACCOUNT (SPOT)

```rust
use binance_rs_plus::api::*;
use binance_rs_plus::account::*;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = Some("YOUR_API_KEY".into());
    let secret_key = Some("YOUR_SECRET_KEY".into());

    let account: Account = Binance::new(api_key, secret_key);

    match account.get_account().await {
        Ok(answer) => println!("{:?}", answer.balances),
        Err(e) => println!("Error: {:?}", e),
    }

    match account.get_open_orders("WTCETH").await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }

    match account.limit_buy("WTCETH", 10.0, 0.014000).await { // Quantity type updated to f64
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }

    match account.market_buy("WTCETH", 5.0).await { // Quantity type updated to f64
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }

    match account.limit_sell("WTCETH", 10.0, 0.035000).await { // Quantity type updated to f64
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }

    match account.market_sell("WTCETH", 5.0).await { // Quantity type updated to f64
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }
    
    // custom_order example was using i64 for quantity, ensure it's f64 if that's the new standard
    match account.custom_order("WTCETH", 9999.0, 0.0123, "SELL", "LIMIT", "IOC", None, None, None, None, None).await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }

    let order_id = 1_957_528;
    match account.order_status("WTCETH", order_id).await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }

    match account.cancel_order("WTCETH", order_id).await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }

    match account.cancel_all_open_orders("WTCETH").await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }

    match account.get_balance("KNC").await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }

    match account.trade_history("WTCETH").await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }
    Ok(())
}
```

### USER STREAM (SPOT)

```rust
use binance_rs_plus::api::*;
use binance_rs_plus::userstream::*;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let api_key_user = Some("YOUR_API_KEY".into());
    let user_stream: UserStream = Binance::new(api_key_user.clone(), None);

    if let Ok(answer) = user_stream.start().await {
        println!("Data Stream Started ...");
        let listen_key = answer.listen_key;
        println!("Listen Key: {}", listen_key);

        match user_stream.keep_alive(&listen_key).await {
            Ok(msg) => println!("Keepalive user data stream: {:?}", msg),
            Err(e) => println!("Error: {:?}", e),
        }

        // Example: Close stream after some time or action
        // tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;

        match user_stream.close(&listen_key).await {
            Ok(msg) => println!("Close user data stream: {:?}", msg),
            Err(e) => println!("Error: {:?}", e),
        }
    } else {
        println!("Not able to start an User Stream (Check your API_KEY)");
    }
    Ok(())
}
```

### WEBSOCKETS (SPOT)

#### USER STREAM DATA (SPOT)

Listen to account updates and order trades.

```rust
use binance_rs_plus::api::*;
use binance_rs_plus::userstream::*;
use binance_rs_plus::websockets::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use anyhow::Result;
use std::pin::Pin;
use std::future::Future;
use binance_rs_plus::errors::Result as BinanceResult;


#[tokio::main]
async fn main() -> Result<()> {
    let api_key_user = Some("YOUR_API_KEY".into()); // IMPORTANT: Substitute with your API key
    let user_stream: UserStream = Binance::new(api_key_user, None);

    match user_stream.start().await {
        Ok(answer) => {
            let listen_key = answer.listen_key;
            println!("Listen Key: {}", listen_key);

            let keep_running = Arc::new(AtomicBool::new(true));
            let kr_clone = keep_running.clone();

            let mut web_socket: WebSockets<'_,_> = WebSockets::new(move |event: WebsocketEvent| -> Pin<Box<dyn Future<Output = BinanceResult<()>> + Send + 'static>> {
                let kr_handler_clone = kr_clone.clone();
                Box::pin(async move {
                    match event {
                        WebsocketEvent::AccountUpdate(account_update) => {
                            println!("Account Update: {:?}", account_update.data);
                            // Example: Stop after first account update
                            // kr_handler_clone.store(false, Ordering::Relaxed); 
                        },
                        WebsocketEvent::OrderTrade(trade) => {
                            println!("Order Trade: {:?}", trade);
                        },
                        WebsocketEvent::OcoOrderTrade(oco_trade) => {
                            println!("OCO Order Trade: {:?}", oco_trade);
                        }
                        _ => println!("Unhandled event: {:?}", event),
                    };
                    Ok(())
                })
            });
            
            if let Err(e) = web_socket.connect(&listen_key).await { // User Stream endpoint
                println!("Error connecting to Spot User Stream: {:?}", e);
                return Ok(());
            }

            if let Err(e) = web_socket.event_loop(keep_running.clone()).await {
                println!("Error in Spot User Stream event loop: {:?}", e);
            }
            
            web_socket.disconnect().await.unwrap();
            println!("Spot User Stream Disconnected");

        }
        Err(e) => println!("Could not start Spot User Stream: {:?}", e),
    }
    Ok(())
}
```

#### TRADES (SPOT)

Listen to individual trades.

```rust
use binance_rs_plus::websockets::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use anyhow::Result;
use std::pin::Pin;
use std::future::Future;
use binance_rs_plus::errors::Result as BinanceResult;

#[tokio::main]
async fn main() -> Result<()> {
    let keep_running = Arc::new(AtomicBool::new(true));
    let kr_clone = keep_running.clone();
    let trade_symbol = "bnbbtc"; // Example symbol
    let stream_name = format!("{}@trade", trade_symbol);

    let mut web_socket: WebSockets<'_,_> = WebSockets::new(move |event: WebsocketEvent| -> Pin<Box<dyn Future<Output = BinanceResult<()>> + Send + 'static>> {
        let kr_handler_clone = kr_clone.clone();
        Box::pin(async move {
            match event {
                WebsocketEvent::Trade(trade_event) => {
                    println!("Symbol: {}, Price: {}, Qty: {}", trade_event.symbol, trade_event.price, trade_event.qty);
                    // kr_handler_clone.store(false, Ordering::Relaxed); // Stop after first trade
                }
                _ => (),
            };
            Ok(())
        })
    });

    if let Err(e) = web_socket.connect(&stream_name).await {
        println!("Error connecting to {} stream: {:?}", stream_name, e);
        return Ok(());
    }
    if let Err(e) = web_socket.event_loop(keep_running.clone()).await {
        println!("Error in event loop: {:?}", e);
    }
    web_socket.disconnect().await.unwrap();
    println!("Disconnected from {} stream", stream_name);
    Ok(())
}
```

#### KLINE (SPOT)

Listen to kline (candlestick) data.

```rust
use binance_rs_plus::websockets::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use anyhow::Result;
use std::pin::Pin;
use std::future::Future;
use binance_rs_plus::errors::Result as BinanceResult;

#[tokio::main]
async fn main() -> Result<()> {
    let keep_running = Arc::new(AtomicBool::new(true));
    let kr_clone = keep_running.clone();
    let symbol = "ethbtc";
    let interval = "1m";
    let stream_name = format!("{}@kline_{}", symbol, interval);

    let mut web_socket: WebSockets<'_,_> = WebSockets::new(move |event: WebsocketEvent| -> Pin<Box<dyn Future<Output = BinanceResult<()>> + Send + 'static>> {
        let kr_handler_clone = kr_clone.clone();
        Box::pin(async move {
            if let WebsocketEvent::Kline(kline_event) = event {
                println!("Symbol: {}, Interval: {}, Open: {}, Close: {}, High: {}, Low: {}", 
                    kline_event.kline.symbol, kline_event.kline.interval, kline_event.kline.open, 
                    kline_event.kline.close, kline_event.kline.high, kline_event.kline.low);
                // kr_handler_clone.store(false, Ordering::Relaxed); // Stop after first kline
            }
            Ok(())
        })
    });

    if let Err(e) = web_socket.connect(&stream_name).await {
        println!("Error connecting to {} stream: {:?}", stream_name, e);
        return Ok(());
    }
    if let Err(e) = web_socket.event_loop(keep_running.clone()).await {
        println!("Error in event loop: {:?}", e);
    }
    web_socket.disconnect().await.unwrap();
    println!("Disconnected from {} stream", stream_name);
    Ok(())
}
```

#### MULTIPLE STREAMS (SPOT)

Connect to multiple streams simultaneously.

```rust
use binance_rs_plus::websockets::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use anyhow::Result;
use std::pin::Pin;
use std::future::Future;
use binance_rs_plus::errors::Result as BinanceResult;

#[tokio::main]
async fn main() -> Result<()> {
    let streams = vec![
        "ethbtc@kline_1m".to_string(),
        "bnbbtc@depth@100ms".to_string(),
        "ltcbtc@trade".to_string(),
    ];

    let keep_running = Arc::new(AtomicBool::new(true));
    let kr_clone = keep_running.clone();

    let mut web_socket: WebSockets<'_,_> = WebSockets::new(move |event: WebsocketEvent| -> Pin<Box<dyn Future<Output = BinanceResult<()>> + Send + 'static>> {
        let _kr_handler_clone = kr_clone.clone(); // Use if you want to stop based on an event
        Box::pin(async move {
            println!("Received event: {:?}", event);
            // Example: Stop after receiving any 3 events for demonstration.
            // static COUNTER: AtomicUsize = AtomicUsize::new(0);
            // if COUNTER.fetch_add(1, Ordering::Relaxed) >= 2 {
            //     _kr_handler_clone.store(false, Ordering::Relaxed);
            // }
            Ok(())
        })
    });

    if let Err(e) = web_socket.connect_multiple_streams(&streams).await {
         println!("Error connecting to multiple streams: {:?}", e);
        return Ok(());
    }
    
    if let Err(e) = web_socket.event_loop(keep_running.clone()).await {
        println!("Error in event loop: {:?}", e);
    }
    web_socket.disconnect().await.unwrap();
    println!("Disconnected from multiple streams");
    Ok(())
}
```

## FUTURES API

The Futures API client allows interaction with both USD-M (USDT Margined) and COIN-M (Coin Margined) futures.

### GENERAL (FUTURES)

General endpoints for Binance Futures.

```rust
use binance_rs_plus::api::*;
use binance_rs_plus::futures::general::*;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let general: FuturesGeneral = Binance::new(None, None);

    // Ping USD-M Futures
    match general.ping().await { // Default is USD-M
        Ok(answer) => println!("Futures Ping (USD-M) successful: {:?}", answer),
        Err(err) => println!("Error pinging USD-M futures: {:?}", err),
    }
    
    // Ping COIN-M Futures
    match general.ping_coin_m().await {
        Ok(answer) => println!("Futures Ping (COIN-M) successful: {:?}", answer),
        Err(err) => println!("Error pinging COIN-M futures: {:?}", err),
    }

    // Server Time (USD-M)
    match general.get_server_time().await { // Default is USD-M
        Ok(answer) => println!("Futures Server Time (USD-M): {}", answer.server_time),
        Err(e) => println!("Error getting USD-M futures server time: {:?}", e),
    }
    
    // Server Time (COIN-M)
    match general.get_server_time_coin_m().await {
        Ok(answer) => println!("Futures Server Time (COIN-M): {}", answer.server_time),
        Err(e) => println!("Error getting COIN-M futures server time: {:?}", e),
    }

    // Exchange Info (USD-M)
    match general.exchange_info().await { // Default is USD-M
        Ok(answer) => println!("Futures Exchange Information (USD-M): {} symbols", answer.symbols.len()),
        Err(e) => println!("Error getting USD-M futures exchange info: {:?}", e),
    }

    // Exchange Info (COIN-M)
    match general.exchange_info_coin_m().await {
        Ok(answer) => println!("Futures Exchange Information (COIN-M): {} symbols", answer.symbols.len()),
        Err(e) => println!("Error getting COIN-M futures exchange info: {:?}", e),
    }
    Ok(())
}
```

### MARKET DATA (FUTURES)

Access market data for futures.

```rust
use binance::api::*;
use binance::futures::market::*;
use binance::futures::model::*; // For enums like Trades, KlineSummaries etc.
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let market: FuturesMarket = Binance::new(None, None);

    // USD-M Futures Example (Default)
    let symbol_usdm = "BTCUSDT";
    println!("--- USD-M Futures for {} ---", symbol_usdm);

    match market.get_depth(symbol_usdm).await { // Default is USD-M
        Ok(answer) => println!("Depth (USD-M) update ID: {:?}", answer.last_update_id),
        Err(e) => println!("Error getting USD-M depth: {:?}", e),
    }

    match market.get_klines(symbol_usdm, "5m", 10, None, None).await { // Default is USD-M
        Ok(KlineSummaries::AllKlineSummaries(answer)) => {
            if let Some(kline) = answer.first() {
                println!("First kline (USD-M): {:?}", kline);
            }
        },
        Err(e) => println!("Error getting USD-M klines: {:?}", e),
    }

    // COIN-M Futures Example
    let symbol_coinm = "BTCUSD_PERP"; // Or a specific delivery contract like BTCUSD_240927
    println!("\n--- COIN-M Futures for {} ---", symbol_coinm);

    match market.get_depth_coin_m(symbol_coinm).await {
        Ok(answer) => println!("Depth (COIN-M) update ID: {:?}", answer.last_update_id),
        Err(e) => println!("Error getting COIN-M depth: {:?}", e),
    }

    match market.get_klines_coin_m(symbol_coinm, "15m", 5, None, None).await {
         Ok(KlineSummaries::AllKlineSummaries(answer)) => {
            if let Some(kline) = answer.first() {
                println!("First kline (COIN-M): {:?}", kline);
            }
        },
        Err(e) => println!("Error getting COIN-M klines: {:?}", e),
    }
    
    // Other market data endpoints (showing USD-M, add _coin_m suffix for COIN-M versions)
    // e.g., get_trades, get_agg_trades, get_24h_price_stats, get_price, get_all_book_tickers, etc.
    // Example:
    match market.get_price(symbol_usdm).await {
        Ok(price_ticker) => println!("Price for {} (USD-M): {}", symbol_usdm, price_ticker.price),
        Err(e) =>  println!("Error: {:?}", e)
    }
    
    match market.get_price_coin_m(symbol_coinm).await {
        Ok(price_ticker) => println!("Price for {} (COIN-M): {}", symbol_coinm, price_ticker.price),
        Err(e) =>  println!("Error: {:?}", e)
    }

    Ok(())
}
```

### USER STREAM (FUTURES)

Manage user data streams for futures. API Key required.

```rust
use binance::api::*;
use binance::futures::userstream::*;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = Some("YOUR_FUTURES_API_KEY".into());
    // Secret key might be needed for some user stream related operations if Binance changes requirements.
    // For start/keep_alive/close, often only API key with correct permissions is needed.
    let user_stream: FuturesUserStream = Binance::new(api_key, None); 

    // USD-M User Stream
    println!("--- USD-M Futures User Stream ---");
    match user_stream.start().await { // Default is USD-M
        Ok(answer) => {
            println!("USD-M Data Stream Started. Listen Key: {}", answer.listen_key);
            let listen_key = answer.listen_key;

            match user_stream.keep_alive(&listen_key).await {
                Ok(msg) => println!("Keepalive USD-M stream: {:?}", msg),
                Err(e) => println!("Error keeping USD-M stream alive: {:?}", e),
            }
            // tokio::time::sleep(tokio::time::Duration::from_secs(10)).await; // Keep stream open
            match user_stream.close(&listen_key).await {
                Ok(msg) => println!("Closed USD-M stream: {:?}", msg),
                Err(e) => println!("Error closing USD-M stream: {:?}", e),
            }
        }
        Err(e) => println!("Could not start USD-M User Stream: {:?}", e),
    }

    // COIN-M User Stream
    println!("\n--- COIN-M Futures User Stream ---");
    match user_stream.start_coin_m().await {
        Ok(answer) => {
            println!("COIN-M Data Stream Started. Listen Key: {}", answer.listen_key);
            let listen_key_coinm = answer.listen_key;

            match user_stream.keep_alive_coin_m(&listen_key_coinm).await {
                Ok(msg) => println!("Keepalive COIN-M stream: {:?}", msg),
                Err(e) => println!("Error keeping COIN-M stream alive: {:?}", e),
            }
            // tokio::time::sleep(tokio::time::Duration::from_secs(10)).await; // Keep stream open
            match user_stream.close_coin_m(&listen_key_coinm).await {
                Ok(msg) => println!("Closed COIN-M stream: {:?}", msg),
                Err(e) => println!("Error closing COIN-M stream: {:?}", e),
            }
        }
        Err(e) => println!("Could not start COIN-M User Stream: {:?}", e),
    }
    Ok(())
}
```

### WEBSOCKETS (FUTURES)

Connect to market data WebSocket streams for USD-M and COIN-M futures.
For User Data Streams (account updates, order updates), use the `listen_key` obtained from the `FuturesUserStream` `start()` or `start_coin_m()` methods and connect to the respective USD-M or COIN-M user data WebSocket endpoint.

```rust
use binance::futures::websockets::{FuturesWebSockets, FuturesWebsocketEvent, FuturesMarket};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use anyhow::Result;
use std::pin::Pin;
use std::future::Future;
use binance::errors::Result as BinanceResult; // To avoid conflict with anyhow::Result

#[tokio::main]
async fn main() -> Result<()> {
    // USD-M Market Stream Example
    let usdm_symbol = "btcusdt";
    let usdm_stream_name = format!("{}@kline_1m", usdm_symbol); // Example: 1 minute klines for BTCUSDT
    println!("Connecting to USD-M stream: {}", usdm_stream_name);

    let keep_running_usdm = Arc::new(AtomicBool::new(true));
    let kr_usdm_clone = Arc::clone(&keep_running_usdm);

    let mut ws_usdm: FuturesWebSockets<'_,_> = FuturesWebSockets::new(move |event: FuturesWebsocketEvent| -> Pin<Box<dyn Future<Output = BinanceResult<()>> + Send + 'static>> {
        let kr_handler_clone = kr_usdm_clone.clone();
        Box::pin(async move {
            println!("USD-M Event: {:?}", event);
            // kr_handler_clone.store(false, Ordering::Relaxed); // Stop after first event for example
            Ok(())
        })
    });

    if let Err(e) = ws_usdm.connect(&FuturesMarket::USDM, &usdm_stream_name).await {
        eprintln!("Failed to connect to USD-M stream {}: {:?}", usdm_stream_name, e);
    } else {
        if let Err(e) = ws_usdm.event_loop(keep_running_usdm).await {
            eprintln!("Error in USD-M event loop for {}: {:?}", usdm_stream_name, e);
        }
        ws_usdm.disconnect().await.unwrap_or_else(|e| eprintln!("Failed to disconnect USD-M: {:?}", e));
        println!("Disconnected from USD-M stream: {}", usdm_stream_name);
    }
    
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await; // Brief pause

    // COIN-M Market Stream Example
    let coinm_symbol = "btcusd_perp";
    let coinm_stream_name = format!("{}@aggTrade", coinm_symbol); // Example: Aggregate Trades for BTCUSD_PERP
    println!("\nConnecting to COIN-M stream: {}", coinm_stream_name);
    
    let keep_running_coinm = Arc::new(AtomicBool::new(true));
    let kr_coinm_clone = Arc::clone(&keep_running_coinm);

    let mut ws_coinm: FuturesWebSockets<'_,_> = FuturesWebSockets::new(move |event: FuturesWebsocketEvent| -> Pin<Box<dyn Future<Output = BinanceResult<()>> + Send + 'static>> {
        let kr_handler_clone = kr_coinm_clone.clone();
        Box::pin(async move {
            println!("COIN-M Event: {:?}", event);
            // kr_handler_clone.store(false, Ordering::Relaxed); // Stop after first event for example
            Ok(())
        })
    });

    if let Err(e) = ws_coinm.connect(&FuturesMarket::COINM, &coinm_stream_name).await {
        eprintln!("Failed to connect to COIN-M stream {}: {:?}", coinm_stream_name, e);
    } else {
        if let Err(e) = ws_coinm.event_loop(keep_running_coinm).await {
             eprintln!("Error in COIN-M event loop for {}: {:?}", coinm_stream_name, e);
        }
        ws_coinm.disconnect().await.unwrap_or_else(|e| eprintln!("Failed to disconnect COIN-M: {:?}", e));
        println!("Disconnected from COIN-M stream: {}", coinm_stream_name);
    }

    // For User Data Streams (Futures):
    // 1. Obtain listen_key using `FuturesUserStream::start()` (for USD-M) or `FuturesUserStream::start_coin_m()` (for COIN-M).
    // 2. Connect using `FuturesWebSockets::connect()` with `FuturesMarket::USDMUserData(&listen_key)` or `FuturesMarket::COINMUserData(&listen_key)`.
    // Example (conceptual, ensure you have a valid listen_key):
    // let listen_key_example = "your_futures_listen_key";
    // if let Err(e) = ws_usdm.connect(&FuturesMarket::USDMUserData(listen_key_example), "").await { ... } 
    // The stream name parameter for user data streams is often ignored or should be an empty string, 
    // as the listen_key itself defines the stream. Check Binance documentation for specifics if needed.

    Ok(())
}
```

## ERROR HANDLING

Provides more detailed error information. The library uses `anyhow::Result` for most fallible operations and `binance::errors::ErrorKind` for specific Binance API errors.

You can check out the [Binance Error Codes](https://github.com/binance-exchange/binance-official-api-docs/blob/master/errors.md) and [Futures Error Codes](https://binance-docs.github.io/apidocs/futures/en/#error-codes).

```rust
use binance::errors::{Error as BinanceError, ErrorKind as BinanceLibErrorKind}; // Assuming ErrorKind is part of BinanceError
use anyhow::Result;

async fn example_error_handling() -> Result<()> {
    // ... some binance client call ...
    // let result = account.get_account().await;
    // match result {
    //     Ok(data) => { /* ... */ }
    //     Err(e) => {
    //         println!("An error occurred: {}", e); // General anyhow error
    //
    //         // To get more specific Binance error details:
    //         if let Some(binance_err) = e.downcast_ref::<BinanceError>() {
    //             match binance_err.kind() { // Assuming kind() method exists on your BinanceError
    //                 BinanceLibErrorKind::BinanceError(response) => {
    //                     println!("Binance API Error Code: {}, Msg: {}", response.code, response.msg);
    //                     match response.code {
    //                         -1013i32 => println!("Filter failure: LOT_SIZE!"), // Note: codes might be i32 or other types
    //                         -2010i32 => println!("Funds insufficient! {}", response.msg),
    //                         _ => println!("Non-catched code {}: {}", response.code, response.msg),
    //                     }
    //                 }
    //                 BinanceLibErrorKind::Msg(msg) => {
    //                     println!("Binance library internal error msg: {}", msg);
    //                 }
    //                 _ => println!("Other Binance library error: {:?}", binance_err.kind()),
    //             }
    //         }
    //     }
    // }
    Ok(()) // Placeholder
}
```
*Note: The exact structure of `BinanceError` and `ErrorKind` should be checked against `src/errors.rs` for precise handling.*

## TESTNET AND API CLUSTERS

You can overwrite the default Binance API URLs if there are performance issues or if you want to use Testnet.
The `Config` struct can be used to specify custom endpoints for Spot, USD-M Futures, and COIN-M Futures.

[Binance Spot API Clusters](https://github.com/binance/binance-spot-api-docs/blob/master/rest-api.md#general-api-information)
[Binance Futures API Endpoints](https://binance-docs.github.io/apidocs/futures/en/#endpoint-information) (includes Testnet)

```rust
use binance::api::*;
use binance::config::Config;
use binance::market::Market; // For spot
use binance::futures::general::FuturesGeneral; // For futures

fn main() {
    let use_spot_testnet = true;
    let use_futures_testnet = true;

    // Spot Testnet Configuration
    if use_spot_testnet {
        let config_spot = Config::default()
            .set_rest_api_endpoint("https://testnet.binance.vision")
            .set_ws_endpoint("wss://testnet.binance.vision/ws"); // For Spot WebSockets

        let market_spot_testnet: Market = Binance::new_with_config(None, None, &config_spot);
        // Now use market_spot_testnet for Spot Testnet calls
        println!("Using Spot Testnet: {}", config_spot.rest_api_endpoint);
    }

    // Futures Testnet Configuration (USD-M)
    if use_futures_testnet {
        let config_futures_usdm = Config::default()
            .set_futures_rest_api_endpoint(binance::config::FuturesEndpoint::UsdMFuturesTestnet) // Or custom string
            .set_futures_ws_endpoint(binance::config::FuturesEndpoint::UsdMFuturesTestnet);     // Or custom string for WS

        let general_futures_usdm_testnet: FuturesGeneral = Binance::new_with_config(None, None, &config_futures_usdm);
        // Now use general_futures_usdm_testnet for USD-M Futures Testnet calls
        println!("Using USD-M Futures Testnet REST: {}", config_futures_usdm.futures_rest_api_endpoint);
        println!("Using USD-M Futures Testnet WS: {}", config_futures_usdm.futures_ws_endpoint);

        // For COIN-M Futures Testnet
        let config_futures_coinm = Config::default()
            .set_futures_coin_m_rest_api_endpoint(binance::config::FuturesEndpoint::CoinMFuturesTestnet)
            .set_futures_coin_m_ws_endpoint(binance::config::FuturesEndpoint::CoinMFuturesTestnet);
        
        let general_futures_coinm_testnet: FuturesGeneral = Binance::new_with_config(None, None, &config_futures_coinm);
         println!("Using COIN-M Futures Testnet REST: {}", config_futures_coinm.futures_coin_m_rest_api_endpoint);
         println!("Using COIN-M Futures Testnet WS: {}", config_futures_coinm.futures_coin_m_ws_endpoint);

    }
}
```

