use std::fs;

use tokio::sync::mpsc::Receiver;

use crate::Coin;
use crate::Rarity;
use crate::communication::BotUser;
use crate::communication::{Command, Request};
use crate::helpers::s_if;
use crate::user::CoinUser;

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    pub users: Vec<CoinUser>,
    pub coin: Coin,
}

impl Server {
    pub fn new() -> Self {
        serde_json::from_reader(
            fs::OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open("./data.json")
                .unwrap(),
        )
        .unwrap_or_else(|_| Self {
            users: vec![],
            coin: Coin::none(),
        })
    }

    pub fn save(&mut self) -> Result<()> {
        println!("Saving file...");
        self.users.sort();
        serde_json::to_writer_pretty(
            {
                fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open("./data.json")
                    .unwrap_or_else(|why| panic!("Error opening file: {why:?}"))
            },
            self,
        )
    }

    pub async fn start(mut receiver: Receiver<Request>) {
        let mut server = Self::new();
        let _ = server.save();
        loop {
            println!("Server running!");
            if let Some(request) = receiver.recv().await {
                if let Err(why) = request
                    .reply_to
                    .send(match request.command {
                        // get coin - not implemented
                        Command::GetCoin(user) => server.get_coin(user),

                        // coin count
                        Command::CoinCount(user) => Some(server.coin_count(vec![user]).await),
                        Command::CoinCountMultiple(users) => Some(server.coin_count(users).await),

                        // coin leaderboard
                        Command::CoinLeaderboard(_id) => None,

                        // give coin
                        Command::GiveCoin(_sender, _recipient) => None,

                        Command::CreateCoin => server.create_coin(),
                    })
                    .await
                {
                    println!("Error sending message: {why:?}");
                }
            }
            server.save().ok();
        }
    }
    async fn coin_count(&mut self, users: Vec<BotUser>) -> String {
        self.users.sort();
        let mut output: String = "".into();
        for temp_user in users {
            let user = self.get_user_from_id(&temp_user);
            output.push_str(&format!(
                "{} has {} coin{}.\n",
                user.nickname.clone().unwrap_or(temp_user.display_name),
                user.coins,
                s_if(user.coins)
            ));
        }
        output
    }

    fn get_user_from_id(&mut self, user: &BotUser) -> &CoinUser {
        self.users.sort();
        if let Ok(v) = self.users.binary_search_by_key(&user.id, |x| x.id) {
            {
                let new_user = self.users.get_mut(v).unwrap();
                if user.nickname != new_user.nickname && user.nickname.is_some() {
                    new_user.nickname = user.nickname.clone();
                }
            }
            self.users.get(v).unwrap()
        } else {
            println!("Creating new item for user {}", user.id);
            self.users
                .push(CoinUser::new(user.id, user.nickname.clone()));
            self.users.last().unwrap()
        }
    }
    fn get_mut_user_from_id(&mut self, user: &BotUser) -> &mut CoinUser {
        self.users.sort();
        if let Ok(v) = self.users.binary_search_by_key(&user.id, |x| x.id) {
            let new_user = self.users.get_mut(v).unwrap();
            if user.nickname != new_user.nickname && user.nickname.is_some() {
                new_user.nickname = user.nickname.clone();
            }
            new_user
        } else {
            self.users
                .push(CoinUser::new(user.id, user.nickname.clone()));
            let output = self.users.last_mut().unwrap();
            output
        }
    }

    fn get_coin(&mut self, user: BotUser) -> Option<String> {
        if !self.coin.is_none() {
            {
                self.get_mut_user_from_id(&user).coins += self.coin.value;
                self.clear_coin();
            }
            let user = self.get_user_from_id(&user);
            Some(format!(
                "You got a coin!\nYou now have {} coin{}.",
                user.coins,
                s_if(user.coins)
            ))
        } else {
            None
        }
    }

    fn create_coin(&mut self) -> Option<String> {
        if !self.coin.is_none() {
            Some("A coin already exists!".into())
        } else {
            self.coin = Coin::new();
            Some(self.coin.arrival_message())
        }
    }
    fn clear_coin(&mut self) {
        self.coin = Coin::none();
    }
}
