<div align="center">

# 🚀  binance-rs-plus 

[![Fork Relationship](https://img.shields.io/badge/Origin-wisespace__io/binance__rs-6f42c1?style=flat&logo=git)](https://github.com/wisespace-io/binance-rs-plus)

> 📢 **衍生声明**：此项目衍生自 [binance-rs](https://github.com/wisespace-io/binance-rs)，  
> 感谢原始作者 [@wisespace-io](https://github.com/wisespace-io) 的贡献

</div>

一个非官方的 Rust 库，用于 [Binance API](https://github.com/binance/binance-spot-api-docs) (币安现货 API) 和 [Binance Futures API](https://binance-docs.github.io/apidocs/futures/en/#general-info) (币安合约 API)。

[![Crates.io](https://img.shields.io/crates/v/binance-rs-plus.svg)](https://crates.io/crates/binance-rs-plus)
[![Build Status](https://travis-ci.org/wisespace-io/binance-rs-plus.png?branch=master)](https://travis-ci.org/wisespace-io/binance-rs-plus)
[![CI](https://github.com/Praying/binance-rs-plus/workflows/Rust/badge.svg)](https://github.com/Praying/binance-rs-plus/actions?query=workflow%3ARust)
[![MIT licensed](https://img.shields.io/badge/License-MIT-blue.svg)](./LICENSE-MIT)
[![Apache-2.0 licensed](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](./LICENSE-APACHE)

[docs.rs 上的文档](https://docs.rs/crate/binance-rs-plus/)

## 风险提示

这是一个个人项目，请自行承担风险。我不会对您的投资损失负责。
加密货币投资存在较高的市场风险。

## 如何使用

将以下内容添加到您的 `Cargo.toml` 文件中：

```toml
[dependencies]
binance-rs-plus = "0.1.1" # 或者 crates.io 上的最新版本
# binance-rs-plus = { git = "https://github.com/Praying/binance-rs-plus" }
tokio = { version = "1", features = ["full"] }
anyhow = "1.0"
```

## Rust >= 1.84.0 (Edition 2024)

```sh
rustup update stable
```

## 目录
- [现货 API](#现货-api)
  - [市场数据 (现货)](#市场数据-现货)
  - [账户 (现货)](#账户-现货)
  - [用户数据流 (现货)](#用户数据流-现货)
  - [WEBSOCKETS (现货)](#websockets-现货)
    - [用户数据流数据 (现货)](#用户数据流数据-现货)
    - [成交 (现货)](#成交-现货)
    - [K线 (现货)](#k线-现货)
    - [多流订阅 (现货)](#多流订阅-现货)
- [合约 API](#合约-api)
  - [通用 (合约)](#通用-合约)
  - [市场数据 (合约)](#市场数据-合约)
  - [用户数据流 (合约)](#用户数据流-合约)
  - [WEBSOCKETS (合约)](#websockets-合约)
- [错误处理](#错误处理)
- [测试网和API集群](#测试网和api集群)
- [其他交易所](#其他交易所)

## 现货 API

### 市场数据 (现货)

```rust
use binance_rs_plus::api::*;
use binance_rs_plus::model::*;
use binance_rs_plus::market::*;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let market: Market = Binance::new(None, None);

    // 获取默认深度的订单簿
    match market.get_depth("BNBETH").await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("错误: {}", e),
    }

    // 获取深度为 500 的订单簿
    match market.get_custom_depth("BNBETH", 500).await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("错误: {}", e),
    }

    // 获取所有交易对的最新价格
    match market.get_all_prices().await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("错误: {:?}", e),
    }

    // 获取单个交易对的最新价格
    match market.get_price("BNBETH").await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("错误: {:?}", e),
    }

    // 获取单个交易对的当前平均价格
    match market.get_average_price("BNBETH").await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("错误: {:?}", e),
    }

    // 获取所有交易对订单簿上的最优买卖价/量
    match market.get_all_book_tickers().await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("错误: {:?}", e),
    }

    // 获取单个交易对订单簿上的最优买卖价/量
    match market.get_book_ticker("BNBETH").await {
        Ok(answer) => println!(
            "买单价: {}, 卖单价: {}",
            answer.bid_price, answer.ask_price
        ),
        Err(e) => println!("错误: {:?}", e),
    }

    // 24小时价格变动统计
    match market.get_24h_price_stats("BNBETH").await {
        Ok(answer) => println!(
            "开盘价: {}, 最高价: {}, 最低价: {:?}",
            answer.open_price, answer.high_price, answer.low_price
        ),
        Err(e) => println!("错误: {:?}", e),
    }

    // 获取某个交易对最近10条5分钟K线数据:
    match market.get_klines("BNBETH", "5m", 10, None, None).await {
        Ok(klines) => {   
            match klines {
                binance_rs_plus::model::KlineSummaries::AllKlineSummaries(klines) => {
                    if let Some(kline) = klines.first() { // 示例：打印第一条K线
                        println!(
                            "开盘价: {}, 最高价: {}, 最低价: {}",
                            kline.open, kline.high, kline.low
                        );
                    }
                }
            }
        },
        Err(e) => println!("错误: {}", e),
    }
    Ok(())
}
```

### 账户 (现货)

```rust
use binance_rs_plus::api::*;
use binance_rs_plus::account::*;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = Some("YOUR_API_KEY".into()); // 替换为你的API Key
    let secret_key = Some("YOUR_SECRET_KEY".into()); // 替换为你的Secret Key

    let account: Account = Binance::new(api_key, secret_key);

    // 获取账户信息
    match account.get_account().await {
        Ok(answer) => println!("{:?}", answer.balances),
        Err(e) => println!("错误: {:?}", e),
    }

    // 获取某个交易对的当前挂单
    match account.get_open_orders("WTCETH").await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("错误: {:?}", e),
    }

    // 限价买入 (数量类型已更新为 f64)
    match account.limit_buy("WTCETH", 10.0, 0.014000).await { 
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("错误: {:?}", e),
    }

    // 市价买入 (数量类型已更新为 f64)
    match account.market_buy("WTCETH", 5.0).await { 
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("错误: {:?}", e),
    }

    // 限价卖出 (数量类型已更新为 f64)
    match account.limit_sell("WTCETH", 10.0, 0.035000).await { 
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("错误: {:?}", e),
    }

    // 市价卖出 (数量类型已更新为 f64)
    match account.market_sell("WTCETH", 5.0).await { 
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("错误: {:?}", e),
    }
    
    // 自定义订单 (custom_order 的数量参数之前是 i64, 如果新标准是 f64 请确保类型正确)
    match account.custom_order("WTCETH", 9999.0, 0.0123, "SELL", "LIMIT", "IOC", None, None, None, None, None).await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("错误: {:?}", e),
    }

    let order_id = 1_957_528;
    // 查询订单状态
    match account.order_status("WTCETH", order_id).await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("错误: {:?}", e),
    }

    // 取消订单
    match account.cancel_order("WTCETH", order_id).await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("错误: {:?}", e),
    }

    // 取消某个交易对的所有挂单
    match account.cancel_all_open_orders("WTCETH").await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("错误: {:?}", e),
    }

    // 获取特定资产的余额
    match account.get_balance("KNC").await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("错误: {:?}", e),
    }

    // 获取某个交易对的成交历史
    match account.trade_history("WTCETH").await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("错误: {:?}", e),
    }
    Ok(())
}
```

### 用户数据流 (现货)

```rust
use binance_rs_plus::api::*;
use binance_rs_plus::userstream::*;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let api_key_user = Some("YOUR_API_KEY".into()); // 替换为你的API Key
    let user_stream: UserStream = Binance::new(api_key_user.clone(), None);

    // 开启用户数据流
    if let Ok(answer) = user_stream.start().await {
        println!("用户数据流已启动 ...");
        let listen_key = answer.listen_key;
        println!("Listen Key: {}", listen_key);

        // 保持用户数据流连接
        match user_stream.keep_alive(&listen_key).await {
            Ok(msg) => println!("保持用户数据流连接: {:?}", msg),
            Err(e) => println!("错误: {:?}", e),
        }

        // 示例：在一段时间或操作后关闭数据流
        // tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;

        // 关闭用户数据流
        match user_stream.close(&listen_key).await {
            Ok(msg) => println!("关闭用户数据流: {:?}", msg),
            Err(e) => println!("错误: {:?}", e),
        }
    } else {
        println!("无法启动用户数据流 (请检查您的 API_KEY)");
    }
    Ok(())
}
```

### WEBSOCKETS (现货)

#### 用户数据流数据 (现货)

监听账户更新和订单成交。

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
    let api_key_user = Some("YOUR_API_KEY".into()); // 重要：替换为您的 API Key
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
                            println!("账户更新: {:?}", account_update.data);
                            // 示例：在第一次账户更新后停止
                            // kr_handler_clone.store(false, Ordering::Relaxed); 
                        },
                        WebsocketEvent::OrderTrade(trade) => {
                            println!("订单成交: {:?}", trade);
                        },
                        WebsocketEvent::OcoOrderTrade(oco_trade) => {
                            println!("OCO 订单成交: {:?}", oco_trade);
                        }
                        _ => println!("未处理事件: {:?}", event),
                    };
                    Ok(())
                })
            });
            
            if let Err(e) = web_socket.connect(&listen_key).await { // 用户数据流端点
                println!("连接现货用户数据流错误: {:?}", e);
                return Ok(());
            }

            if let Err(e) = web_socket.event_loop(keep_running.clone()).await {
                println!("现货用户数据流事件循环错误: {:?}", e);
            }
            
            web_socket.disconnect().await.unwrap();
            println!("现货用户数据流已断开");

        }
        Err(e) => println!("无法启动现货用户数据流: {:?}", e),
    }
    Ok(())
}
```

#### 成交 (现货)

监听单个成交信息。

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
    let trade_symbol = "bnbbtc"; // 示例交易对
    let stream_name = format!("{}@trade", trade_symbol);

    let mut web_socket: WebSockets<'_,_> = WebSockets::new(move |event: WebsocketEvent| -> Pin<Box<dyn Future<Output = BinanceResult<()>> + Send + 'static>> {
        let kr_handler_clone = kr_clone.clone();
        Box::pin(async move {
            match event {
                WebsocketEvent::Trade(trade_event) => {
                    println!("交易对: {}, 价格: {}, 数量: {}", trade_event.symbol, trade_event.price, trade_event.qty);
                    // kr_handler_clone.store(false, Ordering::Relaxed); // 第一次成交后停止
                }
                _ => (),
            };
            Ok(())
        })
    });

    if let Err(e) = web_socket.connect(&stream_name).await {
        println!("连接 {} 数据流错误: {:?}", stream_name, e);
        return Ok(());
    }
    if let Err(e) = web_socket.event_loop(keep_running.clone()).await {
        println!("事件循环错误: {:?}", e);
    }
    web_socket.disconnect().await.unwrap();
    println!("已从 {} 数据流断开", stream_name);
    Ok(())
}
```

#### K线 (现货)

监听K线 (蜡烛图) 数据。

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
                println!("交易对: {}, 时间间隔: {}, 开: {}, 收: {}, 高: {}, 低: {}", 
                    kline_event.kline.symbol, kline_event.kline.interval, kline_event.kline.open, 
                    kline_event.kline.close, kline_event.kline.high, kline_event.kline.low);
                // kr_handler_clone.store(false, Ordering::Relaxed); // 第一条K线后停止
            }
            Ok(())
        })
    });

    if let Err(e) = web_socket.connect(&stream_name).await {
        println!("连接 {} 数据流错误: {:?}", stream_name, e);
        return Ok(());
    }
    if let Err(e) = web_socket.event_loop(keep_running.clone()).await {
        println!("事件循环错误: {:?}", e);
    }
    web_socket.disconnect().await.unwrap();
    println!("已从 {} 数据流断开", stream_name);
    Ok(())
}
```

#### 多流订阅 (现货)

同时连接多个数据流。

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
        let _kr_handler_clone = kr_clone.clone(); // 如果希望根据事件停止，请使用此克隆
        Box::pin(async move {
            println!("收到事件: {:?}", event);
            // 示例：为演示目的，在收到任意3个事件后停止。
            // static COUNTER: AtomicUsize = AtomicUsize::new(0);
            // if COUNTER.fetch_add(1, Ordering::Relaxed) >= 2 {
            //     _kr_handler_clone.store(false, Ordering::Relaxed);
            // }
            Ok(())
        })
    });

    if let Err(e) = web_socket.connect_multiple_streams(&streams).await {
         println!("连接多数据流错误: {:?}", e);
        return Ok(());
    }
    
    if let Err(e) = web_socket.event_loop(keep_running.clone()).await {
        println!("事件循环错误: {:?}", e);
    }
    web_socket.disconnect().await.unwrap();
    println!("已从多数据流断开");
    Ok(())
}
```

## 合约 API

合约 API 客户端允许与 USD-M (USDT本位) 和 COIN-M (币本位) 合约进行交互。

### 通用 (合约)

币安合约的通用端点。

```rust
use binance::api::*;
use binance::futures::general::*;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let general: FuturesGeneral = Binance::new(None, None);

    // Ping USD-M 合约
    match general.ping().await { // 默认为 USD-M
        Ok(answer) => println!("合约 Ping (USD-M) 成功: {:?}", answer),
        Err(err) => println!("Ping USD-M 合约错误: {:?}", err),
    }
    
    // Ping COIN-M 合约
    match general.ping_coin_m().await {
        Ok(answer) => println!("合约 Ping (COIN-M) 成功: {:?}", answer),
        Err(err) => println!("Ping COIN-M 合约错误: {:?}", err),
    }

    // 服务器时间 (USD-M)
    match general.get_server_time().await { // 默认为 USD-M
        Ok(answer) => println!("合约服务器时间 (USD-M): {}", answer.server_time),
        Err(e) => println!("获取 USD-M 合约服务器时间错误: {:?}", e),
    }
    
    // 服务器时间 (COIN-M)
    match general.get_server_time_coin_m().await {
        Ok(answer) => println!("合约服务器时间 (COIN-M): {}", answer.server_time),
        Err(e) => println!("获取 COIN-M 合约服务器时间错误: {:?}", e),
    }

    // 交易规则 (USD-M)
    match general.exchange_info().await { // 默认为 USD-M
        Ok(answer) => println!("合约交易规则 (USD-M): {} 个交易对", answer.symbols.len()),
        Err(e) => println!("获取 USD-M 合约交易规则错误: {:?}", e),
    }

    // 交易规则 (COIN-M)
    match general.exchange_info_coin_m().await {
        Ok(answer) => println!("合约交易规则 (COIN-M): {} 个交易对", answer.symbols.len()),
        Err(e) => println!("获取 COIN-M 合约交易规则错误: {:?}", e),
    }
    Ok(())
}
```

### 市场数据 (合约)

访问合约的市场数据。

```rust
use binance::api::*;
use binance::futures::market::*;
use binance::futures::model::*; // 用于 Trades, KlineSummaries 等枚举
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let market: FuturesMarket = Binance::new(None, None);

    // USD-M 合约示例 (默认)
    let symbol_usdm = "BTCUSDT";
    println!("--- {} USD-M 合约 ---", symbol_usdm);

    match market.get_depth(symbol_usdm).await { // 默认为 USD-M
        Ok(answer) => println!("深度 (USD-M) 更新 ID: {:?}", answer.last_update_id),
        Err(e) => println!("获取 USD-M 深度错误: {:?}", e),
    }

    match market.get_klines(symbol_usdm, "5m", 10, None, None).await { // 默认为 USD-M
        Ok(KlineSummaries::AllKlineSummaries(answer)) => {
            if let Some(kline) = answer.first() {
                println!("第一条 K 线 (USD-M): {:?}", kline);
            }
        },
        Err(e) => println!("获取 USD-M K 线错误: {:?}", e),
    }

    // COIN-M 合约示例
    let symbol_coinm = "BTCUSD_PERP"; // 或特定交割合约，如 BTCUSD_240927
    println!("\n--- {} COIN-M 合约 ---", symbol_coinm);

    match market.get_depth_coin_m(symbol_coinm).await {
        Ok(answer) => println!("深度 (COIN-M) 更新 ID: {:?}", answer.last_update_id),
        Err(e) => println!("获取 COIN-M 深度错误: {:?}", e),
    }

    match market.get_klines_coin_m(symbol_coinm, "15m", 5, None, None).await {
         Ok(KlineSummaries::AllKlineSummaries(answer)) => {
            if let Some(kline) = answer.first() {
                println!("第一条 K 线 (COIN-M): {:?}", kline);
            }
        },
        Err(e) => println!("获取 COIN-M K 线错误: {:?}", e),
    }
    
    // 其他市场数据端点 (显示 USD-M, COIN-M 版本添加 _coin_m 后缀)
    // 例如: get_trades, get_agg_trades, get_24h_price_stats, get_price, get_all_book_tickers 等。
    // 示例:
    match market.get_price(symbol_usdm).await {
        Ok(price_ticker) => println!("{} 价格 (USD-M): {}", symbol_usdm, price_ticker.price),
        Err(e) =>  println!("错误: {:?}", e)
    }
    
    match market.get_price_coin_m(symbol_coinm).await {
        Ok(price_ticker) => println!("{} 价格 (COIN-M): {}", symbol_coinm, price_ticker.price),
        Err(e) =>  println!("错误: {:?}", e)
    }

    Ok(())
}
```

### 用户数据流 (合约)

管理合约的用户数据流。需要 API Key。

```rust
use binance::api::*;
use binance::futures::userstream::*;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = Some("YOUR_FUTURES_API_KEY".into()); // 替换为你的合约 API Key
    // 如果币安更改要求，某些用户数据流相关操作可能需要 Secret Key。
    // 对于 start/keep_alive/close，通常只需要具有正确权限的 API Key。
    let user_stream: FuturesUserStream = Binance::new(api_key, None); 

    // USD-M 用户数据流
    println!("--- USD-M 合约用户数据流 ---");
    match user_stream.start().await { // 默认为 USD-M
        Ok(answer) => {
            println!("USD-M 数据流已启动。Listen Key: {}", answer.listen_key);
            let listen_key = answer.listen_key;

            match user_stream.keep_alive(&listen_key).await {
                Ok(msg) => println!("保持 USD-M 数据流连接: {:?}", msg),
                Err(e) => println!("保持 USD-M 数据流连接错误: {:?}", e),
            }
            // tokio::time::sleep(tokio::time::Duration::from_secs(10)).await; // 保持数据流开启
            match user_stream.close(&listen_key).await {
                Ok(msg) => println!("已关闭 USD-M 数据流: {:?}", msg),
                Err(e) => println!("关闭 USD-M 数据流错误: {:?}", e),
            }
        }
        Err(e) => println!("无法启动 USD-M 用户数据流: {:?}", e),
    }

    // COIN-M 用户数据流
    println!("\n--- COIN-M 合约用户数据流 ---");
    match user_stream.start_coin_m().await {
        Ok(answer) => {
            println!("COIN-M 数据流已启动。Listen Key: {}", answer.listen_key);
            let listen_key_coinm = answer.listen_key;

            match user_stream.keep_alive_coin_m(&listen_key_coinm).await {
                Ok(msg) => println!("保持 COIN-M 数据流连接: {:?}", msg),
                Err(e) => println!("保持 COIN-M 数据流连接错误: {:?}", e),
            }
            // tokio::time::sleep(tokio::time::Duration::from_secs(10)).await; // 保持数据流开启
            match user_stream.close_coin_m(&listen_key_coinm).await {
                Ok(msg) => println!("已关闭 COIN-M 数据流: {:?}", msg),
                Err(e) => println!("关闭 COIN-M 数据流错误: {:?}", e),
            }
        }
        Err(e) => println!("无法启动 COIN-M 用户数据流: {:?}", e),
    }
    Ok(())
}
```

### WEBSOCKETS (合约)

连接到 USD-M 和 COIN-M 合约的市场数据 WebSocket 流。
对于用户数据流 (账户更新、订单更新)，请使用从 `FuturesUserStream` 的 `start()` 或 `start_coin_m()` 方法获取的 `listen_key`，并连接到相应的 USD-M 或 COIN-M 用户数据 WebSocket 端点。

```rust
use binance::futures::websockets::{FuturesWebSockets, FuturesWebsocketEvent, FuturesMarket};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use anyhow::Result;
use std::pin::Pin;
use std::future::Future;
use binance::errors::Result as BinanceResult; // 避免与 anyhow::Result 冲突

#[tokio::main]
async fn main() -> Result<()> {
    // USD-M 市场数据流示例
    let usdm_symbol = "btcusdt";
    let usdm_stream_name = format!("{}@kline_1m", usdm_symbol); // 示例：BTCUSDT 的 1 分钟 K 线
    println!("正在连接到 USD-M 数据流: {}", usdm_stream_name);

    let keep_running_usdm = Arc::new(AtomicBool::new(true));
    let kr_usdm_clone = Arc::clone(&keep_running_usdm);

    let mut ws_usdm: FuturesWebSockets<'_,_> = FuturesWebSockets::new(move |event: FuturesWebsocketEvent| -> Pin<Box<dyn Future<Output = BinanceResult<()>> + Send + 'static>> {
        let kr_handler_clone = kr_usdm_clone.clone();
        Box::pin(async move {
            println!("USD-M 事件: {:?}", event);
            // kr_handler_clone.store(false, Ordering::Relaxed); // 示例：在第一个事件后停止
            Ok(())
        })
    });

    if let Err(e) = ws_usdm.connect(&FuturesMarket::USDM, &usdm_stream_name).await {
        eprintln!("连接 USD-M 数据流 {} 失败: {:?}", usdm_stream_name, e);
    } else {
        if let Err(e) = ws_usdm.event_loop(keep_running_usdm).await {
            eprintln!("USD-M 数据流 {} 事件循环错误: {:?}", usdm_stream_name, e);
        }
        ws_usdm.disconnect().await.unwrap_or_else(|e| eprintln!("断开 USD-M 连接失败: {:?}", e));
        println!("已从 USD-M 数据流 {} 断开", usdm_stream_name);
    }
    
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await; // 短暂暂停

    // COIN-M 市场数据流示例
    let coinm_symbol = "btcusd_perp";
    let coinm_stream_name = format!("{}@aggTrade", coinm_symbol); // 示例：BTCUSD_PERP 的聚合交易
    println!("\n正在连接到 COIN-M 数据流: {}", coinm_stream_name);
    
    let keep_running_coinm = Arc::new(AtomicBool::new(true));
    let kr_coinm_clone = Arc::clone(&keep_running_coinm);

    let mut ws_coinm: FuturesWebSockets<'_,_> = FuturesWebSockets::new(move |event: FuturesWebsocketEvent| -> Pin<Box<dyn Future<Output = BinanceResult<()>> + Send + 'static>> {
        let kr_handler_clone = kr_coinm_clone.clone();
        Box::pin(async move {
            println!("COIN-M 事件: {:?}", event);
            // kr_handler_clone.store(false, Ordering::Relaxed); // 示例：在第一个事件后停止
            Ok(())
        })
    });

    if let Err(e) = ws_coinm.connect(&FuturesMarket::COINM, &coinm_stream_name).await {
        eprintln!("连接 COIN-M 数据流 {} 失败: {:?}", coinm_stream_name, e);
    } else {
        if let Err(e) = ws_coinm.event_loop(keep_running_coinm).await {
             eprintln!("COIN-M 数据流 {} 事件循环错误: {:?}", coinm_stream_name, e);
        }
        ws_coinm.disconnect().await.unwrap_or_else(|e| eprintln!("断开 COIN-M 连接失败: {:?}", e));
        println!("已从 COIN-M 数据流 {} 断开", coinm_stream_name);
    }

    // 对于合约用户数据流:
    // 1. 使用 `FuturesUserStream::start()` (USD-M) 或 `FuturesUserStream::start_coin_m()` (COIN-M) 获取 listen_key。
    // 2. 使用 `FuturesWebSockets::connect()` 和 `FuturesMarket::USDMUserData(&listen_key)` 或 `FuturesMarket::COINMUserData(&listen_key)` 连接。
    // 示例 (概念性，确保您拥有有效的 listen_key):
    // let listen_key_example = "your_futures_listen_key";
    // if let Err(e) = ws_usdm.connect(&FuturesMarket::USDMUserData(listen_key_example), "").await { ... } 
    // 用户数据流的 stream name 参数通常被忽略或应为空字符串，
    // 因为 listen_key 本身定义了数据流。如果需要，请查阅币安文档以获取具体信息。

    Ok(())
}
```

## 错误处理

提供更详细的错误信息。该库对大多数可能失败的操作使用 `anyhow::Result`，对特定的币安 API 错误使用 `binance::errors::ErrorKind`。

您可以查看 [币安错误代码](https://github.com/binance-exchange/binance-official-api-docs/blob/master/errors.md) 和 [合约错误代码](https://binance-docs.github.io/apidocs/futures/en/#error-codes)。

```rust
use binance::errors::{Error as BinanceError, ErrorKind as BinanceLibErrorKind}; // 假设 ErrorKind 是 BinanceError 的一部分
use anyhow::Result;

async fn example_error_handling() -> Result<()> {
    // ... 一些币安客户端调用 ...
    // let result = account.get_account().await;
    // match result {
    //     Ok(data) => { /* ... */ }
    //     Err(e) => {
    //         println!("发生错误: {}", e); // 通用的 anyhow 错误
    //
    //         // 获取更具体的币安错误详情:
    //         if let Some(binance_err) = e.downcast_ref::<BinanceError>() {
    //             match binance_err.kind() { // 假设您的 BinanceError 上有 kind() 方法
    //                 BinanceLibErrorKind::BinanceError(response) => {
    //                     println!("币安 API 错误代码: {}, 信息: {}", response.code, response.msg);
    //                     match response.code {
    //                         -1013i32 => println!("过滤器失败: LOT_SIZE!"), // 注意: 代码可能是 i32 或其他类型
    //                         -2010i32 => println!("资金不足! {}", response.msg),
    //                         _ => println!("未捕获的代码 {}: {}", response.code, response.msg),
    //                     }
    //                 }
    //                 BinanceLibErrorKind::Msg(msg) => {
    //                     println!("币安库内部错误信息: {}", msg);
    //                 }
    //                 _ => println!("其他币安库错误: {:?}", binance_err.kind()),
    //             }
    //         }
    //     }
    // }
    Ok(()) // 占位符
}
```
*注意: `BinanceError` 和 `ErrorKind` 的确切结构应参照 `src/errors.rs` 以进行精确处理。*

## 测试网和API集群

如果默认的币安 API URL 存在性能问题，或者您想使用测试网，可以覆盖它们。
`Config` 结构体可用于为现货、USD-M 合约和 COIN-M 合约指定自定义端点。

[币安现货 API 集群](https://github.com/binance/binance-spot-api-docs/blob/master/rest-api.md#general-api-information)
[币安合约 API 端点](https://binance-docs.github.io/apidocs/futures/en/#endpoint-information) (包含测试网)

```rust
use binance::api::*;
use binance::config::Config;
use binance::market::Market; // 用于现货
use binance::futures::general::FuturesGeneral; // 用于合约

fn main() {
    let use_spot_testnet = true;
    let use_futures_testnet = true;

    // 现货测试网配置
    if use_spot_testnet {
        let config_spot = Config::default()
            .set_rest_api_endpoint("https://testnet.binance.vision")
            .set_ws_endpoint("wss://testnet.binance.vision/ws"); // 用于现货 WebSockets

        let market_spot_testnet: Market = Binance::new_with_config(None, None, &config_spot);
        // 现在使用 market_spot_testnet 进行现货测试网调用
        println!("正在使用现货测试网: {}", config_spot.rest_api_endpoint);
    }

    // 合约测试网配置 (USD-M)
    if use_futures_testnet {
        let config_futures_usdm = Config::default()
            .set_futures_rest_api_endpoint(binance::config::FuturesEndpoint::UsdMFuturesTestnet) // 或自定义字符串
            .set_futures_ws_endpoint(binance::config::FuturesEndpoint::UsdMFuturesTestnet);     // 或用于 WS 的自定义字符串

        let general_futures_usdm_testnet: FuturesGeneral = Binance::new_with_config(None, None, &config_futures_usdm);
        // 现在使用 general_futures_usdm_testnet 进行 USD-M 合约测试网调用
        println!("正在使用 USD-M 合约测试网 REST: {}", config_futures_usdm.futures_rest_api_endpoint);
        println!("正在使用 USD-M 合约测试网 WS: {}", config_futures_usdm.futures_ws_endpoint);

        // 对于 COIN-M 合约测试网
        let config_futures_coinm = Config::default()
            .set_futures_coin_m_rest_api_endpoint(binance::config::FuturesEndpoint::CoinMFuturesTestnet)
            .set_futures_coin_m_ws_endpoint(binance::config::FuturesEndpoint::CoinMFuturesTestnet);
        
        let general_futures_coinm_testnet: FuturesGeneral = Binance::new_with_config(None, None, &config_futures_coinm);
         println!("正在使用 COIN-M 合约测试网 REST: {}", config_futures_coinm.futures_coin_m_rest_api_endpoint);
         println!("正在使用 COIN-M 合约测试网 WS: {}", config_futures_coinm.futures_coin_m_ws_endpoint);
    }
}
```