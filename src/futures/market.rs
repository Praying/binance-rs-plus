/*!
## Implemented functionality
- [x] `Order Book`
- [x] `Recent Trades List`
- [ ] `Old Trades Lookup (MARKET_DATA)`
- [x] `Compressed/Aggregate Trades List`
- [x] `Kline/Candlestick Data`
- [x] `Mark Price`
- [ ] `Get Funding Rate History (MARKET_DATA)`
- [x] `24hr Ticker Price Change Statistics`
- [x] `Symbol Price Ticker`
- [x] `Symbol Order Book Ticker`
- [x] `Get all Liquidation Orders`
- [x] `Open Interest`
- [ ] `Notional and Leverage Brackets (MARKET_DATA)`
- [ ] `Open Interest Statistics (MARKET_DATA)`
- [ ] `Top Trader Long/Short Ratio (Accounts) (MARKET_DATA)`
- [ ] `Top Trader Long/Short Ratio (Positions) (MARKET_DATA)`
- [ ] `Long/Short Ratio (MARKET_DATA)`
- [ ] `Taker Buy/Sell Volume (MARKET_DATA)`
*/

use crate::util::{build_request, build_signed_request};
use crate::futures::model::{
    AggTrades, BookTickers, KlineSummaries, KlineSummary, LiquidationOrders, MarkPrices,
    OpenInterest, OpenInterestHist, OrderBook, PriceStats, SymbolPrice, Tickers, Trades,
};
use crate::client::Client;
use crate::errors::Result; // Error will be brought in by Result if needed via crate::errors::Error
use std::collections::BTreeMap;
use serde_json::Value;
use crate::api::API;
use crate::api::Futures;

// TODO
// Make enums for Strings
// Add limit parameters to functions
// Implement all functions

#[derive(Clone)]
pub struct FuturesMarket {
    pub client: Client,
    pub recv_window: u64,
}

impl FuturesMarket {
    // Order book (Default 100; max 1000)
    pub async fn get_depth<S>(&self, symbol: S) -> Result<OrderBook>
    // async added
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("symbol".into(), symbol.into());
        let request = build_request(parameters);

