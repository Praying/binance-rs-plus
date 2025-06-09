#![allow(dead_code)]

use binance_rs_plus::api::*;
use binance_rs_plus::userstream::*;
use binance_rs_plus::websockets::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc; // Added
use anyhow::Result; // Added
use std::pin::Pin; // Added
use std::future::Future; // Added
use binance_rs_plus::errors::Result as BinanceResult; // For handler return type

#[tokio::main]
async fn main() -> Result<()> {
    // Uncomment the examples you want to run:
    // user_stream().await?;
    // user_stream_websocket().await?;
    market_websocket().await?;
    // kline_websocket().await?;
    // all_trades_websocket().await?;
    // last_price_for_one_symbol().await?;
    // multiple_streams().await?;
    Ok(())
}

async fn user_stream() -> Result<()> {
    let api_key_user = Some("YOUR_API_KEY".into()); // Replace with your actual API key
    // let secret_key_user = Some("YOUR_SECRET_KEY".into()); // Secret might be needed for some operations
    let user_stream: UserStream = Binance::new(api_key_user, None);

    match user_stream.start().await {
        Ok(answer) => {
            println!("Data Stream Started ...");
            let listen_key = answer.listen_key;
            println!("Listen Key: {}", listen_key);

            match user_stream.keep_alive(&listen_key).await {
                Ok(msg) => println!("Keepalive user data stream: {:?}", msg),
                Err(e) => eprintln!("Error keeping alive user data stream: {:?}", e),
            }

            // tokio::time::sleep(tokio::time::Duration::from_secs(10)).await; // Keep alive for a bit

            match user_stream.close(&listen_key).await {
                Ok(msg) => println!("Close user data stream: {:?}", msg),
                Err(e) => eprintln!("Error closing user data stream: {:?}", e),
            }
        }
        Err(e) => {
            eprintln!(
                "Not able to start an User Stream (Check your API_KEY): {:?}",
                e
            );
        }
    }
    Ok(())
}

async fn user_stream_websocket() -> Result<()> {
    let keep_running = Arc::new(AtomicBool::new(true));
    let api_key_user = Some("YOUR_API_KEY".into()); // Replace with your actual API key
    let user_stream: UserStream = Binance::new(api_key_user, None);

    match user_stream.start().await {
        Ok(answer) => {
            let listen_key = answer.listen_key;
            println!("User stream started with listen key: {}", listen_key);

            let mut web_socket: WebSockets<'_> = WebSockets::new(move |event: WebsocketEvent| {
                Box::pin(async move {
                    match event {
                        WebsocketEvent::AccountUpdate(account_update) => {
                            println!("Account Update: {:?}", account_update.data);
                            for balance in &account_update.data.balances {
                                println!(
                                    "Asset: {}, Wallet Balance: {}, Cross Wallet Balance: {}, Balance Change: {}",
                                    balance.asset,
                                    balance.wallet_balance,
                                    balance.cross_wallet_balance,
                                    balance.balance_change
                                );
                            }
                        }
                        WebsocketEvent::OrderTrade(trade) => {
                            println!(
                                "Order Trade Update: Symbol: {}, Side: {}, Price: {}, Execution Type: {}",
                                trade.symbol, trade.side, trade.price, trade.execution_type
                            );
                        }
                        _ => { /* println!("Other user stream event: {:?}", event) */ }
                    };
                    Ok(()) as BinanceResult<()>
                })
            });

            web_socket.connect(&listen_key).await?;
            println!("Connected to user stream WebSocket.");

            // Run for a bit then stop
            let keep_running_clone = keep_running.clone();
            tokio::spawn(async move {
                tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
                println!("Signaling user stream WebSocket to stop...");
                keep_running_clone.store(false, Ordering::Relaxed);
            });

            if let Err(e) = web_socket.event_loop(keep_running.clone()).await {
                if e.to_string().contains("WebSocket closed by server")
                    || e.to_string().contains("WebSocket stream ended")
                {
                    println!("User stream WebSocket closed as expected or stream ended.");
                } else {
                    eprintln!("Error in user stream WebSocket event loop: {:?}", e);
                }
            }

            println!("Closing user stream channel with Binance...");
            user_stream.close(&listen_key).await?;
            println!("Disconnecting user stream WebSocket...");
            web_socket.disconnect().await?;
            println!("User stream closed and disconnected.");
        }
        Err(e) => {
            eprintln!(
                "Not able to start an User Stream (Check your API_KEY): {:?}",
                e
            );
        }
    }
    Ok(())
}

