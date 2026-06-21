use crate::helpers::default_timestamp;
use crate::helpers::*;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Serialize, Deserialize, Clone, Eq)]
pub struct CoinUser {
    pub id: u64,

    #[serde(default)]
    pub nickname: Option<String>,

    #[serde(default)]
    pub display_name: String,

    #[serde(default)]
    pub coins: i64,

    #[serde(default)]
    pub style_points: i64,

    #[serde(default)]
    pub xp: i64,

    #[serde(default)]
    pub level: i64,

    #[serde(default = "default_timestamp")]
    pub time_of_last_trick: SystemTime,

    #[serde(default = "default_timestamp")]
    pub time_of_last_investment: SystemTime,
}

impl CoinUser {
    pub fn new(id: u64, nickname: Option<String>, display_name: String) -> Self {
        Self {
            id,
            display_name,
            nickname,
            coins: 0,
            style_points: 0,
            xp: 0,
            level: 1,
            time_of_last_trick: SystemTime::UNIX_EPOCH,
            time_of_last_investment: SystemTime::UNIX_EPOCH,
        }
    }
    pub fn xp_cap(&self) -> i64 {
        (100.0 * f64::max(((self.level - 1) as f64 * 0.1) + 1.0, 1.0)) as i64
    }

    pub fn coin_count_message(&self) -> String {
        format!(
            "{}**{}** {}\n> - {} coin{}{}{}",
            if self.xp > 0 || self.level > 1 {
                format!("{} ", self.arena_title())
            } else {
                String::new()
            },
            self.nickname.as_ref().unwrap_or(&self.display_name),
            if self.xp > 0 || self.level > 1 {
                format!("(Lv. {})", self.level)
            } else {
                String::new()
            },
            self.coins,
            s_if(self.coins),
            if self.style_points > 0 {
                format!(
                    "\n> - {} StylePoint{}™",
                    self.style_points,
                    s_if(self.style_points),
                )
            } else {
                String::new()
            },
            if self.xp > 0 || self.level > 1 {
                format!("\n> - [{}] - {}/{}", self.xp_bar(), self.xp, self.xp_cap())
            } else {
                String::new()
            }
        )
    }

    pub fn add_xp(&mut self, amount: i64) {
        self.xp += amount;
        while self.xp >= 100 {
            self.xp -= 100;
            self.level += 1;
        }
    }
    pub fn add_xp_with_response(&mut self, amount: i64) -> String {
        let level = self.level;
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
        Some(self.cmp(other))
    }
}

impl Ord for CoinUser {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}
