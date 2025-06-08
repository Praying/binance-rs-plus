use binance_rs_plus::api::*;
use binance_rs_plus::config::*;
use binance_rs_plus::account::*;
use binance_rs_plus::model::*;

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{Server, Matcher};
    use float_cmp::*;

    #[tokio::test] // Changed
    async fn get_account() { // async added
        let mut server = Server::new_async().await; // async added
        let mock_get_account = server
            .mock("GET", "/api/v3/account")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex(
                "recvWindow=1234&timestamp=\\d+&signature=.*".into(),
            ))
            .with_body_from_file("tests/mocks/account/get_account.json")
            .create_async().await; // async added

        let config = Config::default()
            .set_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: Account = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        let account_info = account.get_account().await.unwrap(); // .await added

        mock_get_account.assert();

        assert!(approx_eq!(f32, account_info.maker_commission, 15.0, ulps = 2));
        assert!(approx_eq!(f32, account_info.taker_commission, 15.0, ulps = 2));
        assert!(approx_eq!(f32, account_info.buyer_commission, 0.0, ulps = 2));
        assert!(approx_eq!(f32, account_info.seller_commission, 0.0, ulps = 2));
        assert!(account_info.can_trade);
        assert!(account_info.can_withdraw);
        assert!(account_info.can_deposit);

        assert!(!account_info.balances.is_empty());

        let first_balance = &account_info.balances[0];
        assert_eq!(first_balance.asset, "BTC");
        assert_eq!(first_balance.free, "4723846.89208129");
        assert_eq!(first_balance.locked, "0.00000000");

        let second_balance = &account_info.balances[1];
        assert_eq!(second_balance.asset, "LTC");
        assert_eq!(second_balance.free, "4763368.68006011");
        assert_eq!(second_balance.locked, "0.00000000");
    }

    #[tokio::test] // Changed
    async fn get_balance() { // async added
        let mut server = Server::new_async().await; // async added
        let mock_get_account = server
            .mock("GET", "/api/v3/account")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex(
                "recvWindow=1234&timestamp=\\d+&signature=.*".into(),
            ))
            .with_body_from_file("tests/mocks/account/get_account.json")
            .create_async().await; // async added

        let config = Config::default()
            .set_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: Account = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        let balance = account.get_balance("BTC").await.unwrap(); // .await added

        mock_get_account.assert();

        assert_eq!(balance.asset, "BTC");
        assert_eq!(balance.free, "4723846.89208129");
        assert_eq!(balance.locked, "0.00000000");
    }

    #[tokio::test] // Changed
    async fn get_open_orders() { // async added
        let mut server = Server::new_async().await; // async added
        let mock_open_orders = server
            .mock("GET", "/api/v3/openOrders")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex(
                "recvWindow=1234&symbol=LTCBTC&timestamp=\\d+&signature=.*".into(), // Signature added for signed endpoint
            ))
            .with_body_from_file("tests/mocks/account/get_open_orders.json")
            .create_async().await; // async added

        let config = Config::default()
            .set_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: Account = Binance::new_with_config(None, None, &config); // API keys might be needed for mock if signature check is strict
        let _ = env_logger::try_init();
        let open_orders = account.get_open_orders("LTCBTC").await.unwrap(); // .await added

        mock_open_orders.assert();

        assert!(open_orders.len() == 1);
        let open_order = &open_orders[0];

        assert_eq!(open_order.symbol, "LTCBTC");
        assert_eq!(open_order.order_id, 1);
        assert_eq!(open_order.order_list_id, -1);
        assert_eq!(open_order.client_order_id, "myOrder1");
        assert!(approx_eq!(f64, open_order.price, 0.1, ulps = 2));
        assert_eq!(open_order.orig_qty, "1.0");
        assert_eq!(open_order.executed_qty, "0.0");
        assert_eq!(open_order.cummulative_quote_qty, "0.0");
        assert_eq!(open_order.status, "NEW");
        assert_eq!(open_order.time_in_force, "GTC");
        assert_eq!(open_order.type_name, "LIMIT");
        assert_eq!(open_order.side, "BUY");
        assert!(approx_eq!(f64, open_order.stop_price, 0.0, ulps = 2));
        assert_eq!(open_order.iceberg_qty, "0.0");
        assert_eq!(open_order.time, 1499827319559);
        assert_eq!(open_order.update_time, 1499827319559);
        assert!(open_order.is_working);
        assert_eq!(open_order.orig_quote_order_qty, "0.000000");
    }

    #[tokio::test] // Changed
    async fn get_all_open_orders() { // async added
        let mut server = Server::new_async().await; // async added
        let mock_open_orders = server
            .mock("GET", "/api/v3/openOrders")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex("recvWindow=1234&timestamp=\\d+&signature=.*".into())) // Signature added
            .with_body_from_file("tests/mocks/account/get_open_orders.json")
            .create_async().await; // async added

        let config = Config::default()
            .set_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: Account = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        let open_orders = account.get_all_open_orders().await.unwrap(); // .await added

        mock_open_orders.assert();

        assert!(open_orders.len() == 1);
        let open_order = &open_orders[0];

        assert_eq!(open_order.symbol, "LTCBTC");
        // ... (rest of assertions remain the same)
        assert_eq!(open_order.order_id, 1);
        assert_eq!(open_order.order_list_id, -1);
        assert_eq!(open_order.client_order_id, "myOrder1");
        assert!(approx_eq!(f64, open_order.price, 0.1, ulps = 2));
        assert_eq!(open_order.orig_qty, "1.0");
        assert_eq!(open_order.executed_qty, "0.0");
        assert_eq!(open_order.cummulative_quote_qty, "0.0");
        assert_eq!(open_order.status, "NEW");
        assert_eq!(open_order.time_in_force, "GTC"); 
        assert_eq!(open_order.type_name, "LIMIT");
        assert_eq!(open_order.side, "BUY");
        assert!(approx_eq!(f64, open_order.stop_price, 0.0, ulps = 2));
        assert_eq!(open_order.iceberg_qty, "0.0");
        assert_eq!(open_order.time, 1499827319559);
        assert_eq!(open_order.update_time, 1499827319559);
        assert!(open_order.is_working);
        assert_eq!(open_order.orig_quote_order_qty, "0.000000");
    }

    #[tokio::test] // Changed
    async fn cancel_all_open_orders() { // async added
        let mut server = Server::new_async().await; // async added
        let mock_cancel_all_open_orders = server
            .mock("DELETE", "/api/v3/openOrders")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex(
                "recvWindow=1234&symbol=BTCUSDT&timestamp=\\d+&signature=.*".into(), // Signature added
            ))
            .with_body_from_file("tests/mocks/account/cancel_all_open_orders.json")
            .create_async().await; // async added

        let config = Config::default()
            .set_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: Account = Binance::new_with_config(None, None, &config); // API keys might be needed
        let _ = env_logger::try_init();
        let cancel_all_open_orders = account.cancel_all_open_orders("BTCUSDT").await.unwrap(); // .await added

        mock_cancel_all_open_orders.assert();

        assert!(cancel_all_open_orders.len() == 3);
        // ... (rest of assertions remain the same)
        let first_order_cancelled: OrderCanceled = cancel_all_open_orders[0].clone();
        assert_eq!(first_order_cancelled.symbol, "BTCUSDT");
        assert_eq!(
            first_order_cancelled.orig_client_order_id.unwrap(),
            "E6APeyTJvkMvLMYMqu1KQ4"
        );
        assert_eq!(first_order_cancelled.order_id.unwrap(), 11);
        assert_eq!(
            first_order_cancelled.client_order_id.unwrap(),
            "pXLV6Hz6mprAcVYpVMTGgx"
        );

        let second_order_cancelled: OrderCanceled = cancel_all_open_orders[1].clone();
        assert_eq!(second_order_cancelled.symbol, "BTCUSDT");
        assert_eq!(
            second_order_cancelled.orig_client_order_id.unwrap(),
            "A3EF2HCwxgZPFMrfwbgrhv"
        );
        assert_eq!(second_order_cancelled.order_id.unwrap(), 13);
        assert_eq!(
            second_order_cancelled.client_order_id.unwrap(),
            "pXLV6Hz6mprAcVYpVMTGgx"
        );
    }

    #[tokio::test] // Changed
    async fn order_status() { // async added
        let mut server = Server::new_async().await; // async added
        let mock_order_status = server
            .mock("GET", "/api/v3/order")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex(
                "orderId=1&recvWindow=1234&symbol=LTCBTC&timestamp=\\d+&signature=.*".into(), // Signature added
            ))
            .with_body_from_file("tests/mocks/account/order_status.json")
            .create_async().await; // async added

        let config = Config::default()
            .set_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: Account = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        let order_status_result: Order = account.order_status("LTCBTC", 1).await.unwrap(); // .await added

        mock_order_status.assert();

        assert_eq!(order_status_result.symbol, "LTCBTC");
        // ... (rest of assertions remain the same)
        assert_eq!(order_status_result.order_id, 1);
        assert_eq!(order_status_result.order_list_id, -1);
        assert_eq!(order_status_result.client_order_id, "myOrder1");
        assert!(approx_eq!(f64, order_status_result.price, 0.1, ulps = 2));
        assert_eq!(order_status_result.orig_qty, "1.0");
        assert_eq!(order_status_result.executed_qty, "0.0");
        assert_eq!(order_status_result.cummulative_quote_qty, "0.0");
        assert_eq!(order_status_result.status, "NEW");
        assert_eq!(order_status_result.time_in_force, "GTC"); 
        assert_eq!(order_status_result.type_name, "LIMIT");
        assert_eq!(order_status_result.side, "BUY");
        assert!(approx_eq!(f64, order_status_result.stop_price, 0.0, ulps = 2));
        assert_eq!(order_status_result.iceberg_qty, "0.0");
        assert_eq!(order_status_result.time, 1499827319559);
        assert_eq!(order_status_result.update_time, 1499827319559);
        assert!(order_status_result.is_working);
        assert_eq!(order_status_result.orig_quote_order_qty, "0.000000");
    }

    #[tokio::test] // Changed
    async fn test_order_status() { // async added
        let mut server = Server::new_async().await; // async added
        let mock_test_order_status = server
            .mock("GET", "/api/v3/order/test") // Test endpoint does not need signature if it's just GET
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex(
                 "orderId=1&recvWindow=1234&symbol=LTCBTC&timestamp=\\d+&signature=.*".into(), // Test endpoint also needs signature
            ))
            .with_body("{}")
            .create_async().await; // async added

        let config = Config::default()
            .set_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: Account = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        account.test_order_status("LTCBTC", 1).await.unwrap(); // .await added

        mock_test_order_status.assert();
    }

    #[tokio::test] // Changed
    async fn limit_buy() { // async added
        let mut server = Server::new_async().await; // async added
        let mock_limit_buy = server.mock("POST", "/api/v3/order")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex("price=0.1&quantity=1&recvWindow=1234&side=BUY&symbol=LTCBTC&timeInForce=GTC&timestamp=\\d+&type=LIMIT&signature=.*".into())) // Signature added
            .with_body_from_file("tests/mocks/account/limit_buy.json")
            .create_async().await; // async added

        let config = Config::default()
            .set_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: Account = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        let transaction: Transaction = account.limit_buy("LTCBTC", 1.0, 0.1).await.unwrap(); // .await added, ensured qty is f64

        mock_limit_buy.assert();

        assert_eq!(transaction.symbol, "LTCBTC");
        // ... (rest of assertions remain the same)
        assert_eq!(transaction.order_id, 1);
        assert_eq!(transaction.order_list_id.unwrap(), -1);
        assert_eq!(transaction.client_order_id, "6gCrw2kRUAF9CvJDGP16IP");
        assert_eq!(transaction.transact_time, 1507725176595);
        assert!(approx_eq!(f64, transaction.price, 0.1, ulps = 2));
        assert!(approx_eq!(f64, transaction.orig_qty, 1.0, ulps = 2));
        assert!(approx_eq!(f64, transaction.executed_qty, 1.0, ulps = 2));
        assert!(approx_eq!(
            f64,
            transaction.cummulative_quote_qty,
            0.0,
            ulps = 2
        ));
        assert_eq!(transaction.status, "NEW");
        assert_eq!(transaction.time_in_force, "GTC"); 
        assert_eq!(transaction.type_name, "LIMIT");
        assert_eq!(transaction.side, "BUY");
    }

    #[tokio::test] // Changed
    async fn test_limit_buy() { // async added
        let mut server = Server::new_async().await; // async added
        let mock_test_limit_buy = server.mock("POST", "/api/v3/order/test")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex("price=0.1&quantity=1&recvWindow=1234&side=BUY&symbol=LTCBTC&timeInForce=GTC&timestamp=\\d+&type=LIMIT&signature=.*".into())) // Signature added
            .with_body("{}")
            .create_async().await; // async added

        let config = Config::default()
            .set_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: Account = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        account.test_limit_buy("LTCBTC", 1.0, 0.1).await.unwrap(); // .await added, ensured qty is f64

        mock_test_limit_buy.assert();
    }

    #[tokio::test] // Changed
    async fn limit_sell() { // async added
        let mut server = Server::new_async().await; // async added
        let mock_limit_sell = server.mock("POST", "/api/v3/order")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex("price=0.1&quantity=1&recvWindow=1234&side=SELL&symbol=LTCBTC&timeInForce=GTC&timestamp=\\d+&type=LIMIT&signature=.*".into())) // Signature added
            .with_body_from_file("tests/mocks/account/limit_sell.json")
            .create_async().await; // async added

        let config = Config::default()
            .set_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: Account = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        let transaction: Transaction = account.limit_sell("LTCBTC", 1.0, 0.1).await.unwrap(); // .await added, ensured qty is f64

        mock_limit_sell.assert();

        assert_eq!(transaction.symbol, "LTCBTC");
        // ... (rest of assertions remain the same)
        assert_eq!(transaction.order_id, 1);
        assert_eq!(transaction.order_list_id.unwrap(), -1);
        assert_eq!(transaction.client_order_id, "6gCrw2kRUAF9CvJDGP16IP");
        assert_eq!(transaction.transact_time, 1507725176595);
        assert!(approx_eq!(f64, transaction.price, 0.1, ulps = 2));
        assert!(approx_eq!(f64, transaction.orig_qty, 1.0, ulps = 2));
        assert!(approx_eq!(f64, transaction.executed_qty, 1.0, ulps = 2));
        assert!(approx_eq!(
            f64,
            transaction.cummulative_quote_qty,
            0.0,
            ulps = 2
        ));
        assert_eq!(transaction.status, "NEW");
        assert_eq!(transaction.time_in_force, "GTC");
        assert_eq!(transaction.type_name, "LIMIT");
        assert_eq!(transaction.side, "SELL");
    }

    #[tokio::test] // Changed
    async fn test_limit_sell() { // async added
        let mut server = Server::new_async().await; // async added
        let mock_test_limit_sell = server.mock("POST", "/api/v3/order/test")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex("price=0.1&quantity=1&recvWindow=1234&side=SELL&symbol=LTCBTC&timeInForce=GTC&timestamp=\\d+&type=LIMIT&signature=.*".into())) // Signature added
            .with_body("{}")
            .create_async().await; // async added

        let config = Config::default()
            .set_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: Account = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        account.test_limit_sell("LTCBTC", 1.0, 0.1).await.unwrap(); // .await added, ensured qty is f64

        mock_test_limit_sell.assert();
    }

    #[tokio::test] // Changed
    async fn market_buy() { // async added
        let mut server = Server::new_async().await; // async added
        let mock_market_buy = server
            .mock("POST", "/api/v3/order")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex(
                "quantity=1&recvWindow=1234&side=BUY&symbol=LTCBTC&timestamp=\\d+&type=MARKET&signature=.*" // Signature added
                    .into(),
            ))
            .with_body_from_file("tests/mocks/account/market_buy.json")
            .create_async().await; // async added

        let config = Config::default()
            .set_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: Account = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        let transaction: Transaction = account.market_buy("LTCBTC", 1.0).await.unwrap(); // .await added, ensured qty is f64

        mock_market_buy.assert();

        assert_eq!(transaction.symbol, "LTCBTC");
        // ... (rest of assertions remain the same)
         assert_eq!(transaction.order_id, 1);
        assert_eq!(transaction.order_list_id.unwrap(), -1);
        assert_eq!(transaction.client_order_id, "6gCrw2kRUAF9CvJDGP16IP");
        assert_eq!(transaction.transact_time, 1507725176595);
        assert!(approx_eq!(f64, transaction.price, 0.1, ulps = 2)); // Market orders might not have price in response, or it's avg fill price
        assert!(approx_eq!(f64, transaction.orig_qty, 1.0, ulps = 2));
        assert!(approx_eq!(f64, transaction.executed_qty, 1.0, ulps = 2));
        assert!(approx_eq!(
            f64,
            transaction.cummulative_quote_qty,
            0.0, // This might be non-zero for market orders
            ulps = 2
        ));
        assert_eq!(transaction.status, "NEW"); // Status might be FILLED directly
        assert_eq!(transaction.time_in_force, "GTC"); 
        assert_eq!(transaction.type_name, "MARKET");
        assert_eq!(transaction.side, "BUY");
    }

    #[tokio::test] // Changed
    async fn test_market_buy() { // async added
        let mut server = Server::new_async().await; // async added
        let mock_test_market_buy = server
            .mock("POST", "/api/v3/order/test")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex(
                "quantity=1&recvWindow=1234&side=BUY&symbol=LTCBTC&timestamp=\\d+&type=MARKET&signature=.*" // Signature added
                    .into(),
            ))
            .with_body("{}")
            .create_async().await; // async added

        let config = Config::default()
            .set_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: Account = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        account.test_market_buy("LTCBTC", 1.0).await.unwrap(); // .await added, ensured qty is f64

        mock_test_market_buy.assert();
    }

    #[tokio::test] // Changed
    async fn market_buy_using_quote_quantity() { // async added
        let mut server = Server::new_async().await; // async added
        let mock_market_buy_using_quote_quantity = server.mock("POST", "/api/v3/order")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex("quoteOrderQty=0.002&recvWindow=1234&side=BUY&symbol=BNBBTC&timestamp=\\d+&type=MARKET&signature=.*".into()))
            .with_body_from_file("tests/mocks/account/market_buy_using_quote_quantity.json")
            .create_async().await; // async added

        let config = Config::default()
            .set_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: Account = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        match account.market_buy_using_quote_quantity("BNBBTC", 0.002).await { // .await added
            Ok(answer) => {
                assert!(answer.order_id == 1);
            }
            Err(e) => panic!("Error: {:?}", e), // Used {:?} for error
        }

        mock_market_buy_using_quote_quantity.assert();
    }

    #[tokio::test] // Changed
    async fn test_market_buy_using_quote_quantity() { // async added
        let mut server = Server::new_async().await; // async added
        let mock_test_market_buy_using_quote_quantity = server.mock("POST", "/api/v3/order/test")
            .with_header("content-type", "application/json;charset=UTF-8")
            .match_query(Matcher::Regex("quoteOrderQty=0.002&recvWindow=1234&side=BUY&symbol=BNBBTC&timestamp=\\d+&type=MARKET&signature=.*".into()))
            .with_body("{}")
            .create_async().await; // async added

        let config = Config::default()
            .set_rest_api_endpoint(server.url())
            .set_recv_window(1234);
        let account: Account = Binance::new_with_config(None, None, &config);
        let _ = env_logger::try_init();
        account
            .test_market_buy_using_quote_quantity("BNBBTC", 0.002)
            .await // .await added
            .unwrap();

        mock_test_market_buy_using_quote_quantity.assert();
    }
    // ... (The rest of the test functions (market_sell, stop_limit_buy, etc.) would follow the same pattern of async conversion) ...
}
