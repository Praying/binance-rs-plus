use std::collections::BTreeMap;
use std::fmt::Display;
use crate::util::build_signed_request;
use crate::errors::{Result, Error}; // Added Error
use crate::client::Client;
use crate::api::{API, Futures};
use crate::model::Empty;
use crate::account::OrderSide; // Re-using OrderSide from spot account, assuming it's compatible
use crate::futures::model::{Order as FuturesOrder, TradeHistory, Income}; // Aliased Order to avoid conflict

use super::model::{
    ChangeLeverageResponse, Transaction, CanceledOrder, PositionRisk, AccountBalance,
    AccountInformation,
};

#[derive(Clone)]
pub struct FuturesAccount {
    pub client: Client,
    pub recv_window: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContractType {
    Perpetual,
    CurrentMonth,
    NextMonth,
    CurrentQuarter,
    NextQuarter,
}

impl From<ContractType> for String {
    fn from(item: ContractType) -> Self {
        match item {
            ContractType::Perpetual => String::from("PERPETUAL"),
            ContractType::CurrentMonth => String::from("CURRENT_MONTH"),
            ContractType::NextMonth => String::from("NEXT_MONTH"),
            ContractType::CurrentQuarter => String::from("CURRENT_QUARTER"),
            ContractType::NextQuarter => String::from("NEXT_QUARTER"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PositionSide {
    Both,
    Long,
    Short,
}

impl Display for PositionSide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Both => write!(f, "BOTH"),
            Self::Long => write!(f, "LONG"),
            Self::Short => write!(f, "SHORT"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderType {
    Limit,
    Market,
    Stop,
    StopMarket,
    TakeProfit,
    TakeProfitMarket,
    TrailingStopMarket,
}

impl Display for OrderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Limit => write!(f, "LIMIT"),
            Self::Market => write!(f, "MARKET"),
            Self::Stop => write!(f, "STOP"),
            Self::StopMarket => write!(f, "STOP_MARKET"),
            Self::TakeProfit => write!(f, "TAKE_PROFIT"),
            Self::TakeProfitMarket => write!(f, "TAKE_PROFIT_MARKET"),
            Self::TrailingStopMarket => write!(f, "TRAILING_STOP_MARKET"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkingType {
    MarkPrice,
    ContractPrice,
}

impl Display for WorkingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MarkPrice => write!(f, "MARK_PRICE"),
            Self::ContractPrice => write!(f, "CONTRACT_PRICE"),
        }
    }
}

#[allow(clippy::all)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeInForce {
    GTC, // Good Till Cancel
    IOC, // Immediate or Cancel
    FOK, // Fill or Kill
    GTX, // Good Till Crossing (Post Only)
}

impl Display for TimeInForce {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GTC => write!(f, "GTC"),
            Self::IOC => write!(f, "IOC"),
            Self::FOK => write!(f, "FOK"),
            Self::GTX => write!(f, "GTX"),
        }
    }
}

// Internal struct to build order requests
struct OrderRequestBuilder {
    pub symbol: String,
    pub side: OrderSide,
    pub position_side: Option<PositionSide>,
    pub order_type: OrderType,
    pub time_in_force: Option<TimeInForce>,
    pub quantity: Option<f64>, // Changed from qty to quantity for clarity
    pub reduce_only: Option<bool>,
    pub price: Option<f64>,
    pub new_client_order_id: Option<String>,
    pub stop_price: Option<f64>,
    pub close_position: Option<bool>, // if true, all existing long/short positions will be closed
    pub activation_price: Option<f64>, // Used with TRAILING_STOP_MARKET orders
    pub callback_rate: Option<f64>,    // Used with TRAILING_STOP_MARKET orders
    pub working_type: Option<WorkingType>,
    pub price_protect: Option<bool>, // For STOP_MARKET and TAKE_PROFIT_MARKET orders
}

// Public struct for custom orders, mirrors internal builder fields for user convenience
#[derive(Debug, Clone)]
pub struct CustomOrderRequest {
    pub symbol: String,
    pub side: OrderSide,
    pub position_side: Option<PositionSide>,
    pub order_type: OrderType,
    pub time_in_force: Option<TimeInForce>,
    pub quantity: Option<f64>,
    pub reduce_only: Option<bool>,
    pub price: Option<f64>,
    pub new_client_order_id: Option<String>,
    pub stop_price: Option<f64>,
    pub close_position: Option<bool>,
    pub activation_price: Option<f64>,
    pub callback_rate: Option<f64>,
    pub working_type: Option<WorkingType>,
    pub price_protect: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct IncomeRequest {
    pub symbol: Option<String>,
    pub income_type: Option<IncomeType>,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub limit: Option<u32>, // Max 1000
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IncomeType {
    TRANSFER,
    WELCOME_BONUS,
    REALIZED_PNL,
    FUNDING_FEE,
    COMMISSION,
    INSURANCE_CLEAR,
    REFERRAL_KICKBACK,
    COMMISSION_REBATE,
    API_REBATE,
    CONTEST_REWARD,
    CROSS_COLLATERAL_TRANSFER,
    OPTIONS_PREMIUM_FEE,
    OPTIONS_SETTLE_PROFIT,
    INTERNAL_TRANSFER,
    AUTO_EXCHANGE,
    DELIVERED_SETTELMENT,
    COIN_SWAP_DEPOSIT,
    COIN_SWAP_WITHDRAW,
    POSITION_LIMIT_INCREASE_FEE,
}

impl Display for IncomeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self) // Use Debug representation for string conversion
    }
}

