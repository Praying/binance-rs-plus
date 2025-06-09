use crate::async_websocket_client::AsyncWebsocketClient;
use crate::config::Config;
use crate::errors::Result;
use crate::model::{
    AccountUpdateEvent, AggrTradesEvent, BalanceUpdateEvent, BookTickerEvent, DayTickerEvent,
    DepthOrderBookEvent, KlineEvent, OrderBook, OrderTradeEvent, TradeEvent, WindowTickerEvent,
};
// New

use serde::{Deserialize, Serialize};

use std::future::Future;
// New
use std::pin::Pin;
// New
use std::sync::atomic::AtomicBool;
// Ordering might be needed by user
use std::sync::Arc;
// New
use tokio::sync::Mutex as TokioMutex;
// New for handler

// WebsocketAPI enum remains the same
#[allow(clippy::all)]
enum WebsocketAPI {
    Default,
    MultiStream,
    Custom(String),
}

impl WebsocketAPI {
    fn params(
        self,
        subscription: &str,
    ) -> String {
        match self {
            WebsocketAPI::Default => format!("wss://stream.binance.com/ws/{}", subscription),
            WebsocketAPI::MultiStream => {
                format!("wss://stream.binance.com/stream?streams={}", subscription)
            }
            WebsocketAPI::Custom(url) => format!("{}/{}", url, subscription),
        }
    }
}

// WebsocketEvent enum remains the same - this is what the user's handler will receive
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WebsocketEvent {
    AccountUpdate(AccountUpdateEvent),
    BalanceUpdate(BalanceUpdateEvent),
    OrderTrade(OrderTradeEvent),
    AggrTrades(AggrTradesEvent),
    Trade(TradeEvent),
    OrderBook(OrderBook),
    DayTicker(DayTickerEvent),
    DayTickerAll(Vec<DayTickerEvent>),
    WindowTicker(WindowTickerEvent),
    WindowTickerAll(Vec<WindowTickerEvent>),
    Kline(KlineEvent),
    DepthOrderBook(DepthOrderBookEvent),
    BookTicker(BookTickerEvent),
}

// Events enum is what AsyncWebsocketClient will deserialize into (as E)
// This needs to be DeserializeOwned + Send + 'a
#[derive(Serialize, Deserialize, Debug, Clone)] // Added Clone for potential use, ensure it's Send + 'a compatible
#[serde(untagged)]
enum Events {
    DayTickerEventAll(Vec<DayTickerEvent>),
    WindowTickerEventAll(Vec<WindowTickerEvent>),
    BalanceUpdateEvent(BalanceUpdateEvent),
    DayTickerEvent(DayTickerEvent),
    WindowTickerEvent(WindowTickerEvent),
    BookTickerEvent(BookTickerEvent),
    AccountUpdateEvent(AccountUpdateEvent),
    OrderTradeEvent(OrderTradeEvent),
    AggrTradesEvent(AggrTradesEvent),
    TradeEvent(TradeEvent),
    KlineEvent(KlineEvent),
    OrderBook(OrderBook),
    DepthOrderBookEvent(DepthOrderBookEvent),
}

// Define the type for the adapter handler passed to AsyncWebsocketClient
type AdapterHandler<'a> = Box<
    dyn FnMut(Events) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> + Send + Sync + 'a,
>;

pub struct WebSockets<'a> {
    client: AsyncWebsocketClient<'a, Events, AdapterHandler<'a>>,
}

impl<'a> WebSockets<'a> {
    pub fn new<Callback>(user_handler: Callback) -> WebSockets<'a>
    where
        Callback: FnMut(WebsocketEvent) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>>
            + Send
            + Sync
            + 'a,
    {
        let shared_user_handler = Arc::new(TokioMutex::new(user_handler));

        let adapter_handler: AdapterHandler<'a> = Box::new(move |events_obj: Events| {
            let user_handler_clone = Arc::clone(&shared_user_handler);
            Box::pin(async move {
                let action = match events_obj {
                    Events::DayTickerEventAll(v) => WebsocketEvent::DayTickerAll(v),
                    Events::WindowTickerEventAll(v) => WebsocketEvent::WindowTickerAll(v),
                    Events::BalanceUpdateEvent(v) => WebsocketEvent::BalanceUpdate(v),
                    Events::DayTickerEvent(v) => WebsocketEvent::DayTicker(v),
                    Events::WindowTickerEvent(v) => WebsocketEvent::WindowTicker(v),
                    Events::BookTickerEvent(v) => WebsocketEvent::BookTicker(v),
                    Events::AccountUpdateEvent(v) => WebsocketEvent::AccountUpdate(v),
                    Events::OrderTradeEvent(v) => WebsocketEvent::OrderTrade(v),
                    Events::AggrTradesEvent(v) => WebsocketEvent::AggrTrades(v),
                    Events::TradeEvent(v) => WebsocketEvent::Trade(v),
                    Events::KlineEvent(v) => WebsocketEvent::Kline(v),
                    Events::OrderBook(v) => WebsocketEvent::OrderBook(v),
                    Events::DepthOrderBookEvent(v) => WebsocketEvent::DepthOrderBook(v),
                };
                let mut handler_guard = user_handler_clone.lock().await;
                (handler_guard)(action).await
            })
        });

        WebSockets {
            client: AsyncWebsocketClient::new(adapter_handler),
        }
    }

    pub async fn connect(
        &mut self,
        subscription: &str,
    ) -> Result<()> {
        self.client
            .connect(&WebsocketAPI::Default.params(subscription))
            .await
    }

    pub async fn connect_with_config(
        &mut self,
        subscription: &str,
        config: &Config,
    ) -> Result<()> {
        self.client
            .connect(&WebsocketAPI::Custom(config.ws_endpoint.clone()).params(subscription))
            .await
    }

    pub async fn connect_multiple_streams(
        &mut self,
        endpoints: &[String],
    ) -> Result<()> {
        self.client
            .connect(&WebsocketAPI::MultiStream.params(&endpoints.join("/")))
            .await
    }

    pub async fn disconnect(&mut self) -> Result<()> {
        self.client.disconnect().await
    }

    // event_loop now takes Arc<AtomicBool>
    pub async fn event_loop(
        &mut self,
        running: Arc<AtomicBool>,
    ) -> Result<()> {
        self.client.event_loop(running).await
    }
}
