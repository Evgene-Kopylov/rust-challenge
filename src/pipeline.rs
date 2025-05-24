use crate::model::{Transfer, UserStats};
use std::collections::HashMap;
use anyhow::Result;

struct UserState {
    balance: f64,
    max_balance: f64,
    buy_prices: Vec<(f64, f64)>,
    sell_prices: Vec<(f64, f64)>,
}

impl Default for UserState {
    fn default() -> Self {
        UserState {
            balance: 0.0,
            max_balance: 0.0,
            buy_prices: Vec::new(),
            sell_prices: Vec::new(),
        }
    }
}

impl UserState {
    fn total_volume(&self) -> f64 {
        self.buy_prices.iter().chain(&self.sell_prices).map(|(_, amt)| amt).sum()
    }

    fn avg_buy_price(&self) -> f64 {
        self.avg_price(&self.buy_prices)
    }

    fn avg_sell_price(&self) -> f64 {
        self.avg_price(&self.sell_prices)
    }

    fn avg_price(&self, data: &[(f64, f64)]) -> f64 {
                let (sum_px, sum_amt): (f64, f64) = data.iter().copied().fold((0.0, 0.0), |acc, (p, a)| (acc.0 + p * a, acc.1 + a));
                if sum_amt > 0.0 { sum_px / sum_amt } else { 0.0 }
            }
}

pub fn calculate_user_stats(transfers: &[Transfer]) -> Result<Vec<UserStats>> {
    let mut state = HashMap::<String, UserState>::new();

    for t in transfers {
        let from = state.entry(t.from.clone()).or_default();
        from.balance -= t.amount;
        from.max_balance = from.max_balance.max(from.balance);
        from.sell_prices.push((t.usd_price, t.amount));

        let to = state.entry(t.to.clone()).or_default();
        to.balance += t.amount;
        to.max_balance = to.max_balance.max(to.balance);
        to.buy_prices.push((t.usd_price, t.amount));
    }

    state.into_iter()
        .map(|(addr, s)| {
            Ok(UserStats {
                address: addr,
                total_volume: s.total_volume(),
                avg_buy_price: s.avg_buy_price(),
                avg_sell_price: s.avg_sell_price(),
                max_balance: s.max_balance,
        })
        })
        .collect()
}

