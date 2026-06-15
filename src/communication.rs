use crate::prelude::*;

use serenity::{
    Result,
    all::{Http, Message},
};
use std::sync::Arc;
use tokio::sync::mpsc::Sender;

#[derive(Debug)]
pub enum Command {
    GetCoin(BotUser),
    CoinCount(BotUser),
    CoinCountMultiple(Vec<BotUser>),
    CoinLeaderboard(BotUser),
    GiveCoin(BotUser, BotUser, i64),
    CreateCoin,
    CoinCreateNotification(Message, Arc<Http>),
    Arena(BotUser, Vec<String>),
    UpdateCoins,
    ClearCoin,
    CoinEscape,
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