impl FuturesAccount {
    // Helper to build BTreeMap for an order
    fn build_order_params(&self, order_builder: OrderRequestBuilder) -> BTreeMap<String, String> {
        let mut parameters = BTreeMap::new();
        parameters.insert("symbol".into(), order_builder.symbol);
        parameters.insert("side".into(), order_builder.side.to_string());
        parameters.insert("type".into(), order_builder.order_type.to_string());

        if let Some(ps) = order_builder.position_side {
            parameters.insert("positionSide".into(), ps.to_string());
        }
        if let Some(tif) = order_builder.time_in_force {
            parameters.insert("timeInForce".into(), tif.to_string());
        }
        if let Some(q) = order_builder.quantity {
            parameters.insert("quantity".into(), q.to_string());
        }
        if let Some(ro) = order_builder.reduce_only {
            parameters.insert("reduceOnly".into(), ro.to_string());
        }
        if let Some(p) = order_builder.price {
            parameters.insert("price".into(), p.to_string());
        }
        if let Some(ncoi) = order_builder.new_client_order_id {
            parameters.insert("newClientOrderId".into(), ncoi);
        }
        if let Some(sp) = order_builder.stop_price {
            parameters.insert("stopPrice".into(), sp.to_string());
        }
        if let Some(cp) = order_builder.close_position {
            parameters.insert("closePosition".into(), cp.to_string());
        }
        if let Some(ap) = order_builder.activation_price {
            parameters.insert("activationPrice".into(), ap.to_string());
        }
        if let Some(cr) = order_builder.callback_rate {
            parameters.insert("callbackRate".into(), cr.to_string());
        }
        if let Some(wt) = order_builder.working_type {
            parameters.insert("workingType".into(), wt.to_string());
        }
        if let Some(pp) = order_builder.price_protect {
            parameters.insert("priceProtect".into(), pp.to_string());
        }
        parameters
    }

    pub async fn limit_buy(
        &self, symbol: impl Into<String>, qty: impl Into<f64>, price: f64,
        time_in_force: TimeInForce,
    ) -> Result<Transaction> {
        let builder = OrderRequestBuilder {
            symbol: symbol.into(),
            side: OrderSide::Buy,
            position_side: None,
            order_type: OrderType::Limit,
            time_in_force: Some(time_in_force),
            quantity: Some(qty.into()),
            reduce_only: None,
            price: Some(price),
            new_client_order_id: None,
            stop_price: None,
            close_position: None,
            activation_price: None,
            callback_rate: None,
            working_type: None,
            price_protect: None,
        };
        let order_params = self.build_order_params(builder);
        let request = build_signed_request(order_params, self.recv_window)?;
        self.client
            .post_signed(API::Futures(Futures::Order), request)
            .await
    }

    pub async fn limit_sell(
        &self, symbol: impl Into<String>, qty: impl Into<f64>, price: f64,
        time_in_force: TimeInForce,
    ) -> Result<Transaction> {
        let builder = OrderRequestBuilder {
            symbol: symbol.into(),
            side: OrderSide::Sell,
            position_side: None,
            order_type: OrderType::Limit,
            time_in_force: Some(time_in_force),
            quantity: Some(qty.into()),
            reduce_only: None,
            price: Some(price),
            new_client_order_id: None,
            stop_price: None,
            close_position: None,
            activation_price: None,
            callback_rate: None,
            working_type: None,
            price_protect: None,
        };
        let order_params = self.build_order_params(builder);
        let request = build_signed_request(order_params, self.recv_window)?;
        self.client
            .post_signed(API::Futures(Futures::Order), request)
            .await
    }

