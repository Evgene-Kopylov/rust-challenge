mod model;
mod generator;
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
    for stat in stats.iter() {
        storage.save_user_stats(stat)?;
    }
    
    // Выводим топ-10 пользователей из хранилища
    for (i, transfer) in storage.transfers.iter()
        .take(3)
        .enumerate()
    {
        if let Some(user_stats) = storage.get_user_stats(&transfer.from)? {
            println!("{}. {}: {:#?}", i+1, transfer.from, user_stats);
        } else {
            println!("{}. {}: No stats available", i+1, transfer.from);
        }
    }

    Ok(())
}

