use serenity::all::{CacheHttp, Guild, GuildId, Http, User};
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

impl DiscordUser {
    pub async fn from_user<T>(user: &User, http: T, guild_id: Option<GuildId>) -> Self
    where
        T: CacheHttp,
    {
        Self {
            display_name: match guild_id {
                Some(g) => user
                    .nick_in(http, g)
                    .await
                    .unwrap_or(user.display_name().into()),
                _ => user.display_name().into(),
            },
            id: user.id.get(),
        }
    }
}
