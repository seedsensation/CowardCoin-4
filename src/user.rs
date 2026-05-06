use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Ord, Eq)]
pub struct CoinUser {
    pub id: u64,

    #[serde(default)]
    pub nickname: Option<String>,

    #[serde(default)]
    pub coins: i64,

    #[serde(default)]
    pub xp: i64,

    #[serde(default)]
    pub level: i64,
}

impl CoinUser {
    pub fn new(id: u64, nickname: Option<String>) -> Self {
        Self {
            id: id,
            nickname: nickname,
            coins: 0,
            xp: 0,
            level: 1,
        }
    }

    pub fn add_xp(&mut self, amount: i64) {
        self.xp += amount;
        while self.xp >= 100 {
            self.xp -= 100;
            self.level += 1;
        }
    }
    pub fn add_xp_with_response(&mut self, amount: i64) -> String {
        let level = self.level.clone();
        self.add_xp(amount);
        if self.level > level {
            format!("\nYou are now Level {}!", self.level)
        } else {
            "".into()
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
