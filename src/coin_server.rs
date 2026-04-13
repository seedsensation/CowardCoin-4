use crate::coin::Coin;
use crate::serenity::UserId;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::iter::Map;
use std::sync::mpsc;

trait Sendable: fmt::Display {}

impl Sendable for String {}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoinServer {
    coin_data: HashMap<i32, Vec<i32>>,
}

impl CoinServer {
    pub fn new() -> CoinServer {
        CoinServer::load()
    }

    pub fn load() -> CoinServer {
        let file = fs::read_to_string("data.txt").unwrap_or("{}".to_string());
        serde_json::from_str(&file).unwrap()
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        fs::write("data.txt", serde_json::to_string(self).unwrap())
    }
}
