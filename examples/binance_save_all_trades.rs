use std::error::Error as StdError;
use csv::Writer;
// Renamed to avoid conflict
use std::fs::File;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
// Added
use tokio::sync::Mutex;
// Added

use anyhow::Result;
use binance_rs_plus::model::DayTickerEvent;
use binance_rs_plus::websockets::*;
// Added
use binance_rs_plus::errors::Result as BinanceResult;
// For handler return type

#[tokio::main]
async fn main() -> Result<()> {
    save_all_trades_websocket().await?;
    Ok(())
}

async fn save_all_trades_websocket() -> Result<()> {
    struct WebSocketHandler {
        wrt: Writer<File>,
    }

    impl WebSocketHandler {
        pub fn new(local_wrt: Writer<File>) -> Self {
            WebSocketHandler { wrt: local_wrt }
        }

        // serialize DayTickerEvent as CSV records
        pub fn write_to_file(
            &mut self,
            events: Vec<DayTickerEvent>,
        ) -> Result<(), Box<dyn StdError>> {
            for event in events {
                self.wrt.serialize(event)?;
            }
            self.wrt.flush()?; // Flush after writing all events in the batch
            Ok(())
        }
    }

    let keep_running = Arc::new(AtomicBool::new(true)); // Used to control the event loop
    let file_path = std::path::Path::new("all_day_tickers.csv"); // Renamed for clarity
    let local_wrt = csv::Writer::from_path(file_path)?;

    let web_socket_handler = Arc::new(Mutex::new(WebSocketHandler::new(local_wrt)));

    let agg_trade_stream_name = String::from("!ticker@arr");

    // Note: The lifetime 'a for WebSockets is now 'static due to the move in the async block.
    // If web_socket_handler needed to be borrowed with a shorter lifetime, more complex handling would be needed.
    let mut web_socket: WebSockets<'_> = WebSockets::new(move |event: WebsocketEvent| {
        let handler_clone = Arc::clone(&web_socket_handler);
        // let keep_running_clone_for_callback = Arc::clone(&keep_running); // If needed inside callback

        Box::pin(async move {
            if let WebsocketEvent::DayTickerAll(events) = event {
                // Example: Stop after receiving a few batches or based on some condition
                // if some_condition {
                //     keep_running_clone_for_callback.store(false, Ordering::Relaxed);
                // }
                let mut locked_handler = handler_clone.lock().await;
                if let Err(error) = locked_handler.write_to_file(events) {
                    eprintln!("Error writing to CSV: {}", error); // Use eprintln for errors
                } else {
                    println!("Successfully wrote a batch of ticker events to CSV.");
                }
            }
            Ok(()) as BinanceResult<()> // Explicitly cast to BinanceResult
        })
    });

    web_socket.connect(&agg_trade_stream_name).await?; // .await and check error with ?
    println!(
        "Connected to {} stream. Waiting for events...",
        agg_trade_stream_name
    );

    // Event loop will run until keep_running is false or an error occurs.
    // For this example, it will run indefinitely until manually stopped or an error.
    // You might want to add a timeout or a counter to stop it for testing.
    // For instance, to run for a specific duration:
    // tokio::select! {
    //     res = web_socket.event_loop(keep_running.clone()) => {
    //         if let Err(e) = res {
    //              eprintln!("Event loop error: {}", e);
    //         }
    //     },
    //     _ = tokio::time::sleep(tokio::time::Duration::from_secs(60)) => {
    //         println!("Stopping after 60 seconds.");
    //         keep_running.store(false, Ordering::Relaxed);
    //     }
    // }
    // For now, let it run and user can stop with Ctrl+C or it stops on WebSocket error.
    if let Err(e) = web_socket.event_loop(keep_running.clone()).await {
        eprintln!("Error in WebSocket event loop: {}", e);
    }

    web_socket.disconnect().await?; // .await and check error
    println!("Disconnected from WebSocket stream.");
    Ok(())
}
