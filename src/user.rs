use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Ord, Eq)]
pub struct User {
    pub id: u64,

    #[serde(default)]
    pub coins: i64,
}

impl User {
    pub fn new(id: u64) -> Self {
        Self { id: id, coins: 0 }
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialOrd for User {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}