    pub async fn market_buy<S, F>(&self, symbol: S, qty: F) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let builder = OrderRequestBuilder {
            symbol: symbol.into(),
            side: OrderSide::Buy,
            position_side: None,
            order_type: OrderType::Market,
            time_in_force: None,
            quantity: Some(qty.into()),
            reduce_only: None,
            price: None,
            new_client_order_id: None,
            stop_price: None,
            close_position: None,
            activation_price: None,
            callback_rate: None,
            working_type: None,
            price_protect: None,
        };
        let order_params = self.build_order_params(builder);
        let request = build_signed_request(order_params, self.recv_window)?;
        self.client
            .post_signed(API::Futures(Futures::Order), request)
            .await
    }

    pub async fn market_sell<S, F>(&self, symbol: S, qty: F) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let builder = OrderRequestBuilder {
            symbol: symbol.into(),
            side: OrderSide::Sell,
            position_side: None,
            order_type: OrderType::Market,
            time_in_force: None,
            quantity: Some(qty.into()),
            reduce_only: None,
            price: None,
            new_client_order_id: None,
            stop_price: None,
            close_position: None,
            activation_price: None,
            callback_rate: None,
            working_type: None,
            price_protect: None,
        };
        let order_params = self.build_order_params(builder);
        let request = build_signed_request(order_params, self.recv_window)?;
        self.client
            .post_signed(API::Futures(Futures::Order), request)
            .await
    }

    pub async fn cancel_order<S>(&self, symbol: S, order_id: u64) -> Result<CanceledOrder>
    where
        S: Into<String>,
    {
        let mut parameters = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("orderId".into(), order_id.to_string());

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .delete_signed(API::Futures(Futures::Order), Some(request))
            .await
    }

    pub async fn cancel_order_with_client_id<S>(
        &self, symbol: S, orig_client_order_id: String,
    ) -> Result<CanceledOrder>
    where
        S: Into<String>,
    {
        let mut parameters = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("origClientOrderId".into(), orig_client_order_id);

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .delete_signed(API::Futures(Futures::Order), Some(request))
            .await
    }

    pub async fn stop_market_close_buy<S, F>(&self, symbol: S, stop_price: F) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let builder = OrderRequestBuilder {
            symbol: symbol.into(),
            side: OrderSide::Buy,
            position_side: None,
            order_type: OrderType::StopMarket,
            time_in_force: None,
            quantity: None,
            reduce_only: None,
            price: None,
            new_client_order_id: None,
            stop_price: Some(stop_price.into()),
            close_position: Some(true),
            activation_price: None,
            callback_rate: None,
            working_type: None,
            price_protect: None,
        };
        let order_params = self.build_order_params(builder);
        let request = build_signed_request(order_params, self.recv_window)?;
        self.client
            .post_signed(API::Futures(Futures::Order), request)
            .await
    }

    pub async fn stop_market_close_sell<S, F>(&self, symbol: S, stop_price: F) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let builder = OrderRequestBuilder {
            symbol: symbol.into(),
            side: OrderSide::Sell,
            position_side: None,
            order_type: OrderType::StopMarket,
            time_in_force: None,
            quantity: None,
            reduce_only: None,
            price: None,
            new_client_order_id: None,
            stop_price: Some(stop_price.into()),
            close_position: Some(true),
            activation_price: None,
            callback_rate: None,
            working_type: None,
            price_protect: None,
        };
        let order_params = self.build_order_params(builder);
        let request = build_signed_request(order_params, self.recv_window)?;
        self.client
            .post_signed(API::Futures(Futures::Order), request)
            .await
    }

    pub async fn custom_order(&self, order_request: CustomOrderRequest) -> Result<Transaction> {
        let builder = OrderRequestBuilder {
            symbol: order_request.symbol,
            side: order_request.side,
            position_side: order_request.position_side,
            order_type: order_request.order_type,
            time_in_force: order_request.time_in_force,
            quantity: order_request.quantity,
            reduce_only: order_request.reduce_only,
            price: order_request.price,
            new_client_order_id: order_request.new_client_order_id,
            stop_price: order_request.stop_price,
            close_position: order_request.close_position,
            activation_price: order_request.activation_price,
            callback_rate: order_request.callback_rate,
            working_type: order_request.working_type,
            price_protect: order_request.price_protect,
        };
        let order_params = self.build_order_params(builder);
        let request = build_signed_request(order_params, self.recv_window)?;
        self.client
            .post_signed(API::Futures(Futures::Order), request)
            .await
    }

    pub async fn custom_batch_orders(
        &self, _order_requests: Vec<CustomOrderRequest>,
    ) -> Result<Vec<Transaction>> {
        Err(Error::Custom("Batch order functionality not yet fully implemented.".to_string()))
    }

    pub async fn get_all_orders<S, F, N>(
        &self, symbol: S, order_id: F, start_time: F, end_time: F, limit: N,
    ) -> Result<Vec<FuturesOrder>>
    where
        S: Into<String>,
        F: Into<Option<u64>>,
        N: Into<Option<u16>>,
    {
        let mut parameters = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        if let Some(oid) = order_id.into() {
            parameters.insert("orderId".into(), oid.to_string());
        }
        if let Some(st) = start_time.into() {
            parameters.insert("startTime".into(), st.to_string());
        }
        if let Some(et) = end_time.into() {
            parameters.insert("endTime".into(), et.to_string());
        }
        if let Some(lim) = limit.into() {
            parameters.insert("limit".into(), lim.to_string());
        }

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed(API::Futures(Futures::AllOrders), Some(request))
            .await
    }

    pub async fn get_user_trades<S, F, N>(
        &self, symbol: S, from_id: F, start_time: F, end_time: F, limit: N,
    ) -> Result<Vec<TradeHistory>>
    where
        S: Into<String>,
        F: Into<Option<u64>>,
        N: Into<Option<u16>>,
    {
        let mut parameters = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        if let Some(fid) = from_id.into() {
            parameters.insert("fromId".into(), fid.to_string());
        }
        if let Some(st) = start_time.into() {
            parameters.insert("startTime".into(), st.to_string());
        }
        if let Some(et) = end_time.into() {
            parameters.insert("endTime".into(), et.to_string());
        }
        if let Some(lim) = limit.into() {
            parameters.insert("limit".into(), lim.to_string());
        }

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client.get_signed(API::Futures(Futures::UserTrades), Some(request)).await
    }

    pub async fn position_information<S>(&self, symbol: S) -> Result<Vec<PositionRisk>>
    where S: Into<String>,
    {
        let mut parameters = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client.get_signed(API::Futures(Futures::PositionRisk), Some(request)).await
    }

    pub async fn account_information(&self) -> Result<AccountInformation> {
        let request = build_signed_request(BTreeMap::new(), self.recv_window)?;
        self.client.get_signed(API::Futures(Futures::Account), Some(request)).await
    }

    pub async fn account_balance(&self) -> Result<Vec<AccountBalance>> {
        let request = build_signed_request(BTreeMap::new(), self.recv_window)?;
        self.client.get_signed(API::Futures(Futures::Balance), Some(request)).await
    }

    pub async fn change_initial_leverage<S>(&self, symbol: S, leverage: u8) -> Result<ChangeLeverageResponse>
    where S: Into<String>,
    {
        let mut parameters = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("leverage".into(), leverage.to_string());
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client.post_signed(API::Futures(Futures::ChangeInitialLeverage), request).await
    }

    pub async fn change_margin_type<S>(&self, symbol: S, isolated: bool) -> Result<()>
    where S: Into<String>,
    {
        let mut parameters = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("marginType".into(), if isolated { "ISOLATED".to_string() } else { "CROSSED".to_string() });
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client.post_signed::<Empty>(API::Futures(Futures::MarginType), request).await?;
        Ok(())
    }

    pub async fn change_position_margin<S>(&self, symbol: S, amount: f64, margin_type: u8) -> Result<()>
    where S: Into<String>,
    {
        let mut parameters = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("amount".into(), amount.to_string());
        parameters.insert("type".into(), margin_type.to_string());
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client.post_signed::<Empty>(API::Futures(Futures::PositionMargin), request).await?;
        Ok(())
    }

    pub async fn change_position_mode(&self, dual_side_position: bool) -> Result<()> {
        let mut parameters = BTreeMap::new();
        parameters.insert("dualSidePosition".into(), dual_side_position.to_string());
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client.post_signed::<Empty>(API::Futures(Futures::PositionSide), request).await?;
        Ok(())
    }

    pub async fn cancel_all_open_orders<S>(&self, symbol: S) -> Result<()>
    where S: Into<String>,
    {
        let mut parameters = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client.delete_signed::<Empty>(API::Futures(Futures::AllOpenOrders), Some(request)).await?;
        Ok(())
    }

    pub async fn get_all_open_orders<S>(&self, symbol: S) -> Result<Vec<FuturesOrder>>
    where S: Into<String>,
    {
        let mut parameters = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client.get_signed(API::Futures(Futures::OpenOrders), Some(request)).await
    }
    
    pub async fn get_income(&self, income_request: IncomeRequest) -> Result<Vec<Income>> {
        let mut parameters = BTreeMap::new();
        if let Some(symbol) = income_request.symbol {
            parameters.insert("symbol".into(), symbol);
        }
        if let Some(income_type) = income_request.income_type {
            parameters.insert("incomeType".into(), income_type.to_string());
        }
        if let Some(start_time) = income_request.start_time {
            parameters.insert("startTime".into(), start_time.to_string());
        }
        if let Some(end_time) = income_request.end_time {
            parameters.insert("endTime".into(), end_time.to_string());
        }
        if let Some(limit) = income_request.limit {
            parameters.insert("limit".into(), limit.to_string());
        }
        let request = build_signed_request(parameters, self.recv_window)?;
        self.client.get_signed(API::Futures(Futures::Income), Some(request)).await
    }
}
