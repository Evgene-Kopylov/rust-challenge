mod generator;
mod model;
mod pipeline;
mod storage;

use generator::{DefaultTransferGenerator, TransferGenerator};
use pipeline::calculate_user_stats;
use storage::{MockStorage, Storage as _};

fn main() -> anyhow::Result<()> {
    let transfers = DefaultTransferGenerator::default().generate(10_000)?;

    let mut storage = MockStorage::default();
    storage.save_transfers(&transfers)?;

    let stats = calculate_user_stats(&transfers)?;
    for stat in stats {
        storage.save_user_stats(&stat)?;
    }

    for (i, transfer) in storage.transfers.iter().take(3).enumerate() {
        if let Some(user_stats) = storage.get_user_stats(&transfer.from)? {
            println!("{}. {}: {:#?}", i + 1, transfer.from, user_stats);
        } else {
            println!("{}. {}: No stats available", i + 1, transfer.from);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transfer_processing() -> anyhow::Result<()> {
        let generator = DefaultTransferGenerator::default();

        let transfers = generator.generate(1)?;
        let test_transfer = &transfers[0];

        let mut storage = MockStorage::default();

        storage.save_transfers(&transfers)?;
        let stats = calculate_user_stats(&transfers)?;

        for stat in stats {
            storage.save_user_stats(&stat)?;
        }

        assert_eq!(
            storage.transfers.len(),
            1,
            "Транзакция не сохранена в хранилище"
        );

        let stored_transfer = &storage.transfers[0];

        assert_eq!(
            stored_transfer.ts, test_transfer.ts,
            "Некорректное время транзакции"
        );

        assert!(
            stored_transfer.from.starts_with("0x") && stored_transfer.from.len() == 12,
            "Некорректный адрес отправителя: {}",
            stored_transfer.from
        );

        assert!(
            stored_transfer.to.starts_with("0x") && stored_transfer.to.len() == 12,
            "Некорректный адрес получателя: {}",
            stored_transfer.to
        );

        assert!(
            stored_transfer.amount >= generator.config.min_amount
                && stored_transfer.amount <= generator.config.max_amount,
            "Сумма {} выходит за пределы диапазона [{}, {}]",
            stored_transfer.amount,
            generator.config.min_amount,
            generator.config.max_amount
        );

        assert!(
            stored_transfer.usd_price >= generator.config.min_price
                && stored_transfer.usd_price <= generator.config.max_price,
            "Цена {} выходит за пределы диапазона [{}, {}]",
            stored_transfer.usd_price,
            generator.config.min_price,
            generator.config.max_price
        );

        let from_stats = storage.get_user_stats(&test_transfer.from)?.unwrap();
        let to_stats = storage.get_user_stats(&test_transfer.to)?.unwrap();

        assert_eq!(
            from_stats.total_volume, test_transfer.amount,
            "Некорректный объем операций отправителя"
        );

        assert_eq!(
            to_stats.total_volume, test_transfer.amount,
            "Некорректный объем операций получателя"
        );

        assert_eq!(
            from_stats.avg_sell_price, test_transfer.usd_price,
            "Некорректная средняя цена продажи"
        );

        assert_eq!(
            to_stats.avg_buy_price, test_transfer.usd_price,
            "Некорректная средняя цена покупки"
        );

        Ok(())
    }
}
