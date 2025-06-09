use binance_rs_plus::api::*;
use binance_rs_plus::config::*;
use binance_rs_plus::futures::account::*;

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{Server, Matcher};
    use float_cmp::*;
    use binance_rs_plus::account::OrderSide; // Assuming this OrderSide is compatible or aliased correctly
    use binance_rs_plus::futures::model::{Transaction, Income}; // Added Income

    #[tokio::test] // Changed
    async fn change_initial_leverage() {
        // async added
        let mut server = Server::new_async().await; // async added
        let mock_change_leverage = server
            .mock("POST", "/fapi/v1/leverage")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex(
                "leverage=2&recvWindow=1234&symbol=LTCUSDT&timestamp=\\d+&signature=.*".into(),
            ))
            .with_body_from_file("tests/mocks/futures/account/change_initial_leverage.json")
            .create_async()
            .await; // async added

        let config = Config::default()
            .set_futures_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: FuturesAccount = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        let response = account.change_initial_leverage("LTCUSDT", 2).await.unwrap(); // .await added

        mock_change_leverage.assert();

        assert_eq!(response.leverage, 2);
        assert_eq!(response.symbol, "LTCUSDT");
        assert!(approx_eq!(
            f64,
            response.max_notional_value,
            9223372036854776000.0, // This looks like a very large number, ensure it's correct from API
            ulps = 2
        ));
    }

    #[tokio::test] // Changed
    async fn change_margin_type() {
        // async added
        let mut server = Server::new_async().await; // async added
        let mock = server
            .mock("POST", "/fapi/v1/marginType")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex(
                "marginType=ISOLATED&recvWindow=1234&symbol=BTCUSDT&timestamp=\\d+&signature=.*"
                    .into(),
            ))
            .with_body_from_file("tests/mocks/futures/account/change_margin_type.json") // This mock should return an empty JSON object {} for success
            .create_async()
            .await; // async added

        let config = Config::default()
            .set_futures_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: FuturesAccount = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        account.change_margin_type("BTCUSDT", true).await.unwrap(); // .await added

        mock.assert();
    }

    #[tokio::test] // Changed
    async fn change_position_margin() {
        // async added
        let mut server = Server::new_async().await; // async added
        let mock = server
            .mock("POST", "/fapi/v1/positionMargin")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex(
                // The original mock was for amount=100 and type=1 (ADD_MARGIN)
                // If the function signature changed `true` to `1` for type, this is okay.
                // The function `change_position_margin` now takes amount: f64, margin_type: u8
                // Assuming `true` was meant to be `1` for ADD_MARGIN.
                "amount=100&recvWindow=1234&symbol=BTCUSDT&timestamp=\\d+&type=1&signature=.*"
                    .into(),
            ))
            // This mock should return an empty JSON object {} or specific success/error response
            .with_body_from_file("tests/mocks/futures/account/change_position_margin.json")
            .create_async()
            .await; // async added

        let config = Config::default()
            .set_futures_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: FuturesAccount = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        account
            .change_position_margin("BTCUSDT", 100.0, 1) // Assuming 1 means ADD_MARGIN
            .await // .await added
            .unwrap();

        mock.assert();
    }

    #[tokio::test] // Changed
    async fn cancel_all_open_orders() {
        // async added
        let mut server = Server::new_async().await; // async added
        let mock = server
            .mock("DELETE", "/fapi/v1/allOpenOrders")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex(
                "recvWindow=1234&symbol=BTCUSDT&timestamp=\\d+&signature=.*".into(),
            ))
            .with_body_from_file("tests/mocks/futures/account/cancel_all_open_orders.json") // This should return success or error code
            .create_async()
            .await; // async added

        let config = Config::default()
            .set_futures_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: FuturesAccount = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        account.cancel_all_open_orders("BTCUSDT").await.unwrap(); // .await added

        mock.assert();
    }

    #[tokio::test] // Changed
    async fn change_position_mode() {
        // async added
        let mut server = Server::new_async().await; // async added
        let mock = server
            .mock("POST", "/fapi/v1/positionSide/dual")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex(
                "dualSidePosition=true&recvWindow=1234&timestamp=\\d+&signature=.*".into(),
            ))
            .with_body_from_file("tests/mocks/futures/account/change_position_mode.json") // Should return success/error
            .create_async()
            .await; // async added

        let config = Config::default()
            .set_futures_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: FuturesAccount = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        account.change_position_mode(true).await.unwrap(); // .await added

        mock.assert();
    }

    #[tokio::test] // Changed
    async fn stop_market_close_buy() {
        // async added
        let mut server = Server::new_async().await; // async added
        let mock_stop_market_close_buy = server.mock("POST", "/fapi/v1/order") // Renamed for clarity
            .with_header("content-type", "application/json;charset=UTF-8")
            // Regex needs to match query params built by build_order_params now
            .match_query(Matcher::Regex("closePosition=true&recvWindow=1234&side=BUY&stopPrice=10.5&symbol=SRMUSDT&timestamp=\\d+&type=STOP_MARKET&signature=.*".into()))
            .with_body_from_file("tests/mocks/futures/account/stop_market_close_position_buy.json")
            .create_async().await; // async added

        let config = Config::default()
            .set_futures_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: FuturesAccount = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        let transaction: Transaction = account
            .stop_market_close_buy("SRMUSDT", 10.5)
            .await
            .unwrap(); // .await added

        mock_stop_market_close_buy.assert();

        assert_eq!(transaction.symbol, "SRMUSDT");
        assert_eq!(transaction.side, "BUY"); // From the perspective of the executed order after stop is triggered
        assert_eq!(transaction.orig_type, "STOP_MARKET");
        assert!(transaction.close_position);
        assert!(approx_eq!(f64, transaction.stop_price, 10.5, ulps = 2));
    }

    #[tokio::test] // Changed
    async fn stop_market_close_sell() {
        // async added
        let mut server = Server::new_async().await; // async added
        let mock_stop_market_close_sell = server.mock("POST", "/fapi/v1/order")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex("closePosition=true&recvWindow=1234&side=SELL&stopPrice=7.4&symbol=SRMUSDT&timestamp=\\d+&type=STOP_MARKET&signature=.*".into()))
            .with_body_from_file("tests/mocks/futures/account/stop_market_close_position_sell.json")
            .create_async().await; // async added

        let config = Config::default()
            .set_futures_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: FuturesAccount = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        let transaction: Transaction = account
            .stop_market_close_sell("SRMUSDT", 7.4)
            .await
            .unwrap(); // .await added

        mock_stop_market_close_sell.assert();

        assert_eq!(transaction.symbol, "SRMUSDT");
        assert_eq!(transaction.side, "SELL");
        assert_eq!(transaction.orig_type, "STOP_MARKET");
        assert!(transaction.close_position);
        assert!(approx_eq!(f64, transaction.stop_price, 7.4, ulps = 2));
    }

    #[tokio::test] // Changed
    async fn custom_order() {
        // async added
        let mut server = Server::new_async().await; // async added
        // This test used the same mock as stop_market_close_sell, if custom_order builds the same query, it's fine.
        let mock_custom_order = server.mock("POST", "/fapi/v1/order")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex("closePosition=true&recvWindow=1234&side=SELL&stopPrice=7.4&symbol=SRMUSDT&timestamp=\\d+&type=STOP_MARKET&signature=.*".into()))
            .with_body_from_file("tests/mocks/futures/account/stop_market_close_position_sell.json") // Assuming this mock fits
            .create_async().await; // async added

        let config = Config::default()
            .set_futures_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: FuturesAccount = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        let custom_order_request = CustomOrderRequest {
            // Renamed for clarity
            symbol: "SRMUSDT".into(),
            side: OrderSide::Sell, // Ensure this is the correct OrderSide from crate::account
            position_side: None,
            order_type: binance_rs_plus::futures::account::OrderType::StopMarket, // Use qualified OrderType
            time_in_force: None,
            quantity: None, // Changed from qty
            reduce_only: None,
            price: None,
            new_client_order_id: None,
            stop_price: Some(7.4),
            close_position: Some(true),
            activation_price: None,
            callback_rate: None,
            working_type: None,
            price_protect: None,
        };
        let transaction: Transaction = account.custom_order(custom_order_request).await.unwrap(); // .await added

        mock_custom_order.assert();

        assert_eq!(transaction.symbol, "SRMUSDT");
        assert_eq!(transaction.side, "SELL");
        assert_eq!(transaction.orig_type, "STOP_MARKET");
        assert!(transaction.close_position);
        assert!(approx_eq!(f64, transaction.stop_price, 7.4, ulps = 2));
    }

    #[tokio::test] // Changed
    async fn get_income() {
        // async added
        let mut server = Server::new_async().await; // async added
        let mock = server
            .mock("GET", "/fapi/v1/income")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex(
                "endTime=12345678910&incomeType=TRANSFER&limit=10\
                &recvWindow=1234&startTime=12345678910&symbol=BTCUSDT&timestamp=\\d+&signature=.*" // Added signature
                    .into(),
            ))
            .with_body_from_file("tests/mocks/futures/account/get_income_history.json")
            .create_async()
            .await; // async added

        let config = Config::default()
            .set_futures_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: FuturesAccount = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        let income_request = IncomeRequest {
            symbol: Some("BTCUSDT".into()),
            income_type: Some(IncomeType::TRANSFER),
            start_time: Some(12345678910),
            end_time: Some(12345678910),
            limit: Some(10),
        };
        let income_history: Vec<Income> = account.get_income(income_request).await.unwrap(); // .await added

        mock.assert();
        assert!(!income_history.is_empty());
        // Add more specific assertions for income history if needed
    }
}
