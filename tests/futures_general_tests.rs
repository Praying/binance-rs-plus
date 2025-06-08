use binance_rs_plus::api::*;
use binance_rs_plus::config::*;
use binance_rs_plus::futures::general::*;

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;

    #[tokio::test] // Changed
    async fn ping() { // async added
        let mut server = Server::new_async().await; // async added
        let mock_ping = server
            .mock("GET", "/fapi/v1/ping")
            .with_header("content-type", "application/json;charset=UTF-8")
            .with_body("{}")
            .create_async().await; // async added

        let config = Config::default().set_futures_rest_api_endpoint(server.url());
        
        let general: FuturesGeneral = Binance::new_with_config(None, None, &config);

        let pong = general.ping().await.unwrap(); // .await added
        mock_ping.assert();

        assert_eq!(pong, "pong");
    }
}
