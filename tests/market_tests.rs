use binance_rs_plus::api::*;
use binance_rs_plus::config::*;
use binance_rs_plus::market::*;
use binance_rs_plus::model::*;

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{Server, Matcher};
    use float_cmp::*;

    #[tokio::test] // Changed
    async fn get_depth() {
        // async added
        let mut server = Server::new_async().await; // async added
        let mock_get_depth = server
            .mock("GET", "/api/v3/depth")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex("symbol=LTCBTC".into()))
            .with_body_from_file("tests/mocks/market/get_depth.json")
            .create_async()
            .await; // async added

        let config = Config::default().set_rest_api_endpoint(server.url());
        let market: Market = Binance::new_with_config(None, None, &config);

        let order_book = market.get_depth("LTCBTC").await.unwrap(); // .await added
        mock_get_depth.assert();

        assert_eq!(order_book.last_update_id, 1027024);
        assert_eq!(order_book.bids[0], Bids::new(4.00000000, 431.00000000));
    }

    #[tokio::test] // Changed
    async fn get_custom_depth() {
        // async added
        let mut server = Server::new_async().await; // async added
        let mock_get_custom_depth = server
            .mock("GET", "/api/v3/depth")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex("limit=10&symbol=LTCBTC".into()))
            .with_body_from_file("tests/mocks/market/get_depth.json")
            .create_async()
            .await; // async added

        let config = Config::default().set_rest_api_endpoint(server.url());
        let market: Market = Binance::new_with_config(None, None, &config);

        let order_book = market.get_custom_depth("LTCBTC", 10).await.unwrap(); // .await added
        mock_get_custom_depth.assert();

        assert_eq!(order_book.last_update_id, 1027024);
        assert_eq!(order_book.bids[0], Bids::new(4.00000000, 431.00000000));
    }

    #[tokio::test] // Changed
    async fn get_all_prices() {
        // async added
        let mut server = Server::new_async().await; // async added
        let mock_get_all_prices = server
            .mock("GET", "/api/v3/ticker/price")
            .with_header("content-type", "application/json;charset=UTF-8")
            .with_body_from_file("tests/mocks/market/get_all_prices.json")
            .create_async()
            .await; // async added

        let config = Config::default().set_rest_api_endpoint(server.url());
        let market: Market = Binance::new_with_config(None, None, &config);

        let prices: Prices = market.get_all_prices().await.unwrap(); // .await added
        mock_get_all_prices.assert();

        match prices {
            binance_rs_plus::model::Prices::AllPrices(symbols) => {
                assert!(!symbols.is_empty());
                let first_symbol = symbols[0].clone();
                assert_eq!(first_symbol.symbol, "LTCBTC");
                assert!(approx_eq!(f64, first_symbol.price, 4.00000200, ulps = 2));
                let second_symbol = symbols[1].clone();
                assert_eq!(second_symbol.symbol, "ETHBTC");
                assert!(approx_eq!(f64, second_symbol.price, 0.07946600, ulps = 2));
            }
        }
    }

    #[tokio::test] // Changed
    async fn get_price() {
        // async added
        let mut server = Server::new_async().await; // async added
        let mock_get_price = server
            .mock("GET", "/api/v3/ticker/price")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex("symbol=LTCBTC".into()))
            .with_body_from_file("tests/mocks/market/get_price.json")
            .create_async()
            .await; // async added

        let config = Config::default().set_rest_api_endpoint(server.url());
        let market: Market = Binance::new_with_config(None, None, &config);

        let symbol = market.get_price("LTCBTC").await.unwrap(); // .await added
        mock_get_price.assert();

        assert_eq!(symbol.symbol, "LTCBTC");
        assert!(approx_eq!(f64, symbol.price, 4.00000200, ulps = 2));
    }

    #[tokio::test] // Changed
    async fn get_average_price() {
        // async added
        let mut server = Server::new_async().await; // async added
        let mock_get_average_price = server
            .mock("GET", "/api/v3/avgPrice")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex("symbol=LTCBTC".into()))
            .with_body_from_file("tests/mocks/market/get_average_price.json")
            .create_async()
            .await; // async added

        let config = Config::default().set_rest_api_endpoint(server.url());
        let market: Market = Binance::new_with_config(None, None, &config);

        let symbol = market.get_average_price("LTCBTC").await.unwrap(); // .await added
        mock_get_average_price.assert();

        assert_eq!(symbol.mins, 5);
        assert!(approx_eq!(f64, symbol.price, 9.35751834, ulps = 2));
    }

    #[tokio::test] // Changed
    async fn get_all_book_tickers() {
        // async added
        let mut server = Server::new_async().await; // async added
        let mock_get_all_book_tickers = server
            .mock("GET", "/api/v3/ticker/bookTicker")
            .with_header("content-type", "application/json;charset=UTF-8")
            .with_body_from_file("tests/mocks/market/get_all_book_tickers.json")
            .create_async()
            .await; // async added

        let config = Config::default().set_rest_api_endpoint(server.url());
        let market: Market = Binance::new_with_config(None, None, &config);

        let book_tickers = market.get_all_book_tickers().await.unwrap(); // .await added
        mock_get_all_book_tickers.assert();

        match book_tickers {
            binance_rs_plus::model::BookTickers::AllBookTickers(tickers) => {
                assert!(!tickers.is_empty());
                let first_ticker = tickers[0].clone();
                assert_eq!(first_ticker.symbol, "LTCBTC");
                assert!(approx_eq!(
                    f64,
                    first_ticker.bid_price,
                    4.00000000,
                    ulps = 2
                ));
                assert!(approx_eq!(
                    f64,
                    first_ticker.bid_qty,
                    431.00000000,
                    ulps = 2
                ));
                assert!(approx_eq!(
                    f64,
                    first_ticker.ask_price,
                    4.00000200,
                    ulps = 2
                ));
                assert!(approx_eq!(f64, first_ticker.ask_qty, 9.00000000, ulps = 2));
                let second_ticker = tickers[1].clone();
                assert_eq!(second_ticker.symbol, "ETHBTC");
                assert!(approx_eq!(
                    f64,
                    second_ticker.bid_price,
                    0.07946700,
                    ulps = 2
                ));
                assert!(approx_eq!(f64, second_ticker.bid_qty, 9.00000000, ulps = 2));
                assert!(approx_eq!(
                    f64,
                    second_ticker.ask_price,
                    100000.00000000,
                    ulps = 2
                ));
                assert!(approx_eq!(
                    f64,
                    second_ticker.ask_qty,
                    1000.00000000,
                    ulps = 2
                ));
            }
        }
    }

    #[tokio::test] // Changed
    async fn get_book_ticker() {
        // async added
        let mut server = Server::new_async().await; // async added
        let mock_get_book_ticker = server
            .mock("GET", "/api/v3/ticker/bookTicker")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex("symbol=LTCBTC".into()))
            .with_body_from_file("tests/mocks/market/get_book_ticker.json")
            .create_async()
            .await; // async added

        let config = Config::default().set_rest_api_endpoint(server.url());
        let market: Market = Binance::new_with_config(None, None, &config);

        let book_ticker = market.get_book_ticker("LTCBTC").await.unwrap(); // .await added
        mock_get_book_ticker.assert();

        assert_eq!(book_ticker.symbol, "LTCBTC");
        assert!(approx_eq!(f64, book_ticker.bid_price, 4.00000000, ulps = 2));
        assert!(approx_eq!(f64, book_ticker.bid_qty, 431.00000000, ulps = 2));
        assert!(approx_eq!(f64, book_ticker.ask_price, 4.00000200, ulps = 2));
        assert!(approx_eq!(f64, book_ticker.ask_qty, 9.00000000, ulps = 2));
    }

    #[tokio::test] // Changed
    async fn get_24h_price_stats() {
        // async added
        let mut server = Server::new_async().await; // async added
        let mock_get_24h_price_stats = server
            .mock("GET", "/api/v3/ticker/24hr")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex("symbol=BNBBTC".into()))
            .with_body_from_file("tests/mocks/market/get_24h_price_stats.json")
            .create_async()
            .await; // async added

        let config = Config::default().set_rest_api_endpoint(server.url());
        let market: Market = Binance::new_with_config(None, None, &config);

        let price_stats = market.get_24h_price_stats("BNBBTC").await.unwrap(); // .await added
        mock_get_24h_price_stats.assert();

        assert_eq!(price_stats.symbol, "BNBBTC");
        assert_eq!(price_stats.price_change, "-94.99999800");
        assert_eq!(price_stats.price_change_percent, "-95.960");
        assert_eq!(price_stats.weighted_avg_price, "0.29628482");
        assert!(approx_eq!(
            f64,
            price_stats.prev_close_price,
            0.10002000,
            ulps = 2
        ));
        assert!(approx_eq!(
            f64,
            price_stats.last_price,
            4.00000200,
            ulps = 2
        ));
        assert!(approx_eq!(f64, price_stats.bid_price, 4.00000000, ulps = 2));
        assert!(approx_eq!(f64, price_stats.ask_price, 4.00000200, ulps = 2));
        assert!(approx_eq!(
            f64,
            price_stats.open_price,
            99.00000000,
            ulps = 2
        ));
        assert!(approx_eq!(
            f64,
            price_stats.high_price,
            100.00000000,
            ulps = 2
        ));
        assert!(approx_eq!(f64, price_stats.low_price, 0.10000000, ulps = 2));
        assert!(approx_eq!(f64, price_stats.volume, 8913.30000000, ulps = 2));
        assert_eq!(price_stats.open_time, 1499783499040);
        assert_eq!(price_stats.close_time, 1499869899040);
        assert_eq!(price_stats.first_id, 28385);
        assert_eq!(price_stats.last_id, 28460);
        assert_eq!(price_stats.count, 76);
    }

    #[tokio::test] // Changed
    async fn get_all_24h_price_stats() {
        // async added
        let mut server = Server::new_async().await; // async added
        let mock_get_all_24h_price_stats = server
            .mock("GET", "/api/v3/ticker/24hr")
            .with_header("content-type", "application/json;charset=UTF-8")
            .with_body_from_file("tests/mocks/market/get_all_24h_price_stats.json")
            .create_async()
            .await; // async added

        let config = Config::default().set_rest_api_endpoint(server.url());
        let market: Market = Binance::new_with_config(None, None, &config);

        let prices_stats = market.get_all_24h_price_stats().await.unwrap(); // .await added
        mock_get_all_24h_price_stats.assert();

        assert!(!prices_stats.is_empty());

        let price_stats = prices_stats[0].clone();

        assert_eq!(price_stats.symbol, "BNBBTC");
        // ... (rest of assertions remain the same)
        assert_eq!(price_stats.price_change, "-94.99999800");
        assert_eq!(price_stats.price_change_percent, "-95.960");
        assert_eq!(price_stats.weighted_avg_price, "0.29628482");
        assert!(approx_eq!(
            f64,
            price_stats.prev_close_price,
            0.10002000,
            ulps = 2
        ));
        assert!(approx_eq!(
            f64,
            price_stats.last_price,
            4.00000200,
            ulps = 2
        ));
        assert!(approx_eq!(f64, price_stats.bid_price, 4.00000000, ulps = 2));
        assert!(approx_eq!(f64, price_stats.ask_price, 4.00000200, ulps = 2));
        assert!(approx_eq!(
            f64,
            price_stats.open_price,
            99.00000000,
            ulps = 2
        ));
        assert!(approx_eq!(
            f64,
            price_stats.high_price,
            100.00000000,
            ulps = 2
        ));
        assert!(approx_eq!(f64, price_stats.low_price, 0.10000000, ulps = 2));
        assert!(approx_eq!(f64, price_stats.volume, 8913.30000000, ulps = 2));
        assert_eq!(price_stats.open_time, 1499783499040);
        assert_eq!(price_stats.close_time, 1499869899040);
        assert_eq!(price_stats.first_id, 28385);
        assert_eq!(price_stats.last_id, 28460);
        assert_eq!(price_stats.count, 76);
    }

    #[tokio::test] // Changed
    async fn get_klines() {
        // async added
        let mut server = Server::new_async().await; // async added
        let mock_get_klines = server
            .mock("GET", "/api/v3/klines")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex("interval=5m&limit=10&symbol=LTCBTC".into()))
            .with_body_from_file("tests/mocks/market/get_klines.json")
            .create_async()
            .await; // async added

        let config = Config::default().set_rest_api_endpoint(server.url());
        let market: Market = Binance::new_with_config(None, None, &config);

        let klines = market
            .get_klines("LTCBTC", "5m", Some(10), None, None)
            .await
            .unwrap(); // .await added, Some for limit
        mock_get_klines.assert();

        match klines {
            binance_rs_plus::model::KlineSummaries::AllKlineSummaries(klines_vec) => {
                // Renamed klines to klines_vec
                assert!(!klines_vec.is_empty());
                let kline: KlineSummary = klines_vec[0].clone();

                assert_eq!(kline.open_time, 1499040000000);
                assert_eq!(kline.open, "0.01634790");
                assert_eq!(kline.high, "0.80000000");
                assert_eq!(kline.low, "0.01575800");
                assert_eq!(kline.close, "0.01577100");
                assert_eq!(kline.volume, "148976.11427815");
                assert_eq!(kline.close_time, 1499644799999);
                assert_eq!(kline.quote_asset_volume, "2434.19055334");
                assert_eq!(kline.number_of_trades, 308);
                assert_eq!(kline.taker_buy_base_asset_volume, "1756.87402397");
                assert_eq!(kline.taker_buy_quote_asset_volume, "28.46694368");
            }
        }
    }
}
