use crate::helpers::default_timestamp;
use serde::{Deserialize, Serialize};
use serenity::all::{CacheHttp, GuildId, User};
use std::time::SystemTime;

#[derive(Debug, Serialize, Deserialize, Clone, Ord, Eq)]
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
            id: id,
            display_name: display_name,
            nickname: nickname,
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

#[derive(Debug)]
pub struct BotUser {
    pub display_name: String,
    pub nickname: Option<String>,
    pub id: u64,
}

impl From<User> for BotUser {
    fn from(value: User) -> Self {
        Self {
            display_name: value.display_name().to_string(),
            nickname: None,
            id: value.id.get(),
        }
    }
}
impl From<&User> for BotUser {
    fn from(value: &User) -> Self {
        Self {
            display_name: value.display_name().to_string(),
            nickname: None,
            id: value.id.get(),
        }
    }
}

impl BotUser {
    pub async fn from_user<T>(user: &User, http: T, guild_id: Option<GuildId>) -> Self
    where
        T: CacheHttp,
    {
        Self {
            display_name: user.display_name().into(),
            nickname: match guild_id {
                Some(g) => user.nick_in(http, g).await,
                _ => None,
            },
            id: user.id.get(),
        }
    }
}
