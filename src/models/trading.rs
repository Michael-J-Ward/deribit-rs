use crate::models::{
    AdvanceOption, AssetKind, Currency, Direction, Either, EmptyRequest, OrderState, OrderType,
    Request, Role, TimeInForce, Trigger,
};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone)]
pub struct BuyRequest(TradeRequest);
#[derive(Deserialize, Debug, Clone)]
pub struct BuyResponse(TradeResponse);

impl BuyRequest {
    pub fn market(instrument_name: &str, amount: f64) -> BuyRequest {
        BuyRequest(TradeRequest::market(instrument_name, amount))
    }
    pub fn limit(instrument_name: &str, amount: f64, price: f64) -> BuyRequest {
        BuyRequest(TradeRequest::limit(instrument_name, amount, price))
    }
}

impl Request for BuyRequest {
    const METHOD: &'static str = "private/buy";
    type Response = BuyResponse;
}

#[derive(Serialize, Debug, Clone)]
pub struct SellRequest(TradeRequest);
#[derive(Deserialize, Debug, Clone)]
pub struct SellResponse(TradeResponse);
impl SellRequest {
    pub fn market(instrument_name: &str, amount: f64) -> SellRequest {
        SellRequest(TradeRequest::market(instrument_name, amount))
    }
    pub fn limit(instrument_name: &str, amount: f64, price: f64) -> SellRequest {
        SellRequest(TradeRequest::limit(instrument_name, amount, price))
    }
}

impl Request for SellRequest {
    const METHOD: &'static str = "private/sell";
    type Response = SellResponse;
}

#[derive(Serialize, Debug, Clone)]
pub struct TradeRequest {
    pub instrument_name: String,
    pub amount: f64,
    pub r#type: OrderType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    pub time_in_force: TimeInForce,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_show: Option<f64>,
    pub post_only: bool,
    pub reduce_only: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger: Option<Trigger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced: Option<AdvanceOption>,
}

impl TradeRequest {
    pub fn market(instrument_name: &str, amount: f64) -> TradeRequest {
        TradeRequest {
            instrument_name: instrument_name.into(),
            amount: amount,
            r#type: OrderType::Market,
            label: None,
            price: None,
            time_in_force: TimeInForce::GoodTilCancelled,
            max_show: None,
            post_only: false,
            reduce_only: false,
            stop_price: None,
            trigger: None,
            advanced: None,
        }
    }

    pub fn limit(instrument_name: &str, amount: f64, price: f64) -> TradeRequest {
        TradeRequest {
            instrument_name: instrument_name.into(),
            amount: amount,
            r#type: OrderType::Limit,
            label: None,
            price: Some(price),
            time_in_force: TimeInForce::GoodTilCancelled,
            max_show: None,
            post_only: false,
            reduce_only: false,
            stop_price: None,
            trigger: None,
            advanced: None,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct TradeResponse {
    pub trades: Vec<Trade>,
    pub order: Order,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Trade {
    pub trade_seq: i64,
    pub trade_id: String,
    pub timestamp: u128,
    pub tick_direction: i64,
    pub state: OrderState,
    pub self_trade: bool,
    pub price: f64,
    pub order_type: OrderType,
    pub order_id: String,
    pub matching_id: Option<String>,
    pub liquidity: Role,
    pub label: Option<String>,
    pub instrument_name: String,
    pub index_price: f64,
    pub fee_currency: Currency,
    pub fee: f64,
    pub direction: Direction,
    pub amount: f64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Order {
    pub time_in_force: TimeInForce,
    pub reduce_only: bool,
    pub profit_loss: f64,
    pub price: Either<String, f64>,
    pub post_only: bool,
    pub order_type: OrderType,
    pub order_state: OrderState,
    pub order_id: String,
    pub max_show: f64,
    pub last_update_timestamp: u128,
    pub label: Option<String>,
    pub is_liquidation: bool,
    pub instrument_name: String,
    pub filled_amount: f64,
    pub direction: Direction,
    pub creation_timestamp: u128,
    pub commission: f64,
    pub average_price: f64,
    pub api: bool,
    pub amount: f64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum CancelOrderType {
    All,
    Limit,
    Stop,
}

#[derive(Serialize, Debug, Clone)]
pub struct CancelAllRequest;

impl Request for CancelAllRequest {
    const METHOD: &'static str = "private/cancel_all";
    type Response = CancelResponse;
}

impl EmptyRequest for CancelAllRequest {
    #[inline]
    fn empty(&self) -> bool {
        true
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct CancelAllByInstrumentRequest {
    pub instrument_name: String,
    pub r#type: CancelOrderType,
}

impl Request for CancelAllByInstrumentRequest {
    const METHOD: &'static str = "private/cancel_all_by_instrument";
    type Response = CancelResponse;
}

#[derive(Serialize, Debug, Clone, Copy)]
pub struct CancelAllByCurrencyRequest {
    pub currency: Currency,
    pub kind: AssetKind,
    pub r#type: CancelOrderType,
}

impl Request for CancelAllByCurrencyRequest {
    const METHOD: &'static str = "private/cancel_all_by_currency";
    type Response = CancelResponse;
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum CancelResponse {
    Ok,
}
