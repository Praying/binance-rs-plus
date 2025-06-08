use crate::util::build_signed_request;
use crate::model::{AssetDetail, CoinInfo, DepositAddress, SpotFuturesTransferType, TransactionId};
use crate::client::Client;
use crate::errors::Result; // Error struct is implicitly available via crate::errors::Error if needed
use std::collections::BTreeMap;
use crate::api::API;
use crate::api::Sapi;

#[derive(Clone)]
pub struct Savings {
    pub client: Client,
    pub recv_window: u64,
}

impl Savings {
    /// Get all coins available for deposit and withdrawal
    pub async fn get_all_coins(&self) -> Result<Vec<CoinInfo>> { // async added
        let request = build_signed_request(BTreeMap::new(), self.recv_window)?;
        self.client
            .get_signed(API::Savings(Sapi::AllCoins), Some(request))
            .await // .await added
    }

    /// Fetch details of assets supported on Binance.
    pub async fn asset_detail(&self, asset: Option<String>) -> Result<BTreeMap<String, AssetDetail>> { // async added
        let mut parameters = BTreeMap::new();
        if let Some(asset_str) = asset { // Renamed to avoid conflict
            parameters.insert("asset".into(), asset_str);
        }
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Savings(Sapi::AssetDetail), Some(request))
            .await // .await added
    }

    /// Fetch deposit address with network.
    ///
    /// You can get the available networks using `get_all_coins`.
    /// If no network is specified, the address for the default network is returned.
    pub async fn deposit_address<S>(&self, coin: S, network: Option<String>) -> Result<DepositAddress> // async added
    where
        S: Into<String>,
    {
        let mut parameters = BTreeMap::new();
        parameters.insert("coin".into(), coin.into());
        if let Some(network_str) = network { // Renamed to avoid conflict
            parameters.insert("network".into(), network_str);
        }
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Savings(Sapi::DepositAddress), Some(request))
            .await // .await added
    }

    pub async fn transfer_funds<S>( // async added
        &self, asset: S, amount: f64, transfer_type: SpotFuturesTransferType,
    ) -> Result<TransactionId>
    where
        S: Into<String>,
    {
        let mut parameters = BTreeMap::new();
        parameters.insert("asset".into(), asset.into());
        parameters.insert("amount".into(), amount.to_string());
        parameters.insert("type".into(), (transfer_type as u8).to_string());
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .post_signed(API::Savings(Sapi::SpotFuturesTransfer), request)
            .await // .await added
    }
}
