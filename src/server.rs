use std::fs;

use tokio::sync::mpsc::Receiver;

use crate::Coin;
use crate::Rarity;
use crate::commands::CoinCommands;
use crate::communication::BotUser;
use crate::communication::{CoinMessage, Command, Request};
use crate::games::Games;
use crate::helpers::random_between;
use crate::helpers::s_if;
use crate::user::CoinUser;

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    pub users: Vec<CoinUser>,
    #[serde(skip)]
    pub coin: Coin,
    #[serde(skip)]
    pub coin_message: Option<CoinMessage>,
}

pub trait ExecuteCommands: CoinCommands {
    async fn execute_command(&mut self, command: Command) -> Option<String>;
}

impl<T> ExecuteCommands for T
where
    T: CoinCommands + Games,
{
    async fn execute_command(&mut self, command: Command) -> Option<String> {
        use Command::*;
        match command {
            GetCoin(bot_user) => self.get_coin(bot_user).await,
            CoinCount(bot_user) => Some(self.coin_count(vec![bot_user])),
            CoinCountMultiple(bot_users) => Some(self.coin_count(bot_users)),
            CoinLeaderboard(bot_user) => Some(self.coin_leaderboard(bot_user)),
            GiveCoin(sender, recipient, amount) => Some(self.give_coin(sender, recipient, amount)),
            CreateCoin => self.create_coin(),
            CoinCreateNotification(message, http) => {
                self.set_coin_message(message, http).await;
                None
            }
            Arena(bot_user, items) => Some(self.arena(bot_user, items)),
        }
    }
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
            coin_message: None,
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
        println!("Server running!");
        loop {
            if let Some(request) = receiver.recv().await {
                if let Err(why) = request
                    .reply_to
                    .send(server.execute_command(request.command).await)
                    .await
                {
                    println!("Error sending message: {why:?}");
                }
            }
        }
    }

    pub fn get_user_from_id(&mut self, user: &BotUser) -> &CoinUser {
        self.users.sort();
        if let Ok(v) = self.users.binary_search_by_key(&user.id, |x| x.id) {
            {
                let new_user = self.users.get_mut(v).unwrap();
                new_user.display_name = user.display_name.clone();
                if user.nickname != new_user.nickname && user.nickname.is_some() {
                    new_user.nickname = user.nickname.clone();
                }
            }
            self.users.get(v).unwrap()
        } else {
            println!("Creating new item for user {}", user.id);
            self.users.push(CoinUser::new(
                user.id,
                user.nickname.clone(),
                user.display_name.clone(),
            ));
            self.users.last().unwrap()
        }
    }

    pub fn get_mut_users_from_ids(
        &mut self,
        sender: &BotUser,
        recipient: &BotUser,
    ) -> (&mut CoinUser, &mut CoinUser) {
        let sender_index = self
            .users
            .binary_search_by_key(&sender.id, |x| x.id)
            .unwrap();
        let recipient_index = self
            .users
            .binary_search_by_key(&recipient.id, |x| x.id)
            .unwrap();

        // baby's first unsafe block
        unsafe {
            let ptr = self.users.as_mut_ptr();
            (&mut *ptr.add(sender_index), &mut *ptr.add(recipient_index))
        }
    }

    //pub fn get_user_from_id(&mut self, user: &BotUser) -> Option<&mut CoinUser> {
    //    match self.users.binary_search_by_key(&user.id, |x| x.id) {
    //        Ok(v) => Some(unsafe { &mut *(self.users.as_mut_ptr().add(v)) }),
    //        Err(_) => None,
    //    }
    //}

    pub fn get_mut_user_from_id(&mut self, user: &BotUser) -> &mut CoinUser {
        self.users.sort();
        if let Ok(v) = self.users.binary_search_by_key(&user.id, |x| x.id) {
            let new_user = self.users.get_mut(v).unwrap();
            new_user.display_name = user.display_name.clone();
            if user.nickname != new_user.nickname && user.nickname.is_some() {
                new_user.nickname = user.nickname.clone();
            }
            new_user
        } else {
            self.users.push(CoinUser::new(
                user.id,
                user.nickname.clone(),
                user.display_name.clone(),
            ));
            let output = self.users.last_mut().unwrap();
            output
        }
    }

    pub fn clear_coin(&mut self) {
        self.coin = Coin::none();
    }
}
