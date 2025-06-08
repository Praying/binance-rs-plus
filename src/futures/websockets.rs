use crate::config::Config;
use crate::errors::Result;
use crate::futures::model as futures_model;
use crate::model::{
    AccountUpdateEvent, AggrTradesEvent, BookTickerEvent, ContinuousKlineEvent, DayTickerEvent,
    DepthOrderBookEvent, IndexKlineEvent, IndexPriceEvent, KlineEvent, LiquidationEvent,
    MarkPriceEvent, MiniTickerEvent, OrderBook, TradeEvent, UserDataStreamExpiredEvent,
};
// Alias for futures specific OrderTradeEvent
use crate::async_websocket_client::AsyncWebsocketClient;
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


#[allow(clippy::all)]
enum FuturesWebsocketAPI {
    Default,
    MultiStream,
    Custom(String),
}

pub enum FuturesMarket {
    USDM,
    COINM,
    Vanilla,
}

impl FuturesWebsocketAPI {
    fn params(self, market: &FuturesMarket, subscription: &str) -> String {
        let baseurl = match market {
            FuturesMarket::USDM => "wss://fstream.binance.com",
            FuturesMarket::COINM => "wss://dstream.binance.com",
            FuturesMarket::Vanilla => "wss://vstream.binance.com",
        };

        match self {
            FuturesWebsocketAPI::Default => {
                format!("{}/ws/{}", baseurl, subscription)
            }
            FuturesWebsocketAPI::MultiStream => {
                format!("{}/stream?streams={}", baseurl, subscription)
            }
            FuturesWebsocketAPI::Custom(url) => url, // Assuming custom URL is the full WSS URL
        }
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FuturesWebsocketEvent {
    AccountUpdate(AccountUpdateEvent),
    OrderTrade(futures_model::OrderTradeEvent), // Use aliased model
    AggrTrades(AggrTradesEvent),
    Trade(TradeEvent),
    OrderBook(OrderBook),
    DayTicker(DayTickerEvent),
    MiniTicker(MiniTickerEvent),
    MiniTickerAll(Vec<MiniTickerEvent>),
    IndexPrice(IndexPriceEvent),
    MarkPrice(MarkPriceEvent),
    MarkPriceAll(Vec<MarkPriceEvent>),
    DayTickerAll(Vec<DayTickerEvent>),
    Kline(KlineEvent),
    ContinuousKline(ContinuousKlineEvent),
    IndexKline(IndexKlineEvent),
    Liquidation(LiquidationEvent),
    DepthOrderBook(DepthOrderBookEvent),
    BookTicker(BookTickerEvent),
    UserDataStreamExpiredEvent(UserDataStreamExpiredEvent),
}

// Events enum is what AsyncWebsocketClient will deserialize into (as E)
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
enum FuturesEvents {
    VecDayTicker(Vec<DayTickerEvent>), // Renamed to avoid conflict if used directly in match
    DayTickerEvent(DayTickerEvent),
    BookTickerEvent(BookTickerEvent),
    MiniTickerEvent(MiniTickerEvent),
    VecMiniTickerEvent(Vec<MiniTickerEvent>),
    AccountUpdateEvent(AccountUpdateEvent),
    OrderTradeEvent(futures_model::OrderTradeEvent), // Use aliased model
    AggrTradesEvent(AggrTradesEvent),
    IndexPriceEvent(IndexPriceEvent),
    MarkPriceEvent(MarkPriceEvent),
    VecMarkPriceEvent(Vec<MarkPriceEvent>),
    TradeEvent(TradeEvent),
    KlineEvent(KlineEvent),
    ContinuousKlineEvent(ContinuousKlineEvent),
    IndexKlineEvent(IndexKlineEvent),
    LiquidationEvent(LiquidationEvent),
    OrderBook(OrderBook),
    DepthOrderBookEvent(DepthOrderBookEvent),
    UserDataStreamExpiredEvent(UserDataStreamExpiredEvent),
}


// Define the type for the user-provided handler
type UserCallback<'a> =
    Box<dyn FnMut(FuturesWebsocketEvent) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> + Send + Sync + 'a>;

// Define the type for the adapter handler passed to AsyncWebsocketClient
type AdapterHandler<'a> =
    Box<dyn FnMut(FuturesEvents) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> + Send + Sync + 'a>;


pub struct FuturesWebSockets<'a> {
    client: AsyncWebsocketClient<'a, FuturesEvents, AdapterHandler<'a>>,
}

impl<'a> FuturesWebSockets<'a> {
    pub fn new<Callback>(user_handler: Callback) -> FuturesWebSockets<'a>
    where
        Callback: FnMut(FuturesWebsocketEvent) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> + Send + Sync + 'a,
    {
        let shared_user_handler = Arc::new(TokioMutex::new(user_handler));

        let adapter_handler: AdapterHandler<'a> = Box::new(move |events_obj: FuturesEvents| {
            let user_handler_clone = Arc::clone(&shared_user_handler);
            Box::pin(async move {
                let action = match events_obj {
                    FuturesEvents::VecDayTicker(v) => FuturesWebsocketEvent::DayTickerAll(v),
                    FuturesEvents::DayTickerEvent(v) => FuturesWebsocketEvent::DayTicker(v),
                    FuturesEvents::BookTickerEvent(v) => FuturesWebsocketEvent::BookTicker(v),
                    FuturesEvents::MiniTickerEvent(v) => FuturesWebsocketEvent::MiniTicker(v),
                    FuturesEvents::VecMiniTickerEvent(v) => FuturesWebsocketEvent::MiniTickerAll(v),
                    FuturesEvents::AccountUpdateEvent(v) => FuturesWebsocketEvent::AccountUpdate(v),
                    FuturesEvents::OrderTradeEvent(v) => FuturesWebsocketEvent::OrderTrade(v),
                    FuturesEvents::IndexPriceEvent(v) => FuturesWebsocketEvent::IndexPrice(v),
                    FuturesEvents::MarkPriceEvent(v) => FuturesWebsocketEvent::MarkPrice(v),
                    FuturesEvents::VecMarkPriceEvent(v) => FuturesWebsocketEvent::MarkPriceAll(v),
                    FuturesEvents::TradeEvent(v) => FuturesWebsocketEvent::Trade(v),
                    FuturesEvents::ContinuousKlineEvent(v) => FuturesWebsocketEvent::ContinuousKline(v),
                    FuturesEvents::IndexKlineEvent(v) => FuturesWebsocketEvent::IndexKline(v),
                    FuturesEvents::LiquidationEvent(v) => FuturesWebsocketEvent::Liquidation(v),
                    FuturesEvents::KlineEvent(v) => FuturesWebsocketEvent::Kline(v),
                    FuturesEvents::OrderBook(v) => FuturesWebsocketEvent::OrderBook(v),
                    FuturesEvents::DepthOrderBookEvent(v) => FuturesWebsocketEvent::DepthOrderBook(v),
                    FuturesEvents::AggrTradesEvent(v) => FuturesWebsocketEvent::AggrTrades(v),
                    FuturesEvents::UserDataStreamExpiredEvent(v) => {
                        FuturesWebsocketEvent::UserDataStreamExpiredEvent(v)
                    }
                };
                let mut handler_guard = user_handler_clone.lock().await;
                (handler_guard)(action).await
            })
        });

