mod model;
mod generator;
mod pipeline;
mod storage;

use generator::{DefaultTransferGenerator, TransferGenerator};
use pipeline::calculate_user_stats;

fn main() -> anyhow::Result<()> {
    let transfers = DefaultTransferGenerator::default().generate(10_000)?;
    let stats = calculate_user_stats(&transfers);

    for stat in stats.iter().take(10) {
        println!("{:#?}", stat);
    }

    Ok(())
}

