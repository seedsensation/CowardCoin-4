use crate::User;
use tokio::sync::mpsc::{Receiver, Sender};

pub enum Command {
    GetCoin,
    CoinCount(u64),
    CoinCountMultiple(Vec<u64>),
    CoinLeaderboard(u64),
    GiveCoin(u64, Vec<u64>),
}

pub struct Request {
    pub command: Command,
    pub reply_to: Sender<Option<String>>,
}
