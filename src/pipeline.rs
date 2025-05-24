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

pub fn make_transaction(
    storage: &mut impl crate::storage::Storage,
    transfer: &Transfer,
) -> anyhow::Result<()> {
    // Получаем текущую статистику отправителя (если есть)
    let mut sender_stats = storage
        .get_user_stats(&transfer.from)?
        .unwrap_or_else(|| UserStats {
            address: transfer.from.clone(),
            total_volume: 0.0,
            avg_buy_price: 0.0,
            avg_sell_price: 0.0,
            max_balance: 0.0,
        });

    // Обновляем статистику отправителя
    sender_stats.total_volume += transfer.amount;
    sender_stats.avg_sell_price = if sender_stats.avg_sell_price == 0.0 {
        transfer.usd_price
    } else {
        (sender_stats.avg_sell_price + transfer.usd_price) / 2.0
    };
    sender_stats.max_balance = sender_stats.max_balance.max(transfer.amount);

    // Получаем текущую статистику получателя (если есть)
    let mut receiver_stats = storage
        .get_user_stats(&transfer.to)?
        .unwrap_or_else(|| UserStats {
            address: transfer.to.clone(),
            total_volume: 0.0,
            avg_buy_price: 0.0,
            avg_sell_price: 0.0,
            max_balance: 0.0,
        });

    // Обновляем статистику получателя
    receiver_stats.total_volume += transfer.amount;
    receiver_stats.avg_buy_price = if receiver_stats.avg_buy_price == 0.0 {
        transfer.usd_price
    } else {
        (receiver_stats.avg_buy_price + transfer.usd_price) / 2.0
    };
    receiver_stats.max_balance = receiver_stats.max_balance.max(transfer.amount);

    storage.save_transfers(&[transfer.clone()])?;
    storage.save_user_stats(&sender_stats)?;
    storage.save_user_stats(&receiver_stats)?;

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Transfer;
    use crate::pipeline;
    use crate::storage::Storage;

    #[test]
    fn test_make_transaction() -> anyhow::Result<()> {
        use crate::storage::MockStorage;

        let mut storage = MockStorage::default();
        let transfer = Transfer {
            from: "0xSender".to_string(),
            to: "0xReceiver".to_string(),
            amount: 100.0,
            usd_price: 1.5,
            ts: 1234567890,
        };

        make_transaction(&mut storage, &transfer)?;

        assert_eq!(storage.transfers.len(), 1);
        assert_eq!(storage.transfers[0].amount, 100.0);

        let sender_stats = storage.get_user_stats("0xSender")?.unwrap();
        assert_eq!(sender_stats.total_volume, 100.0);
        assert_eq!(sender_stats.avg_sell_price, 1.5);

        let receiver_stats = storage.get_user_stats("0xReceiver")?.unwrap();
        assert_eq!(receiver_stats.total_volume, 100.0);
        assert_eq!(receiver_stats.avg_buy_price, 1.5);

        Ok(())
    }

    #[test]
    fn test_avg_buy_price_calculation() -> anyhow::Result<()> {
        use crate::storage::MockStorage;

        let mut storage = MockStorage::default();
        let transfer = Transfer {
            from: "0xSender".to_string(),
            to: "0xReceiver".to_string(),
            amount: 100.0,
            usd_price: 1.5,
            ts: 1234567890,
        };

        pipeline::make_transaction(&mut storage, &transfer)?;
        let receiver_stats = storage.get_user_stats("0xReceiver")?.unwrap();

        assert_eq!(
            receiver_stats.avg_buy_price, 1.5,
            "avg_buy_price должен равняться цене транзакции"
        );

        Ok(())
    }

    #[test]
    fn test_multiple_transfers_avg() -> anyhow::Result<()> {
        use crate::storage::MockStorage;

        let mut storage = MockStorage::default();
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

        for transfer in &transfers {
            pipeline::make_transaction(&mut storage, transfer)?;
        }

        let receiver_stats = storage.get_user_stats("0xReceiver")?.unwrap();
        assert_eq!(
            receiver_stats.avg_buy_price, 2.0,
            "Средняя цена должна быть (1.0 + 3.0) / 2 = 2.0"
        );

        Ok(())
    }
}
