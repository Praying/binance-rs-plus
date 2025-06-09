use crate::util::{build_signed_request, is_start_time_valid};
use crate::model::{
    AccountInformation, Balance, Empty, Order, OrderCanceled, TradeHistory, Transaction,
};
use crate::client::Client;
use crate::errors::{Result, Error};
use std::collections::BTreeMap;
use std::fmt::Display;
use crate::api::API;
use crate::api::Spot;

#[derive(Clone)]
pub struct Account {
    pub client: Client,
    pub recv_window: u64,
}

struct OrderRequest {
    pub symbol: String,
    pub qty: f64,
    pub price: f64,
    pub stop_price: Option<f64>,
    pub order_side: OrderSide,
    pub order_type: OrderType,
    pub time_in_force: TimeInForce,
    pub new_client_order_id: Option<String>,
}

struct OrderQuoteQuantityRequest {
    pub symbol: String,
    pub quote_order_qty: f64,
    pub price: f64,
    pub order_side: OrderSide,
    pub order_type: OrderType,
    pub time_in_force: TimeInForce,
    pub new_client_order_id: Option<String>,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum OrderType {
    Limit,
    Market,
    StopLossLimit,
}

impl OrderType {
    pub fn from_int(value: i32) -> Option<Self> {
        match value {
            1 => Some(OrderType::Limit),
            2 => Some(OrderType::Market),
            3 => Some(OrderType::StopLossLimit),
            _ => None,
        }
    }
}

impl Display for OrderType {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::Limit => write!(f, "LIMIT"),
            Self::Market => write!(f, "MARKET"),
            Self::StopLossLimit => write!(f, "STOP_LOSS_LIMIT"),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum OrderSide {
    Buy,
    Sell,
}

impl OrderSide {
    pub fn from_int(value: i32) -> Option<Self> {
        match value {
            1 => Some(OrderSide::Buy),
            2 => Some(OrderSide::Sell),
            _ => None,
        }
    }
}

impl Display for OrderSide {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::Buy => write!(f, "BUY"),
            Self::Sell => write!(f, "SELL"),
        }
    }
}

#[allow(clippy::all)]
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum TimeInForce {
    GTC,
    IOC,
    FOK,
}

impl TimeInForce {
    pub fn from_int(value: i32) -> Option<Self> {
        match value {
            1 => Some(TimeInForce::GTC),
            2 => Some(TimeInForce::IOC),
            3 => Some(TimeInForce::FOK),
            _ => None,
        }
    }
}

impl Display for TimeInForce {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::GTC => write!(f, "GTC"),
            Self::IOC => write!(f, "IOC"),
            Self::FOK => write!(f, "FOK"),
        }
    }
}

impl Account {
    // Account Information
    pub async fn get_account(&self) -> Result<AccountInformation> {
        let request = build_signed_request(BTreeMap::new(), self.recv_window)?;
        self.client
            .get_signed(API::Spot(Spot::Account), Some(request))
            .await
    }

    // Balance for a single Asset
    pub async fn get_balance<S>(
        &self,
        asset: S,
    ) -> Result<Balance>
    where
        S: Into<String>,
    {
        match self.get_account().await {
            Ok(account) => {
                let cmp_asset = asset.into();
                for balance in account.balances {
                    if balance.asset == cmp_asset {
                        return Ok(balance);
                    }
                }
                Err(Error::Custom("Asset not found".to_string()))
            }
            Err(e) => Err(e),
        }
    }

