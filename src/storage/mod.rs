use std::collections::HashMap;

use crate::model::{Transfer, UserStats};

pub trait Storage {
    fn save_transfers(&mut self, transfers: &[Transfer]) -> anyhow::Result<()>;
    fn get_user_stats(&self, address: &str) -> anyhow::Result<Option<UserStats>>;
}

#[derive(Default)]
pub struct MockStorage {
    transfers: Vec<Transfer>,
    stats: HashMap<String, UserStats>,
}

impl Storage for MockStorage {
    fn save_transfers(&mut self, transfers: &[Transfer]) -> anyhow::Result<()> {
        self.transfers.extend_from_slice(transfers);
        Ok(())
    }

    fn get_user_stats(&self, address: &str) -> anyhow::Result<Option<UserStats>> {
        Ok(self.stats.get(address).cloned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_save_transfers() {
        let mut storage = MockStorage::default();
        let transfers = vec![
            Transfer::default(),
            Transfer::default()
        ];
        
        assert!(storage.save_transfers(&transfers).is_ok());
        assert_eq!(storage.transfers.len(), 2);
    }
}
