use binance_rs_plus::futures::websockets::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc; // Added for Arc
use anyhow::Result; // Added for anyhow
use std::pin::Pin; // Added for Pin
use std::future::Future; // Added for Future
use binance_rs_plus::errors::Result as BinanceResult; // To avoid conflict with anyhow::Result for handler return

#[tokio::main]
async fn main() -> Result<()> {
    market_websocket().await?;
    Ok(())
}

async fn market_websocket() -> Result<()> {
    // Example to show the future market websockets. It will print one event for each
    // endpoint and continue to the next.

    let stream_examples_usd_m = vec![
        "btcusdt@aggTrade",
        "btcusdt@markPrice",
        "btcusdt@kline_1m",
        // "btcusdt_perpetual@continuousKline_1m", // Might require specific pair/contract type logic if not standard symbol
        "btcusdt@miniTicker",
        "!miniTicker@arr",
        "btcusdt@ticker",
        "!ticker@arr",
        "btcusdt@bookTicker",
        "!bookTicker",
        "btcusdt@forceOrder", // This might take time to receive an event
        "!forceOrder@arr",
        "btcusdt@depth20@100ms",
        "btcusdt@depth@100ms",
    ];

    let stream_examples_coin_m = vec![
        // For COIN-M, symbols like "btcusd_perp" or "btcusd_240628" (example for June 2024 contract) are common.
        // Using a generic placeholder that is more likely to exist for testing.
        // Update these with valid, active COIN-M symbols if needed.
        "btcusd_perp@aggTrade",
        "btcusd_perp@markPrice",
        "btcusd_perp@kline_1m",
        // "btcusd_next_quarter@continuousKline_1m", // Needs valid next_quarter symbol
        "btcusd_perp@miniTicker",
        "!miniTicker@arr", // For COIN-M
        "btcusd_perp@ticker",
        "!ticker@arr", // For COIN-M
        "btcusd_perp@bookTicker",
        "!bookTicker", // For COIN-M
        "btcusd_perp@forceOrder",
        "!forceOrder@arr", // For COIN-M
        "btcusd_perp@depth20@100ms",
        "btcusd_perp@depth@100ms",
    ];


    // USD-M futures examples
    for stream_example in stream_examples_usd_m {
        println!("Starting with USD_M {:?}", stream_example);
        let keep_running = Arc::new(AtomicBool::new(true));
        let keep_running_clone = Arc::clone(&keep_running);

        let callback_fn = move |event: FuturesWebsocketEvent| -> Pin<Box<dyn Future<Output = BinanceResult<()>> + Send + 'static>> {
            let keep_running_for_handler = Arc::clone(&keep_running_clone);
            Box::pin(async move {
                println!("USD-M Event for {}: {:?}\n", stream_example, event);
                keep_running_for_handler.swap(false, Ordering::Relaxed);
                Ok(())
            })
        };
        
        let mut web_socket: FuturesWebSockets<'_> = FuturesWebSockets::new(callback_fn);
        
        if let Err(e) = web_socket.connect(&FuturesMarket::USDM, stream_example).await {
            eprintln!("Failed to connect to USD-M stream {}: {:?}", stream_example, e);
            continue;
        }

        if let Err(e) = web_socket.event_loop(keep_running.clone()).await {
             // Log error, but don't panic, try next stream
            eprintln!("Error in USD-M event loop for {}: {:?}", stream_example, e);
        }
        
        if let Err(e) = web_socket.disconnect().await {
            eprintln!("Failed to disconnect from USD-M stream {}: {:?}", stream_example, e);
        }
        println!("Finished with USD_M {:?}", stream_example);
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await; // Small delay before next
    }

    // COIN-M futures examples
    for stream_example in stream_examples_coin_m {
        println!("Starting with COIN_M {:?}", stream_example);
        let keep_running = Arc::new(AtomicBool::new(true));
        let keep_running_clone = Arc::clone(&keep_running);
        
        let callback_fn = move |event: FuturesWebsocketEvent| -> Pin<Box<dyn Future<Output = BinanceResult<()>> + Send + 'static>> {
            let keep_running_for_handler = Arc::clone(&keep_running_clone);
            Box::pin(async move {
                println!("COIN-M Event for {}: {:?}\n", stream_example, event);
                keep_running_for_handler.swap(false, Ordering::Relaxed);
                Ok(())
            })
        };

        let mut web_socket: FuturesWebSockets<'_> = FuturesWebSockets::new(callback_fn);

        if let Err(e) = web_socket.connect(&FuturesMarket::COINM, stream_example).await {
            eprintln!("Failed to connect to COIN-M stream {}: {:?}", stream_example, e);
            continue;
        }
        
        if let Err(e) = web_socket.event_loop(keep_running.clone()).await {
            eprintln!("Error in COIN-M event loop for {}: {:?}", stream_example, e);
        }

        if let Err(e) = web_socket.disconnect().await {
            eprintln!("Failed to disconnect from COIN-M stream {}: {:?}", stream_example, e);
        }
         println!("Finished with COIN_M {:?}", stream_example);
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await; // Small delay
    }
    Ok(())
}
