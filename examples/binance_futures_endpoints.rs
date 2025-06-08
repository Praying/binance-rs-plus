use binance_rs_plus::api::*;
use binance_rs_plus::futures::general::*;
use binance_rs_plus::futures::market::*;
use binance_rs_plus::futures::model::*;
// use binance_rs_plus::errors::ErrorKind as BinanceLibErrorKind; // Removed
use anyhow::Result; // Added

#[tokio::main]
async fn main() -> Result<()> {
    general().await?;
    // account().await?; // Account example for futures is not present in original, keeping it commented
    market_data().await?;
    Ok(())
}

async fn general() -> Result<()> {
    let general: FuturesGeneral = Binance::new(None, None);

    match general.ping().await {
        Ok(answer) => println!("Futures Ping successful: {:?}", answer), // Ensure Empty struct is Debug
        Err(err) => {
            println!("Error pinging futures: {:?}", err);
        }
    }

    match general.get_server_time().await {
        Ok(answer) => println!("Futures Server Time: {}", answer.server_time),
        Err(e) => println!("Error getting futures server time: {:?}", e),
    }

    match general.exchange_info().await {
        Ok(answer) => println!("Futures Exchange information: {:?}", answer),
        Err(e) => println!("Error getting futures exchange info: {:?}", e),
    }

    match general.get_symbol_info("btcusdt").await {
        Ok(answer) => println!("Futures Symbol information: {:?}", answer),
        Err(e) => println!("Error getting futures symbol info: {:?}", e),
    }
    Ok(())
}

async fn market_data() -> Result<()> {
    let market: FuturesMarket = Binance::new(None, None);

    match market.get_depth("btcusdt").await {
        Ok(answer) => println!("Futures Depth update ID: {:?}", answer.last_update_id),
        Err(e) => println!("Error getting futures depth: {:?}", e),
    }

    match market.get_trades("btcusdt").await {
        Ok(Trades::AllTrades(answer)) => {
            if let Some(trade) = answer.first() {
                println!("Futures First trade: {:?}", trade);
            } else {
                println!("No trades found for BTCUSDT futures.");
            }
        },
        Err(e) => println!("Error getting futures trades: {:?}", e),
    }

    match market.get_agg_trades("btcusdt", None, None, None, None).await {
        Ok(AggTrades::AllAggTrades(answer)) => {
             if let Some(agg_trade) = answer.first() {
                println!("Futures First aggregated trade: {:?}", agg_trade);
            } else {
                println!("No aggregated trades found for BTCUSDT futures.");
            }
        },
        Err(e) => println!("Error getting futures agg_trades: {:?}", e),
    }

    match market.get_klines("btcusdt", "5m", 10, None, None).await {
        Ok(KlineSummaries::AllKlineSummaries(answer)) => {
            if let Some(kline) = answer.first() {
                println!("Futures First kline: {:?}", kline);
            } else {
                println!("No klines found for BTCUSDT futures.");
            }
        },
        Err(e) => println!("Error getting futures klines: {:?}", e),
    }

    match market.get_24h_price_stats("btcusdt").await {
        Ok(answer) => println!("Futures 24hr price stats: {:?}", answer),
        Err(e) => println!("Error getting futures 24h price stats: {:?}", e),
    }

    match market.get_price("btcusdt").await {
        Ok(answer) => println!("Futures Price: {:?}", answer),
        Err(e) => println!("Error getting futures price: {:?}", e),
    }

    match market.get_all_book_tickers().await {
        Ok(BookTickers::AllBookTickers(answer)) => {
            if let Some(ticker) = answer.first() {
                println!("Futures First book ticker: {:?}", ticker);
            } else {
                println!("No book tickers found for futures.");
            }
        },
        Err(e) => println!("Error getting all futures book tickers: {:?}", e),
    }

    match market.get_book_ticker("btcusdt").await {
        Ok(answer) => println!("Futures Book ticker: {:?}", answer),
        Err(e) => println!("Error getting futures book ticker: {:?}", e),
    }

    match market.get_mark_prices().await {
        Ok(MarkPrices::AllMarkPrices(answer)) => {
            if let Some(mark_price) = answer.first() {
                println!("Futures First mark Prices: {:?}", mark_price);
            } else {
                println!("No mark prices found for futures.");
            }
        },
        Err(e) => println!("Error getting futures mark prices: {:?}", e),
    }

    match market.get_all_liquidation_orders().await {
        Ok(LiquidationOrders::AllLiquidationOrders(answer)) => {
            if let Some(liq_order) = answer.first() {
                println!("Futures First liquidation order: {:?}", liq_order);
            } else {
                println!("No liquidation orders found for futures.");
            }
        }
        Err(e) => println!("Error getting futures liquidation orders: {:?}", e),
    }

    match market.open_interest("btcusdt").await {
        Ok(answer) => println!("Futures Open interest: {:?}", answer),
        Err(e) => println!("Error getting futures open interest: {:?}", e),
    }
    Ok(())
}
