use std::fs;

use std::time::SystemTime;
use tokio::sync::mpsc::Receiver;

use super::{Coin, CoinCommands, CoinUser};

use crate::discord_bot::BotUser;

use crate::communication::{CoinMessage, Request};

use crate::helpers::default_timestamp;

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    pub users: Vec<CoinUser>,
    #[serde(skip)]
    pub coin: Coin,
    #[serde(skip)]
    pub coin_message: Option<CoinMessage>,
    #[serde(default = "default_timestamp")]
    pub time_of_last_interest: SystemTime,
}

impl Server {
    pub fn new() -> Self {
        serde_json::from_reader(
            fs::OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .truncate(false)
                .open("./data.json")
                .unwrap(),
        )
        .unwrap_or_else(|_| Self {
            users: vec![],
            coin: Coin::none(),
            coin_message: None,
            time_of_last_interest: default_timestamp(),
        })
    }

    pub unsafe fn get_two_mut_users(
        &mut self,
        user1: &BotUser,
        user2: &BotUser,
    ) -> (&mut CoinUser, &mut CoinUser) {
        self.sort_by_ids();
        let ptr = self.users.as_mut_ptr();
        unsafe {
            (
                &mut *ptr.add(crate::get_index_from_id!(user1 in self)),
                &mut *ptr.add(crate::get_index_from_id!(user2 in self)),
            )
        }
    }

    pub fn save(&mut self) -> Result<()> {
        println!("Saving file...");
        self.sort_by_ids();
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
            if let Some(request) = receiver.recv().await
                && let Err(why) = request
                    .reply_to
                    .send(server.run_command(request.command).await)
                    .await
            {
                println!("Error sending message: {why:?}");
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

    #[inline]
    pub fn sort_by_ids(&mut self) {
        self.users.sort_by_key(|x| x.id);
    }

    pub fn get_mut_user_from_id(&mut self, user: &BotUser) -> &mut CoinUser {
        self.sort_by_ids();
        match self.users.binary_search_by_key(&user.id, |x| x.id) {
            Ok(v) => self.users.get_mut(v).unwrap(),
            Err(_) => {
                self.users.push(CoinUser::new(
                    user.id,
                    user.nickname.clone(),
                    user.display_name.clone(),
                ));
                self.users.last_mut().unwrap()
            }
        }
    }
}