        self.client
            .get(API::Futures(Futures::Depth), Some(request))
            .await // .await added
    }

    // Order book at a custom depth. Currently supported values
    // are 5, 10, 20, 50, 100, 500, 1000
    pub async fn get_custom_depth<S>(&self, symbol: S, depth: u64) -> Result<OrderBook>
    // async added
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("limit".into(), depth.to_string());
        let request = build_request(parameters);
        self.client
            .get(API::Futures(Futures::Depth), Some(request))
            .await // .await added
    }

    pub async fn get_trades<S>(&self, symbol: S) -> Result<Trades>
    // async added
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        let request = build_request(parameters);
        self.client
            .get(API::Futures(Futures::Trades), Some(request))
            .await // .await added
    }

    // TODO This may be incomplete, as it hasn't been tested
    pub async fn get_historical_trades<S1, S2, S3>(
        // async added
        &self,
        symbol: S1,
        from_id: S2,
        limit: S3,
    ) -> Result<Trades>
    where
        S1: Into<String>,
        S2: Into<Option<u64>>,
        S3: Into<Option<u16>>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("symbol".into(), symbol.into());

        // Add three optional parameters
        if let Some(lt) = limit.into() {
            parameters.insert("limit".into(), format!("{}", lt));
        }
        if let Some(fi) = from_id.into() {
            parameters.insert("fromId".into(), format!("{}", fi));
        }

        let request = build_signed_request(parameters, self.recv_window)?;

        self.client
            .get_signed(API::Futures(Futures::HistoricalTrades), Some(request))
            .await // .await added
    }

    pub async fn get_agg_trades<S1, S2, S3, S4, S5>(
        // async added
        &self,
        symbol: S1,
        from_id: S2,
        start_time: S3,
        end_time: S4,
        limit: S5,
    ) -> Result<AggTrades>
    where
        S1: Into<String>,
        S2: Into<Option<u64>>,
        S3: Into<Option<u64>>,
        S4: Into<Option<u64>>,
        S5: Into<Option<u16>>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("symbol".into(), symbol.into());

        // Add three optional parameters
        if let Some(lt) = limit.into() {
            parameters.insert("limit".into(), format!("{}", lt));
        }
        if let Some(st) = start_time.into() {
            parameters.insert("startTime".into(), format!("{}", st));
        }
        if let Some(et) = end_time.into() {
            parameters.insert("endTime".into(), format!("{}", et));
        }
        if let Some(fi) = from_id.into() {
            parameters.insert("fromId".into(), format!("{}", fi));
        }

        let request = build_request(parameters);

        self.client
            .get(API::Futures(Futures::AggTrades), Some(request))
            .await // .await added
    }

    // Returns up to 'limit' klines for given symbol and interval ("1m", "5m", ...)
    // https://github.com/binance-exchange/binance-official-api-docs/blob/master/rest-api.md#klinecandlestick-data
    pub async fn get_klines<S1, S2, S3, S4, S5>(
        // async added
        &self,
        symbol: S1,
        interval: S2,
        limit: S3,
        start_time: S4,
        end_time: S5,
    ) -> Result<KlineSummaries>
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<Option<u16>>,
        S4: Into<Option<u64>>,
        S5: Into<Option<u64>>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("interval".into(), interval.into());

        // Add three optional parameters
        if let Some(lt) = limit.into() {
            parameters.insert("limit".into(), format!("{}", lt));
        }
        if let Some(st) = start_time.into() {
            parameters.insert("startTime".into(), format!("{}", st));
        }
        if let Some(et) = end_time.into() {
            parameters.insert("endTime".into(), format!("{}", et));
        }

        let request = build_request(parameters);

        let data: Vec<Vec<Value>> = self
            .client
            .get(API::Futures(Futures::Klines), Some(request))
            .await?; // .await? added

        let klines = KlineSummaries::AllKlineSummaries(
            data.iter()
                .map(|row| row.try_into())
                .collect::<Result<Vec<KlineSummary>>>()?,
        );

        Ok(klines)
    }

    // 24hr ticker price change statistics
    pub async fn get_24h_price_stats<S>(&self, symbol: S) -> Result<PriceStats>
    // async added
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("symbol".into(), symbol.into());
        let request = build_request(parameters);

        self.client
            .get(API::Futures(Futures::Ticker24hr), Some(request))
            .await // .await added
    }

    // 24hr ticker price change statistics for all symbols
    pub async fn get_all_24h_price_stats(&self) -> Result<Vec<PriceStats>> {
        // async added
        self.client
            .get(API::Futures(Futures::Ticker24hr), None)
            .await // .await added
    }

    // Latest price for ONE symbol.
    pub async fn get_price<S>(&self, symbol: S) -> Result<SymbolPrice>
    // async added
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        parameters.insert("symbol".into(), symbol.into());
        let request = build_request(parameters);

        self.client
            .get(API::Futures(Futures::TickerPrice), Some(request))
            .await // .await added
    }

    // Latest price for all symbols.
    pub async fn get_all_prices(&self) -> Result<crate::model::Prices> {
        // async added
        self.client
            .get(API::Futures(Futures::TickerPrice), None)
            .await // .await added
    }

    // Symbols order book ticker
    // -> Best price/qty on the order book for ALL symbols.
    pub async fn get_all_book_tickers(&self) -> Result<BookTickers> {
        // async added
        self.client
            .get(API::Futures(Futures::BookTicker), None)
            .await // .await added
    }

    // -> Best price/qty on the order book for ONE symbol
    pub async fn get_book_ticker<S>(&self, symbol: S) -> Result<Tickers>
    // async added
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        let request = build_request(parameters);
        self.client
            .get(API::Futures(Futures::BookTicker), Some(request))
            .await // .await added
    }

    pub async fn get_mark_prices(&self) -> Result<MarkPrices> {
        // async added
        self.client
            .get(API::Futures(Futures::PremiumIndex), None)
            .await // .await added
    }

    pub async fn get_all_liquidation_orders(&self) -> Result<LiquidationOrders> {
        // async added
        self.client
            .get(API::Futures(Futures::AllForceOrders), None)
            .await // .await added
    }

    pub async fn open_interest<S>(&self, symbol: S) -> Result<OpenInterest>
    // async added
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        let request = build_request(parameters);
        self.client
            .get(API::Futures(Futures::OpenInterest), Some(request))
            .await // .await added
    }

    pub async fn open_interest_statistics<S1, S2, S3, S4, S5>(
        // async added
        &self,
        symbol: S1,
        period: S2,
        limit: S3,
        start_time: S4,
        end_time: S5,
    ) -> Result<Vec<OpenInterestHist>>
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<Option<u16>>,
        S4: Into<Option<u64>>,
        S5: Into<Option<u64>>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("period".into(), period.into());

        if let Some(lt) = limit.into() {
            parameters.insert("limit".into(), format!("{}", lt));
        }
        if let Some(st) = start_time.into() {
            parameters.insert("startTime".into(), format!("{}", st));
        }
        if let Some(et) = end_time.into() {
            parameters.insert("endTime".into(), format!("{}", et));
        }

        let request = build_request(parameters);
        self.client
            .get(API::Futures(Futures::OpenInterestHist), Some(request))
            .await // .await added
    }
}
