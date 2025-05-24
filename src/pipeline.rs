use crate::model::{Transfer, UserStats};
use anyhow::Result;
use std::collections::HashMap;

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
        self.buy_prices
            .iter()
            .chain(&self.sell_prices)
            .map(|(_, amt)| amt)
            .sum()
    }

    fn avg_buy_price(&self) -> f64 {
        self.avg_price(&self.buy_prices)
    }

    fn avg_sell_price(&self) -> f64 {
        self.avg_price(&self.sell_prices)
    }

    fn avg_price(&self, data: &[(f64, f64)]) -> f64 {
        let (sum_px, sum_amt) = data
            .iter()
            .fold((0.0, 0.0), |(px, amt), (p, a)| (px + p * a, amt + a));
        if sum_amt.abs() > f64::EPSILON {
            sum_px / sum_amt
        } else {
            0.0
        }
    }
}

pub fn calculate_user_stats(transfers: &[Transfer]) -> Result<Vec<UserStats>> {
    let mut stats = HashMap::new();

    for transfer in transfers {
        let sender = stats.entry(&transfer.from).or_insert_with(|| UserStats {
            address: transfer.from.clone(),
            total_volume: 0.0,
            avg_buy_price: 0.0,
            avg_sell_price: 0.0,
            max_balance: 0.0,
        });

        sender.total_volume += transfer.amount;
        sender.avg_sell_price = if sender.avg_sell_price == 0.0 {
            transfer.usd_price
        } else {
            (sender.avg_sell_price + transfer.usd_price) / 2.0
        };
        sender.max_balance = sender.max_balance.max(transfer.amount);

        let receiver = stats.entry(&transfer.to).or_insert_with(|| UserStats {
            address: transfer.to.clone(),
            total_volume: 0.0,
            avg_buy_price: 0.0,
            avg_sell_price: 0.0,
            max_balance: 0.0,
        });

        receiver.total_volume += transfer.amount;
        receiver.avg_buy_price = if receiver.avg_buy_price == 0.0 {
            transfer.usd_price
        } else {
            (receiver.avg_buy_price + transfer.usd_price) / 2.0
        };
        receiver.max_balance = receiver.max_balance.max(transfer.amount);
    }

    Ok(stats.into_values().collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Transfer;

    #[test]
    fn test_avg_buy_price_calculation() -> anyhow::Result<()> {
        let transfer = Transfer {
            from: "0xSender".to_string(),
            to: "0xReceiver".to_string(),
            amount: 100.0,
            usd_price: 1.5,
            ts: 1234567890,
        };

        let stats = calculate_user_stats(&[transfer])?;

        let receiver_stats = stats.iter()
            .find(|s| s.address == "0xReceiver")
            .unwrap();

        assert_eq!(
            receiver_stats.avg_buy_price, 1.5,
            "avg_buy_price должен равняться цене транзакции"
        );

        Ok(())
    }

    #[test]
    fn test_multiple_transfers_avg() -> anyhow::Result<()> {
        let transfers = vec![
            Transfer {
                from: "0xSender1".to_string(),
                to: "0xReceiver".to_string(),
                amount: 100.0,
                usd_price: 1.0,
                ts: 1,
            },
            Transfer {
                from: "0xSender2".to_string(),
                to: "0xReceiver".to_string(),
                amount: 100.0,
                usd_price: 3.0,
                ts: 2,
            },
        ];

        let stats = calculate_user_stats(&transfers)?;
        let receiver_stats = stats.iter()
            .find(|s| s.address == "0xReceiver")
            .unwrap();

        assert_eq!(
            receiver_stats.avg_buy_price, 2.0,
            "Средняя цена должна быть (1.0 + 3.0) / 2 = 2.0"
        );

        Ok(())
    }
}
