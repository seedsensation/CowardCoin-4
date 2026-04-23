use std::{alloc::GlobalAlloc, sync::Arc};

use serenity::{
    Result,
    all::{CacheHttp, Guild, GuildId, Http, Message, User},
};
use tokio::sync::mpsc::{Receiver, Sender};

pub enum Command {
    GetCoin(BotUser),
    CoinCount(BotUser),
    CoinCountMultiple(Vec<BotUser>),
    CoinLeaderboard(BotUser),
    GiveCoin(BotUser, Vec<BotUser>),
    CreateCoin,
    CreateCoinCheck,
    CoinCreateNotification(Message, Arc<Http>),
    DeleteCoinMessage,
}

#[derive(Debug)]
pub struct CoinMessage {
    pub msg: Message,
    pub http: Arc<Http>,
}
impl CoinMessage {
    pub async fn delete(&mut self) -> Result<()> {
        self.msg.delete(&self.http).await
    }
}

pub struct Request {
    pub command: Command,
    pub reply_to: Sender<Option<String>>,
}

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
