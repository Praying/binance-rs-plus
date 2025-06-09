// use binance_rs_plus::errors::ErrorKind as BinanceLibErrorKind; // Removed
use anyhow::Result;
use binance_rs_plus::account::*;
use binance_rs_plus::api::*;
use binance_rs_plus::config::*;
use binance_rs_plus::general::*;
use binance_rs_plus::market::*;
use binance_rs_plus::savings::*;
// Added for anyhow::Result

#[tokio::main]
async fn main() -> Result<()> {
    // The general spot API endpoints; shown with
    // testnet=false and testnet=true
    general(false).await?;
    general(true).await?;

    // The market data API endpoint
    market_data().await?;

    // The account data API and savings API endpoint examples need an API key. Change those lines locally
    // and uncomment the line below (and do not commit your api key :)).
    // account().await?;
    // savings().await?;
    Ok(())
}

async fn general(use_testnet: bool) -> Result<()> {
    let general: General = if use_testnet {
        let config = Config::default().set_rest_api_endpoint("https://testnet.binance.vision");
        Binance::new_with_config(None, None, &config)
    } else {
        Binance::new(None, None)
    };

    let ping = general.ping().await;
    match ping {
        Ok(answer) => println!("Ping successful: {:?}", answer), // Ensure Empty struct is Debug
        Err(err) => {
            println!("Error pinging: {:?}", err);
        }
    }

    let result = general.get_server_time().await;
    match result {
        Ok(answer) => println!("Server Time: {}", answer.server_time),
        Err(e) => println!("Error getting server time: {:?}", e),
    }

    let result = general.exchange_info().await;
    match result {
        Ok(answer) => println!("Exchange information: {:?}", answer),
        Err(e) => println!("Error getting exchange info: {:?}", e),
    }

    let result = general.get_symbol_info("ethbtc").await;
    match result {
        Ok(answer) => println!("Symbol information: {:?}", answer),
        Err(e) => println!("Error getting symbol info: {:?}", e),
    }
    Ok(())
}

#[allow(dead_code)]
async fn account() -> Result<()> {
    let api_key = Some("YOUR_API_KEY".into());
    let secret_key = Some("YOUR_SECRET_KEY".into());

    let account: Account = Binance::new(api_key, secret_key);

    match account.get_account().await {
        Ok(answer) => println!("{:?}", answer.balances),
        Err(e) => println!("Error getting account: {:?}", e),
    }

    match account.get_open_orders("WTCETH").await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error getting open orders: {:?}", e),
    }

    match account.limit_buy("WTCETH", 10, 0.014000).await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error limit buy: {:?}", e),
    }

    match account.market_buy("WTCETH", 5).await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error market buy: {:?}", e),
    }

    match account.market_buy_using_quote_quantity("WTCETH", 5).await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error market buy (quote): {:?}", e),
    }

    match account.limit_sell("WTCETH", 10, 0.035000).await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error limit sell: {:?}", e),
    }

    match account.market_sell("WTCETH", 5).await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error market sell: {:?}", e),
    }

    match account.market_sell_using_quote_quantity("WTCETH", 5).await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error market sell (quote): {:?}", e),
    }

    let order_id = 1_957_528;
    match account.order_status("WTCETH", order_id).await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error order status: {:?}", e),
    }

    match account.cancel_order("WTCETH", order_id).await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error cancel order: {:?}", e),
    }

    match account.get_balance("KNC").await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error get balance: {:?}", e),
    }

    match account.trade_history("WTCETH").await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error trade history: {:?}", e),
    }
    Ok(())
}

#[allow(dead_code)]
async fn savings() -> Result<()> {
    let api_key = Some("YOUR_API_KEY".into());
    let api_secret = Some("YOUR_SECRET_KEY".into());

    let savings: Savings = Binance::new(api_key, api_secret);

    match savings.get_all_coins().await {
        Ok(answer) => println!("{:#?}", answer),
        Err(e) => println!("Error getting all coins (savings): {:?}", e),
    }

    match savings.asset_detail(None).await {
        Ok(answer) => println!("{:#?}", answer),
        Err(e) => println!("Error getting asset detail (savings): {:?}", e),
    }

    match savings.deposit_address("BTC", None).await {
        Ok(answer) => println!("{:#?}", answer),
        Err(e) => println!("Error getting deposit address (savings): {:?}", e),
    }
    Ok(())
}

#[allow(dead_code)]
async fn market_data() -> Result<()> {
    let market: Market = Binance::new(None, None);

    // Order book at default depth
    match market.get_depth("BNBETH").await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error getting depth: {:?}", e),
    }
    // Order book at depth 500
    match market.get_custom_depth("BNBETH", 500).await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error getting custom depth: {:?}", e),
    }

    // Latest price for ALL symbols
    match market.get_all_prices().await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error getting all prices: {:?}", e),
    }

    // Latest price for ONE symbol
    match market.get_price("KNCETH").await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error getting price: {:?}", e),
    }

    // Current average price for ONE symbol
    match market.get_average_price("KNCETH").await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error getting average price: {:?}", e),
    }

    // Best price/qty on the order book for ALL symbols
    match market.get_all_book_tickers().await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error getting all book tickers: {:?}", e),
    }

    // Best price/qty on the order book for ONE symbol
    match market.get_book_ticker("BNBETH").await {
        Ok(answer) => println!(
            "Bid Price: {}, Ask Price: {}",
            answer.bid_price, answer.ask_price
        ),
        Err(e) => println!("Error getting book ticker: {:?}", e),
    }

    // 24hr ticker price change statistics
    match market.get_24h_price_stats("BNBETH").await {
        Ok(answer) => println!(
            "Open Price: {}, Higher Price: {}, Lower Price: {:?}",
            answer.open_price, answer.high_price, answer.low_price
        ),
        Err(e) => println!("Error getting 24h price stats: {:?}", e),
    }

    // 10 latest (aggregated) trades
    match market
        .get_agg_trades("BNBETH", None, None, None, Some(10))
        .await
    {
        Ok(trades) => {
            if let Some(trade) = trades.first() {
                // Check if trades is not empty
                println!(
                    "{} BNB Qty: {}, Price: {}",
                    if trade.maker { "SELL" } else { "BUY" },
                    trade.qty,
                    trade.price
                );
            } else {
                println!("No agg trades found for BNBETH");
            }
        }
        Err(e) => println!("Error getting agg trades: {:?}", e),
    }

    // last 10 5min klines (candlesticks) for a symbol:
    match market.get_klines("BNBETH", "5m", 10, None, None).await {
        Ok(klines) => {
            match klines {
                binance_rs_plus::model::KlineSummaries::AllKlineSummaries(klines_vec) => {
                    if let Some(kline_summary) = klines_vec.first() {
                        // Check if klines_vec is not empty
                        // kline_summary is already a KlineSummary, no need to clone if just reading
                        println!(
                            "Open: {}, High: {}, Low: {}",
                            kline_summary.open, kline_summary.high, kline_summary.low
                        );
                    } else {
                        println!("No klines found for BNBETH");
                    }
                }
            }
        }
        Err(e) => println!("Error getting klines: {:?}", e),
    }
    Ok(())
}