        FuturesWebSockets {
            client: AsyncWebsocketClient::new(adapter_handler),
        }
    }

    pub async fn connect(&mut self, market: &FuturesMarket, subscription: &'a str) -> Result<()> {
        self.client.connect(&FuturesWebsocketAPI::Default.params(market, subscription)).await
    }

    pub async fn connect_with_config(
        &mut self, market: &FuturesMarket, subscription: &'a str, config: &'a Config,
    ) -> Result<()> {
        let wss_url = if config.ws_endpoint.contains("wss://") || config.ws_endpoint.contains("ws://") {
             // If config.ws_endpoint is already a full URL, use it directly
            config.ws_endpoint.clone()
        } else {
            // Otherwise, assume it's a base path and use FuturesWebsocketAPI::Custom to format it
            FuturesWebsocketAPI::Custom(config.ws_endpoint.clone()).params(market, subscription)
        };
        self.client.connect(&wss_url).await
    }

    pub async fn connect_multiple_streams(
        &mut self, market: &FuturesMarket, endpoints: &[String],
    ) -> Result<()> {
        self.client.connect(&FuturesWebsocketAPI::MultiStream.params(market, &endpoints.join("/"))).await
    }

    pub async fn disconnect(&mut self) -> Result<()> {
        self.client.disconnect().await
    }

    pub async fn event_loop(&mut self, running: Arc<AtomicBool>) -> Result<()> {
        self.client.event_loop(running).await
    }
}
