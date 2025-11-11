use serde::Deserialize;
use crate::domain::Order;

#[derive(Deserialize)]
pub struct AuthRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct OnRampHttpRequest {
    pub user_email: String,
    pub balance: u64,
    pub holding: u64,
}

#[derive(Deserialize)]
pub struct CreateMarketOrderRequest {
    pub market_id: u64,
    pub user_email: String,
    pub order: Order,
}

#[derive(Deserialize)]
pub struct GetOrderBookRequest {
    pub user_email: String,
    pub market_id: u64,
}

