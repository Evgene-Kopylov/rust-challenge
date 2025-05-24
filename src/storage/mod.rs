use std::collections::HashMap;

use crate::model::{Transfer, UserStats};

pub trait Storage {
    fn save_transfers(&self, transfers: &[Transfer]) -> anyhow::Result<()>;
    fn get_user_stats(&self, address: &str) -> anyhow::Result<Option<UserStats>>;
}

#[derive(Default)]
pub struct MockStorage {
    transfers: Vec<Transfer>,
    stats: HashMap<String, UserStats>,
}
