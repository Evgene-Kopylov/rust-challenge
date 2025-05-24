use serde::{Deserialize, Serialize};

/// Представляет финансовую транзакцию
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Transfer {
    pub ts: u64,
    pub from: String,
    pub to: String,
    pub amount: f64,
    pub usd_price: f64,
}

/// Статистика пользователя
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStats {
    pub address: String,
    pub total_volume: f64,
    pub avg_buy_price: f64,
    pub avg_sell_price: f64,
    pub max_balance: f64,
}

impl UserStats {
    pub fn new(address: &str) -> Self {
        Self {
            address: address.to_string(),
            total_volume: 0.0,
            avg_buy_price: 0.0,
            avg_sell_price: 0.0,
            max_balance: 0.0,
        }
    }
}