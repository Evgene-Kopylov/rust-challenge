use std::collections::HashMap;

use crate::model::{Transfer, UserStats};

/// Интерфейс для работы с хранилищем данных
pub trait Storage {
    fn save_transfers(&mut self, transfers: &[Transfer]) -> anyhow::Result<()>;
    fn save_user_stats(&mut self, stats: &UserStats) -> anyhow::Result<()>;
    fn get_user_stats(&self, address: &str) -> anyhow::Result<Option<UserStats>>;
}

/// Mock-реализация хранилища для тестирования
#[derive(Default)]
pub struct MockStorage {
    pub transfers: Vec<Transfer>,
    stats: HashMap<String, UserStats>,
}

impl Storage for MockStorage {
    fn save_transfers(&mut self, transfers: &[Transfer]) -> anyhow::Result<()> {
        self.transfers.extend_from_slice(transfers);
        Ok(())
    }

    fn save_user_stats(&mut self, stats: &UserStats) -> anyhow::Result<()> {
        self.stats.insert(stats.address.clone(), stats.clone());
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
        let transfers = vec![Transfer::default(), Transfer::default()];

        assert!(storage.save_transfers(&transfers).is_ok());
        assert_eq!(storage.transfers.len(), 2);
    }
}
