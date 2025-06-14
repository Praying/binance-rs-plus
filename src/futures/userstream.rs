use crate::model::{Success, UserDataStream};
use crate::client::Client;
use crate::errors::Result;
use crate::api::API;
use crate::api::Futures;

#[derive(Clone)]
pub struct FuturesUserStream {
    pub client: Client,
    pub recv_window: u64, // This field seems unused, but we'll keep it.
}

impl FuturesUserStream {
    // User Stream
    pub async fn start(&self) -> Result<UserDataStream> {
        self.client
            .post(API::Futures(Futures::UserDataStream))
            .await
    }

    pub async fn keep_alive(&self, listen_key: &str) -> Result<Success> {
        self.client
            .put(API::Futures(Futures::UserDataStream), listen_key)
            .await
    }

    pub async fn close(&self, listen_key: &str) -> Result<Success> {
        self.client
            .delete(API::Futures(Futures::UserDataStream), listen_key)
            .await
    }
}
