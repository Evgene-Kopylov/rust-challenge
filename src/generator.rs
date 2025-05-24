use crate::model::Transfer;
use rand::{distributions::Alphanumeric, Rng};
use std::time::{SystemTime, UNIX_EPOCH};
use anyhow::Context;

pub trait TransferGenerator {
    fn generate(&self, count: usize) -> anyhow::Result<Vec<Transfer>>;
}

#[derive(Debug, Clone)]
pub struct TransferGenConfig {
    pub min_amount: f64,
    pub max_amount: f64,
    pub min_price: f64,
    pub max_price: f64,
    pub max_age_secs: u64,
}

impl Default for TransferGenConfig {
    fn default() -> Self {
        Self {
            min_amount: 1.0,
            max_amount: 1000.0,
            min_price: 0.1,
            max_price: 2.0,
            max_age_secs: 86_400 * 30,
        }
    }
}

#[derive(Default)]
pub struct DefaultTransferGenerator {
    pub config: TransferGenConfig,
}

impl TransferGenerator for DefaultTransferGenerator {
    fn generate(&self, count: usize) -> anyhow::Result<Vec<Transfer>> {
        if self.config.min_amount >= self.config.max_amount {
            return Err(anyhow::anyhow!("Invalid amount range: min >= max"));
        }
        if self.config.min_price >= self.config.max_price {
            return Err(anyhow::anyhow!("Invalid price range: min >= max"));
        }

        let mut rng = rand::thread_rng();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .context("Failed to get current time")?
            .as_secs();
        Ok((0..count)
            .map(|_| {
                let from = rand_address(&mut rng);
                let to = rand_address(&mut rng);
                let amount = rng.gen_range(self.config.min_amount..self.config.max_amount);
                let usd_price = rng.gen_range(self.config.min_price..self.config.max_price);
                let ts = now - rng.gen_range(0..self.config.max_age_secs);

                Transfer {
                    ts,
                    from,
                    to,
                    amount,
                    usd_price,
                }
            })
            .collect())
    }
}

fn rand_address(rng: &mut impl Rng) -> String {
    let suffix: String = rng
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();
    format!("0x{}", suffix)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_transfers() -> anyhow::Result<()> {
        let generator = DefaultTransferGenerator::default();
        let transfers = generator.generate(10)?;

        assert_eq!(transfers.len(), 10);
        for t in transfers {
            assert!(t.amount >= generator.config.min_amount);
            assert!(t.amount <= generator.config.max_amount);
            assert!(t.from.starts_with("0x"));
            assert!(t.to.starts_with("0x"));
        }
        Ok(())
    }

    #[test]
    fn test_invalid_config() {
        let generator = DefaultTransferGenerator {
            config: TransferGenConfig {
                min_amount: 100.0,
                max_amount: 10.0, // Невалидный диапазон
                ..Default::default()
            }
        };
        assert!(generator.generate(1).is_err());
    }
}

