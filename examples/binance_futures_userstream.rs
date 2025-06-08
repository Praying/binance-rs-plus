use binance_rs_plus::api::*;
use binance_rs_plus::futures::userstream::*;
use anyhow::Result; // Added

#[tokio::main]
async fn main() -> Result<()> {
    user_stream().await?;
    Ok(())
}

async fn user_stream() -> Result<()> {
    let api_key_user = Some("YOUR_API_KEY".into()); // Replace with your actual API key
    // Secret key might be needed depending on exact endpoint requirements or future changes
    // let secret_key_user = Some("YOUR_SECRET_KEY".into());
    let user_stream: FuturesUserStream = Binance::new(api_key_user, None); // Pass None for secret if not strictly needed by these specific calls

    match user_stream.start().await {
        Ok(answer) => {
            println!("Futures Data Stream Started ...");
            let listen_key = answer.listen_key;
            println!("Listen Key: {}", listen_key);

            match user_stream.keep_alive(&listen_key).await {
                Ok(msg) => println!("Keepalive futures user data stream: {:?}", msg),
                Err(e) => println!("Error keeping alive futures user data stream: {:?}", e),
            }

            // It's good practice to sleep a bit if you intend to actually use the stream
            // For this example, we'll just close it immediately.
            // tokio::time::sleep(tokio::time::Duration::from_secs(60)).await; 

            match user_stream.close(&listen_key).await {
                Ok(msg) => println!("Close futures user data stream: {:?}", msg),
                Err(e) => println!("Error closing futures user data stream: {:?}", e),
            }
        }
        Err(e) => {
            println!("Not able to start a Futures User Stream (Check your API_KEY): {:?}", e);
        }
    }
    Ok(())
}