async fn market_websocket() -> Result<()> {
    let keep_running = Arc::new(AtomicBool::new(true));
    let keep_running_for_event_loop = Arc::clone(&keep_running);
    let agg_trade_stream_name = String::from("ethbtc@aggTrade");
    let mut web_socket: WebSockets<'_> = WebSockets::new(move |event: WebsocketEvent| {
        let keep_running_for_callback = Arc::clone(&keep_running);
        Box::pin(async move {
            match event {
                WebsocketEvent::Trade(trade) => {
                    println!(
                        "Symbol: {}, price: {}, qty: {}",
                        trade.symbol, trade.price, trade.qty
                    );
                }
                WebsocketEvent::DepthOrderBook(depth_order_book) => {
                    println!(
                        "Symbol: {}, Bids: {:?}, Ask: {:?}",
                        depth_order_book.symbol, depth_order_book.bids, depth_order_book.asks
                    );
                }
                WebsocketEvent::OrderBook(order_book) => {
                    println!(
                        "last_update_id: {}, Bids: {:?}, Ask: {:?}",
                        order_book.last_update_id, order_book.bids, order_book.asks
                    );
                }
                WebsocketEvent::AggrTrades(agg_trade_event) => {
                    println!("Aggregated Trade: {:?}", agg_trade_event);
                    // Example: Stop after one event for this type of stream
                    keep_running_for_callback.store(false, Ordering::Relaxed);
                }
                _ => { /* println!("Other market event: {:?}", event) */ }
            };
            Ok(()) as BinanceResult<()>
        })
    });

    web_socket.connect(&agg_trade_stream_name).await?;
    println!("Connected to {} stream.", agg_trade_stream_name);

    if let Err(e) = web_socket.event_loop(keep_running_for_event_loop).await {
        if e.to_string().contains("WebSocket closed by server")
            || e.to_string().contains("WebSocket stream ended")
        {
            println!(
                "Market WebSocket {} closed as expected or stream ended.",
                agg_trade_stream_name
            );
        } else {
            eprintln!(
                "Error in market WebSocket event loop ({}): {:?}",
                agg_trade_stream_name, e
            );
        }
    }
    web_socket.disconnect().await?;
    println!("Disconnected from {} stream.", agg_trade_stream_name);
    Ok(())
}

async fn all_trades_websocket() -> Result<()> {
    let keep_running = Arc::new(AtomicBool::new(true));
    let keep_running_for_event_loop = Arc::clone(&keep_running);
    let all_tickers_stream = String::from("!ticker@arr");
    let mut web_socket = WebSockets::new(move |event: WebsocketEvent| {
        let keep_running_for_callback = Arc::clone(&keep_running);
        Box::pin(async move {
            if let WebsocketEvent::DayTickerAll(ticker_events) = event {
                println!(
                    "Received a batch of {} DayTickerAll events.",
                    ticker_events.len()
                );
                for tick_event in ticker_events.iter().take(2) {
                    // Print first 2 for brevity
                    println!(
                        "Symbol: {}, price: {}, qty: {}",
                        tick_event.symbol, tick_event.best_bid, tick_event.best_bid_qty
                    );
                }
                // Example: Stop after receiving one batch
                keep_running_for_callback.store(false, Ordering::Relaxed);
            }
            Ok(()) as BinanceResult<()>
        })
    });

    web_socket.connect(&all_tickers_stream).await?;
    println!("Connected to {} stream.", all_tickers_stream);
    if let Err(e) = web_socket.event_loop(keep_running_for_event_loop).await {
        if e.to_string().contains("WebSocket closed by server")
            || e.to_string().contains("WebSocket stream ended")
        {
            println!(
                "All trades WebSocket ({}) closed as expected or stream ended.",
                all_tickers_stream
            );
        } else {
            eprintln!(
                "Error in all_trades_websocket event loop ({}): {:?}",
                all_tickers_stream, e
            );
        }
    }
    web_socket.disconnect().await?;
    println!("Disconnected from {} stream.", all_tickers_stream);
    Ok(())
}