    // Current open orders for ONE symbol
    pub async fn get_open_orders<S>(
        &self,
        symbol: S,
    ) -> Result<Vec<Order>>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Spot(Spot::OpenOrders), Some(request))
            .await
    }

    // All current open orders
    pub async fn get_all_open_orders(&self) -> Result<Vec<Order>> {
        let parameters: BTreeMap<String, String> = BTreeMap::new();

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Spot(Spot::OpenOrders), Some(request))
            .await
    }

    // Cancel all open orders for a single symbol
    pub async fn cancel_all_open_orders<S>(
        &self,
        symbol: S,
    ) -> Result<Vec<OrderCanceled>>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .delete_signed(API::Spot(Spot::OpenOrders), Some(request))
            .await
    }

    // Check an order's status
    pub async fn order_status<S>(
        &self,
        symbol: S,
        order_id: u64,
    ) -> Result<Order>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("orderId".into(), order_id.to_string());

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Spot(Spot::Order), Some(request))
            .await
    }

    /// Place a test status order
    ///
    /// This order is sandboxed: it is validated, but not sent to the matching engine.
    pub async fn test_order_status<S>(
        &self,
        symbol: S,
        order_id: u64,
    ) -> Result<()>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("orderId".into(), order_id.to_string());

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed::<Empty>(API::Spot(Spot::OrderTest), Some(request))
            .await?;
        Ok(())
    }

    // Place a LIMIT order - BUY
    pub async fn limit_buy<S, F>(
        &self,
        symbol: S,
        qty: F,
        price: f64,
    ) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let buy = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price,
            stop_price: None,
            order_side: OrderSide::Buy,
            order_type: OrderType::Limit,
            time_in_force: TimeInForce::GTC,
            new_client_order_id: None,
        };
        let order = self.build_order(buy);
        let request = build_signed_request(order, self.recv_window)?;
        self.client
            .post_signed(API::Spot(Spot::Order), request)
            .await
    }

    /// Place a test limit order - BUY
    ///
    /// This order is sandboxed: it is validated, but not sent to the matching engine.
    pub async fn test_limit_buy<S, F>(
        &self,
        symbol: S,
        qty: F,
        price: f64,
    ) -> Result<()>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let buy = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price,
            stop_price: None,
            order_side: OrderSide::Buy,
            order_type: OrderType::Limit,
            time_in_force: TimeInForce::GTC,
            new_client_order_id: None,
        };
        let order = self.build_order(buy);
        let request = build_signed_request(order, self.recv_window)?;
        self.client
            .post_signed::<Empty>(API::Spot(Spot::OrderTest), request)
            .await?;
        Ok(())
    }

    // Place a LIMIT order - SELL
    pub async fn limit_sell<S, F>(
        &self,
        symbol: S,
        qty: F,
        price: f64,
    ) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let sell = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price,
            stop_price: None,
            order_side: OrderSide::Sell,
            order_type: OrderType::Limit,
            time_in_force: TimeInForce::GTC,
            new_client_order_id: None,
        };
        let order = self.build_order(sell);
        let request = build_signed_request(order, self.recv_window)?;
        self.client
            .post_signed(API::Spot(Spot::Order), request)
            .await
    }

    /// Place a test LIMIT order - SELL
    ///
    /// This order is sandboxed: it is validated, but not sent to the matching engine.
    pub async fn test_limit_sell<S, F>(
        &self,
        symbol: S,
        qty: F,
        price: f64,
    ) -> Result<()>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let sell = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price,
            stop_price: None,
            order_side: OrderSide::Sell,
            order_type: OrderType::Limit,
            time_in_force: TimeInForce::GTC,
            new_client_order_id: None,
        };
        let order = self.build_order(sell);
        let request = build_signed_request(order, self.recv_window)?;
        self.client
            .post_signed::<Empty>(API::Spot(Spot::OrderTest), request)
            .await?;
        Ok(())
    }

    // Place a MARKET order - BUY
    pub async fn market_buy<S, F>(
        &self,
        symbol: S,
        qty: F,
    ) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let buy = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price: 0.0,
            stop_price: None,
            order_side: OrderSide::Buy,
            order_type: OrderType::Market,
            time_in_force: TimeInForce::GTC,
            new_client_order_id: None,
        };
        let order = self.build_order(buy);
        let request = build_signed_request(order, self.recv_window)?;
        self.client
            .post_signed(API::Spot(Spot::Order), request)
            .await
    }

    /// Place a test MARKET order - BUY
    ///
    /// This order is sandboxed: it is validated, but not sent to the matching engine.
    pub async fn test_market_buy<S, F>(
        &self,
        symbol: S,
        qty: F,
    ) -> Result<()>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let buy = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price: 0.0,
            stop_price: None,
            order_side: OrderSide::Buy,
            order_type: OrderType::Market,
            time_in_force: TimeInForce::GTC,
            new_client_order_id: None,
        };
        let order = self.build_order(buy);
        let request = build_signed_request(order, self.recv_window)?;
        self.client
            .post_signed::<Empty>(API::Spot(Spot::OrderTest), request)
            .await?;
        Ok(())
    }

    // Place a MARKET order with quote quantity - BUY
    pub async fn market_buy_using_quote_quantity<S, F>(
        &self,
        symbol: S,
        quote_order_qty: F,
    ) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let buy = OrderQuoteQuantityRequest {
            symbol: symbol.into(),
            quote_order_qty: quote_order_qty.into(),
            price: 0.0,
            order_side: OrderSide::Buy,
            order_type: OrderType::Market,
            time_in_force: TimeInForce::GTC,
            new_client_order_id: None,
        };
        let order = self.build_quote_quantity_order(buy);
        let request = build_signed_request(order, self.recv_window)?;
        self.client
            .post_signed(API::Spot(Spot::Order), request)
            .await
    }

    /// Place a test MARKET order with quote quantity - BUY
    ///
    /// This order is sandboxed: it is validated, but not sent to the matching engine.
    pub async fn test_market_buy_using_quote_quantity<S, F>(
        &self,
        symbol: S,
        quote_order_qty: F,
    ) -> Result<()>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let buy = OrderQuoteQuantityRequest {
            symbol: symbol.into(),
            quote_order_qty: quote_order_qty.into(),
            price: 0.0,
            order_side: OrderSide::Buy,
            order_type: OrderType::Market,
            time_in_force: TimeInForce::GTC,
            new_client_order_id: None,
        };
        let order = self.build_quote_quantity_order(buy);
        let request = build_signed_request(order, self.recv_window)?;
        self.client
            .post_signed::<Empty>(API::Spot(Spot::OrderTest), request)
            .await?;
        Ok(())
    }

    // Place a MARKET order - SELL
    pub async fn market_sell<S, F>(
        &self,
        symbol: S,
        qty: F,
    ) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let sell = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price: 0.0,
            stop_price: None,
            order_side: OrderSide::Sell,
            order_type: OrderType::Market,
            time_in_force: TimeInForce::GTC,
            new_client_order_id: None,
        };
        let order = self.build_order(sell);
        let request = build_signed_request(order, self.recv_window)?;
        self.client
            .post_signed(API::Spot(Spot::Order), request)
            .await
    }

    /// Place a test MARKET order - SELL
    ///
    /// This order is sandboxed: it is validated, but not sent to the matching engine.
    pub async fn test_market_sell<S, F>(
        &self,
        symbol: S,
        qty: F,
    ) -> Result<()>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let sell = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price: 0.0,
            stop_price: None,
            order_side: OrderSide::Sell,
            order_type: OrderType::Market,
            time_in_force: TimeInForce::GTC,
            new_client_order_id: None,
        };
        let order = self.build_order(sell);
        let request = build_signed_request(order, self.recv_window)?;
        self.client
            .post_signed::<Empty>(API::Spot(Spot::OrderTest), request)
            .await?;
        Ok(())
    }

    // Place a MARKET order with quote quantity - SELL
    pub async fn market_sell_using_quote_quantity<S, F>(
        &self,
        symbol: S,
        quote_order_qty: F,
    ) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let sell = OrderQuoteQuantityRequest {
            symbol: symbol.into(),
            quote_order_qty: quote_order_qty.into(),
            price: 0.0,
            order_side: OrderSide::Sell,
            order_type: OrderType::Market,
            time_in_force: TimeInForce::GTC,
            new_client_order_id: None,
        };
        let order = self.build_quote_quantity_order(sell);
        let request = build_signed_request(order, self.recv_window)?;
        self.client
            .post_signed(API::Spot(Spot::Order), request)
            .await
    }

    /// Place a test MARKET order with quote quantity - SELL
    ///
    /// This order is sandboxed: it is validated, but not sent to the matching engine.
    pub async fn test_market_sell_using_quote_quantity<S, F>(
        &self,
        symbol: S,
        quote_order_qty: F,
    ) -> Result<()>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let sell = OrderQuoteQuantityRequest {
            symbol: symbol.into(),
            quote_order_qty: quote_order_qty.into(),
            price: 0.0,
            order_side: OrderSide::Sell,
            order_type: OrderType::Market,
            time_in_force: TimeInForce::GTC,
            new_client_order_id: None,
        };
        let order = self.build_quote_quantity_order(sell);
        let request = build_signed_request(order, self.recv_window)?;
        self.client
            .post_signed::<Empty>(API::Spot(Spot::OrderTest), request)
            .await?;
        Ok(())
    }

    /// Create a stop limit buy order for the given symbol, price and stop price.
    /// Returning a `Transaction` value with the same parameters sent on the order.
    ///
    ///```no_run
    /// use binance_rs_plus::api::Binance;
    /// use binance_rs_plus::account::*;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let api_key = Some("api_key".into());
    ///     let secret_key = Some("secret_key".into());
    ///     let account: Account = Binance::new(api_key, secret_key);
    ///
    ///     match account.stop_limit_buy_order("BNBBTC", 1.0, 0.001, 0.0009).await {
    ///         Ok(answer) => println!("{:#?}", answer),
    ///         Err(e) => println!("Error: {:#?}", e),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn stop_limit_buy_order<S, F>(
        &self,
        symbol: S,
        qty: F,
        price: f64,
        stop_price: f64,
    ) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let buy = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price,
            stop_price: Some(stop_price),
            order_side: OrderSide::Buy,
            order_type: OrderType::StopLossLimit,
            time_in_force: TimeInForce::GTC,
            new_client_order_id: None,
        };
        let order = self.build_order(buy);
        let request = build_signed_request(order, self.recv_window)?;
        self.client
            .post_signed(API::Spot(Spot::Order), request)
            .await
    }

    /// Place a test Stop Limit Buy order
    ///
    /// This order is sandboxed: it is validated, but not sent to the matching engine.
    ///
    ///```no_run
    /// use binance_rs_plus::api::Binance;
    /// use binance_rs_plus::account::*;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let api_key = Some("api_key".into());
    ///     let secret_key = Some("secret_key".into());
    ///     let account: Account = Binance::new(api_key, secret_key);
    ///
    ///     match account.test_stop_limit_buy_order("BNBBTC", 1.0, 0.001, 0.0009).await {
    ///         Ok(_answer) => println!("Test stop limit buy order placed successfully."),
    ///         Err(e) => println!("Error: {:#?}", e),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn test_stop_limit_buy_order<S, F>(
        &self,
        symbol: S,
        qty: F,
        price: f64,
        stop_price: f64,
    ) -> Result<()>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let buy = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price,
            stop_price: Some(stop_price),
            order_side: OrderSide::Buy,
            order_type: OrderType::StopLossLimit,
            time_in_force: TimeInForce::GTC,
            new_client_order_id: None,
        };
        let order = self.build_order(buy);
        let request = build_signed_request(order, self.recv_window)?;
        self.client
            .post_signed::<Empty>(API::Spot(Spot::OrderTest), request)
            .await?;
        Ok(())
    }

    /// Create a stop limit sell order for the given symbol, price and stop price.
    /// Returning a `Transaction` value with the same parameters sent on the order.
    ///
    ///```no_run
    /// use binance_rs_plus::api::Binance;
    /// use binance_rs_plus::account::*;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let api_key = Some("api_key".into());
    ///     let secret_key = Some("secret_key".into());
    ///     let account: Account = Binance::new(api_key, secret_key);
    ///
    ///     match account.stop_limit_sell_order("BNBBTC", 1.0, 0.001, 0.0009).await {
    ///         Ok(answer) => println!("{:#?}", answer),
    ///         Err(e) => println!("Error: {:#?}", e),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn stop_limit_sell_order<S, F>(
        &self,
        symbol: S,
        qty: F,
        price: f64,
        stop_price: f64,
    ) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let sell = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price,
            stop_price: Some(stop_price),
            order_side: OrderSide::Sell,
            order_type: OrderType::StopLossLimit,
            time_in_force: TimeInForce::GTC,
            new_client_order_id: None,
        };
        let order = self.build_order(sell);
        let request = build_signed_request(order, self.recv_window)?;
        self.client
            .post_signed(API::Spot(Spot::Order), request)
            .await
    }

    /// Place a test Stop Limit Sell order
    ///
    /// This order is sandboxed: it is validated, but not sent to the matching engine.
    ///
    ///```no_run
    /// use binance_rs_plus::api::Binance;
    /// use binance_rs_plus::account::*;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let api_key = Some("api_key".into());
    ///     let secret_key = Some("secret_key".into());
    ///     let account: Account = Binance::new(api_key, secret_key);
    ///
    ///     match account.test_stop_limit_sell_order("BNBBTC", 1.0, 0.001, 0.0009).await {
    ///         Ok(_answer) => println!("Test stop limit sell order placed successfully."),
    ///         Err(e) => println!("Error: {:#?}", e),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn test_stop_limit_sell_order<S, F>(
        &self,
        symbol: S,
        qty: F,
        price: f64,
        stop_price: f64,
    ) -> Result<()>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let sell = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price,
            stop_price: Some(stop_price),
            order_side: OrderSide::Sell,
            order_type: OrderType::StopLossLimit,
            time_in_force: TimeInForce::GTC,
            new_client_order_id: None,
        };
        let order = self.build_order(sell);
        let request = build_signed_request(order, self.recv_window)?;
        self.client
            .post_signed::<Empty>(API::Spot(Spot::OrderTest), request)
            .await?;
        Ok(())
    }

    /// Create a custom order
    pub async fn custom_order<S, F>(
        &self,
        symbol: S,
        qty: F,
        price: f64,
        order_side: OrderSide,
        order_type: OrderType,
        time_in_force: TimeInForce,
        new_client_order_id: Option<String>,
    ) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let custom_order = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price,
            stop_price: None,
            order_side,
            order_type,
            time_in_force,
            new_client_order_id,
        };
        let order = self.build_order(custom_order);
        let request = build_signed_request(order, self.recv_window)?;
        self.client
            .post_signed(API::Spot(Spot::Order), request)
            .await
    }

    /// Place a test custom order
    ///
    /// This order is sandboxed: it is validated, but not sent to the matching engine.
    pub async fn test_custom_order<S, F>(
        &self,
        symbol: S,
        qty: F,
        price: f64,
        order_side: OrderSide,
        order_type: OrderType,
        time_in_force: TimeInForce,
        new_client_order_id: Option<String>,
    ) -> Result<()>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let custom_order = OrderRequest {
            symbol: symbol.into(),
            qty: qty.into(),
            price,
            stop_price: None,
            order_side,
            order_type,
            time_in_force,
            new_client_order_id,
        };
        let order = self.build_order(custom_order);
        let request = build_signed_request(order, self.recv_window)?;
        self.client
            .post_signed::<Empty>(API::Spot(Spot::OrderTest), request)
            .await?;
        Ok(())
    }

    // Cancel an order
    pub async fn cancel_order<S>(
        &self,
        symbol: S,
        order_id: u64,
    ) -> Result<OrderCanceled>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("orderId".into(), order_id.to_string());

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .delete_signed(API::Spot(Spot::Order), Some(request))
            .await
    }
    pub async fn cancel_order_with_client_id<S>(
        &self,
        symbol: S,
        client_order_id: String,
    ) -> Result<OrderCanceled>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("origClientOrderId".into(), client_order_id);

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .delete_signed(API::Spot(Spot::Order), Some(request))
            .await
    }

    /// Place a test cancel order
    ///
    /// This order is sandboxed: it is validated, but not sent to the matching engine.
    pub async fn test_cancel_order<S>(
        &self,
        symbol: S,
        order_id: u64,
    ) -> Result<()>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("orderId".into(), order_id.to_string());

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .delete_signed::<Empty>(API::Spot(Spot::OrderTest), Some(request))
            .await?;
        Ok(())
    }

    // Trade History
    pub async fn trade_history<S>(
        &self,
        symbol: S,
    ) -> Result<Vec<TradeHistory>>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Spot(Spot::MyTrades), Some(request))
            .await
    }

    // Trade History from a start time
    pub async fn trade_history_from<S>(
        &self,
        symbol: S,
        start_time: u64,
    ) -> Result<Vec<TradeHistory>>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("startTime".into(), start_time.to_string());

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Spot(Spot::MyTrades), Some(request))
            .await
    }

    // Trade History from a start time to an end time
    pub async fn trade_history_from_to<S>(
        &self,
        symbol: S,
        start_time: u64,
        end_time: u64,
    ) -> Result<Vec<TradeHistory>>
    where
        S: Into<String>,
    {
        if is_start_time_valid(&start_time) && (start_time < end_time) {
            self.get_trades(symbol.into(), start_time, end_time).await
        } else {
            Err(Error::Custom(
                "The given start or end time is invalid.".to_string(),
            ))
        }
    }
    async fn get_trades<S>(
        &self,
        symbol: S,
        start_time: u64,
        end_time: u64,
    ) -> Result<Vec<TradeHistory>>
    where
        S: Into<String>,
    {
        let symbol_string = symbol.into(); // Convert once
        let mut trades = match self
            .trade_history_from(symbol_string.clone(), start_time)
            .await
        {
            Ok(trades_vec) => trades_vec,
            Err(e) => return Err(e),
        };

        trades.retain(|trade| trade.time <= end_time);
        Ok(trades)
    }

    fn build_order(
        &self,
        order: OrderRequest,
    ) -> BTreeMap<String, String> {
        let mut open_order: BTreeMap<String, String> = BTreeMap::new();

        open_order.insert("symbol".into(), order.symbol);
        open_order.insert("quantity".into(), format!("{}", order.qty));
        open_order.insert("side".into(), order.order_side.to_string());
        open_order.insert("type".into(), order.order_type.to_string());
        if order.order_type == OrderType::Limit || order.order_type == OrderType::StopLossLimit {
            open_order.insert("timeInForce".into(), order.time_in_force.to_string());
            open_order.insert("price".into(), format!("{}", order.price));
        }
        if order.order_type == OrderType::StopLossLimit {
            if let Some(stop_price) = order.stop_price {
                open_order.insert("stopPrice".into(), format!("{}", stop_price));
            }
        }
        if let Some(ref new_client_order_id) = order.new_client_order_id {
            open_order.insert("newClientOrderId".into(), new_client_order_id.clone());
        }

        open_order
    }

    fn build_quote_quantity_order(
        &self,
        order: OrderQuoteQuantityRequest,
    ) -> BTreeMap<String, String> {
        let mut open_order: BTreeMap<String, String> = BTreeMap::new();

        open_order.insert("symbol".into(), order.symbol);
        open_order.insert("quoteOrderQty".into(), format!("{}", order.quote_order_qty));
        open_order.insert("side".into(), order.order_side.to_string());
        open_order.insert("type".into(), order.order_type.to_string());
        if order.order_type == OrderType::Limit {
            open_order.insert("timeInForce".into(), order.time_in_force.to_string());
            open_order.insert("price".into(), format!("{}", order.price));
        }
        if let Some(ref new_client_order_id) = order.new_client_order_id {
            open_order.insert("newClientOrderId".into(), new_client_order_id.clone());
        }

        open_order
    }
}
