use serenity::all::User;
use tokio::sync::mpsc::{Receiver, Sender};

pub enum Command {
    GetCoin(DiscordUser),
    CoinCount(DiscordUser),
    CoinCountMultiple(Vec<DiscordUser>),
    CoinLeaderboard(DiscordUser),
    GiveCoin(DiscordUser, Vec<DiscordUser>),
    CreateCoin,
}

pub struct Request {
    pub command: Command,
    pub reply_to: Sender<Option<String>>,
}

pub struct DiscordUser {
    pub display_name: String,
    pub id: u64,
}

impl From<User> for DiscordUser {
    fn from(value: User) -> Self {
        Self {
            display_name: value.display_name().to_string(),
            id: value.id.get(),
        }
    }
}
impl From<&User> for DiscordUser {
    fn from(value: &User) -> Self {
        Self {
            display_name: value.display_name().to_string(),
            id: value.id.get(),
        }
    }
}