async fn kline_websocket() -> Result<()> {
    let keep_running = Arc::new(AtomicBool::new(true));
    let keep_running_for_event_loop = Arc::clone(&keep_running);
    let kline_stream = String::from("ethbtc@kline_1m");
    let mut web_socket = WebSockets::new(move |event: WebsocketEvent| {
        let keep_running_for_callback = Arc::clone(&keep_running);
        Box::pin(async move {
            if let WebsocketEvent::Kline(kline_event) = event {
                println!(
                    "Symbol: {}, high: {}, low: {}",
                    kline_event.kline.symbol, kline_event.kline.low, kline_event.kline.high
                );
                // Example: Stop after one kline event
                keep_running_for_callback.store(false, Ordering::Relaxed);
            }
            Ok(()) as BinanceResult<()>
        })
    });

    web_socket.connect(&kline_stream).await?;
    println!("Connected to {} stream.", kline_stream);
    if let Err(e) = web_socket.event_loop(keep_running_for_event_loop).await {
        if e.to_string().contains("WebSocket closed by server")
            || e.to_string().contains("WebSocket stream ended")
        {
            println!(
                "Kline WebSocket ({}) closed as expected or stream ended.",
                kline_stream
            );
        } else {
            eprintln!(
                "Error in kline_websocket event loop ({}): {:?}",
                kline_stream, e
            );
        }
    }
    web_socket.disconnect().await?;
    println!("Disconnected from {} stream.", kline_stream);
    Ok(())
}

async fn last_price_for_one_symbol() -> Result<()> {
    let keep_running = Arc::new(AtomicBool::new(true));
    let keep_running_for_event_loop = Arc::clone(&keep_running);
    let ticker_stream = String::from("btcusdt@ticker");
    // let mut btcusdt_price: f32 = 0.0; // Mutex or other sync primitive needed if accessed outside callback

    let mut web_socket = WebSockets::new(move |event: WebsocketEvent| {
        let keep_running_for_callback = Arc::clone(&keep_running);
        Box::pin(async move {
            if let WebsocketEvent::DayTicker(ticker_event) = event {
                // btcusdt_price = ticker_event.average_price.parse().unwrap_or_default();
                let current_close_price: f32 =
                    ticker_event.current_close.parse().unwrap_or_default();
                println!(
                    "BTCUSDT Ticker - Avg Price: {}, Current Close: {}",
                    ticker_event.average_price, current_close_price
                );

                // Example: Stop if price reaches a certain point (for testing)
                // if current_close_price > 20000.0 { // Adjust threshold as needed
                //     println!("BTCUSDT reached target price, stopping.");
                //     keep_running_for_callback.store(false, Ordering::Relaxed);
                // }
                // For this example, let's stop after one event.
                keep_running_for_callback.store(false, Ordering::Relaxed);
            }
            Ok(()) as BinanceResult<()>
        })
    });

    web_socket.connect(&ticker_stream).await?;
    println!("Connected to {} stream.", ticker_stream);
    if let Err(e) = web_socket.event_loop(keep_running_for_event_loop).await {
        if e.to_string().contains("WebSocket closed by server")
            || e.to_string().contains("WebSocket stream ended")
        {
            println!(
                "Ticker WebSocket ({}) closed as expected or stream ended.",
                ticker_stream
            );
        } else {
            eprintln!(
                "Error in last_price_for_one_symbol event loop ({}): {:?}",
                ticker_stream, e
            );
        }
    }
    web_socket.disconnect().await?;
    println!("Disconnected from {} stream.", ticker_stream);
    Ok(())
}

async fn multiple_streams() -> Result<()> {
    let endpoints = ["ethbtc@depth@100ms", "bnbeth@depth@100ms"]
        .map(String::from)
        .to_vec(); // Ensure Vec<String>

    let keep_running = Arc::new(AtomicBool::new(true));
    let keep_running_for_event_loop = Arc::clone(&keep_running);
    let mut web_socket: WebSockets<'_> = WebSockets::new(move |event: WebsocketEvent| {
        let keep_running_for_callback = Arc::clone(&keep_running);
        Box::pin(async move {
            if let WebsocketEvent::DepthOrderBook(depth_order_book) = event {
                println!("Multi-stream Depth: {:?}", depth_order_book.symbol);
                // Stop after a few events for brevity in example
                static mut COUNT: u8 = 0;
                unsafe {
                    COUNT += 1;
                    if COUNT > 5 {
                        keep_running_for_callback.store(false, Ordering::Relaxed);
                    }
                }
            }
            Ok(()) as BinanceResult<()>
        })
    });

    web_socket.connect_multiple_streams(&endpoints).await?;
    println!("Connected to multiple streams: {:?}", endpoints);
    if let Err(e) = web_socket.event_loop(keep_running_for_event_loop).await {
        if e.to_string().contains("WebSocket closed by server")
            || e.to_string().contains("WebSocket stream ended")
        {
            println!("Multiple streams WebSocket closed as expected or stream ended.");
        } else {
            eprintln!("Error in multiple_streams event loop: {:?}", e);
        }
    }
    web_socket.disconnect().await?;
    println!("Disconnected from multiple streams.");
    Ok(())
}
