use crate::model::{Empty, ExchangeInformation, ServerTime, Symbol};
use crate::client::Client;
use crate::errors::{Result, Error}; // Added Error
use crate::api::API;
use crate::api::Spot;

#[derive(Clone)]
pub struct General {
    pub client: Client,
}

impl General {
    // Test connectivity
    pub async fn ping(&self) -> Result<String> {
        // async added
        self.client
            .get::<Empty>(API::Spot(Spot::Ping), None)
            .await?; // .await? added
        Ok("pong".into())
    }

    // Check server time
    pub async fn get_server_time(&self) -> Result<ServerTime> {
        // async added
        self.client.get(API::Spot(Spot::Time), None).await // .await added
    }

    // Obtain exchange information
    // - Current exchange trading rules and symbol information
    pub async fn exchange_info(&self) -> Result<ExchangeInformation> {
        // async added
        self.client.get(API::Spot(Spot::ExchangeInfo), None).await // .await added
    }

    // Get Symbol information
    pub async fn get_symbol_info<S>(&self, symbol: S) -> Result<Symbol>
    // async added
    where
        S: Into<String>,
    {
        let upper_symbol = symbol.into().to_uppercase();
        match self.exchange_info().await {
            // .await added
            Ok(info) => {
                for item in info.symbols {
                    if item.symbol == upper_symbol {
                        return Ok(item);
                    }
                }
                Err(Error::Custom("Symbol not found".to_string())) // Replaced bail
            }
            Err(e) => Err(e),
        }
    }
}
