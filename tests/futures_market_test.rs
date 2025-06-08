use binance_rs_plus::api::*;
use binance_rs_plus::config::*;
use binance_rs_plus::futures::market::FuturesMarket;
use binance_rs_plus::futures::model::OpenInterestHist;

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{Server, Matcher};

    #[tokio::test] // Changed
    async fn open_interest_statistics() { // async added
        let mut server = Server::new_async().await; // async added
        let mock_open_interest_statistics = server
            .mock("GET", "/futures/data/openInterestHist")
            .with_header("content-type", "application/json;charset=UTF-8")
            // Note: This is a public endpoint as per Binance docs, so signature matching might not be needed.
            // If it were a signed endpoint, the Matcher::Regex would need to include &timestamp=\\d+&signature=.*
            .match_query(Matcher::Regex("limit=10&period=5m&symbol=BTCUSDT".into()))
            .with_body_from_file("tests/mocks/futures/market/open_interest_statistics.json")
            .create_async().await; // async added

        let config = Config::default().set_futures_rest_api_endpoint(server.url());
        let market: FuturesMarket = Binance::new_with_config(None, None, &config);

        let open_interest_hists = market
            .open_interest_statistics("BTCUSDT", "5m", Some(10), None, None) // Added Some() for limit
            .await // .await added
            .unwrap();
        mock_open_interest_statistics.assert();

        let expectation = vec![
            OpenInterestHist {
                symbol: "BTCUSDT".into(),
                sum_open_interest: "20403.63700000".into(),
                sum_open_interest_value: "150570784.07809979".into(),
                timestamp: 1583127900000,
            },
            OpenInterestHist {
                symbol: "BTCUSDT".into(),
                sum_open_interest: "20401.36700000".into(),
                sum_open_interest_value: "149940752.14464448".into(),
                timestamp: 1583128200000,
            },
        ];

        assert_eq!(open_interest_hists, expectation);
    }
}
