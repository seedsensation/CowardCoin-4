use serenity::{
    Result,
    all::{Http, Message},
};
use std::sync::Arc;
use tokio::sync::mpsc::Sender;

use crate::discord_bot::BotUser;

#[derive(Debug)]
#[non_exhaustive]
pub enum Command {
    // user commands
    GetCoin(BotUser),
    CoinCount(BotUser),
    CoinCountMultiple(Vec<BotUser>),
    CoinLeaderboard(BotUser),
    GiveCoin(BotUser, BotUser, i64),
    EatCoin(BotUser, i64),
    Invest(BotUser, i64),
    Arena(BotUser, Vec<String>),
    Trick(BotUser, i64),
    TrickMax(BotUser),

    // internal commands
    CreateCoin,
    CoinCreateNotification(Box<Message>, Arc<Http>),
    UpdateCoins,
    ClearCoin,
    CoinEscape,
    NoCommand,
    Error(&'static str),
}

impl Command {
    pub fn should_save(&self) -> bool {
        use Command::*;
        match self {
	    // mark everything that should save as true
	    // put a comment on each explaining why it should save
	    // when this command is run

	    // when a coin is gotten it should save
            GetCoin(..)

		// when a coin is given it updates coin numbers
		| GiveCoin(..)
		// when a coin is eaten it updates coin numbers
		| EatCoin(..)
		// when a coin is invested it updates coin numbers
		| Invest(..)
		// when you level up it updates numbers
		| Arena(..)
		| Trick(..)
		| TrickMax(..)
		=> true,
            _ => false,
        }
    }
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
