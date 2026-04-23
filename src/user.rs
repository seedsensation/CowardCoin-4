use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Ord, Eq)]
pub struct CoinUser {
    pub id: u64,

    #[serde(default)]
    pub nickname: Option<String>,

    #[serde(default)]
    pub coins: i64,
}

impl CoinUser {
    pub fn new(id: u64, nickname: Option<String>) -> Self {
        Self {
            id: id,
            nickname: nickname,
            coins: 0,
        }
    }
}

impl PartialEq for CoinUser {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialOrd for CoinUser {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}
