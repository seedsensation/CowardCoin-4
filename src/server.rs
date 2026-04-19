use std::fs;

use tokio::sync::mpsc::Receiver;

use crate::communication::DiscordUser;
use crate::communication::{Command, Request};
use crate::user::User;

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    pub users: Vec<User>,
    pub coin_present: bool,
}

impl Server {
    pub fn new() -> Self {
        serde_json::from_reader(
            fs::OpenOptions::new()
                .read(true)
                .create(false)
                .open("./data.json")
                .unwrap(),
        )
        .unwrap_or_else(|_| Self {
            users: vec![],
            coin_present: false,
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
                        Command::GetCoin(user) => server.get_coin(user.id),

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
    async fn coin_count(&mut self, users: Vec<DiscordUser>) -> String {
        self.users.sort();
        let mut output: String = "".into();
        for temp_user in users {
            let user = self.get_user_from_id(temp_user.id);
            output.push_str(&format!(
                "{} has {} coin{}.\n",
                temp_user.display_name,
                user.coins,
                s_if(user.coins)
            ));
        }
        output
    }

    fn get_user_from_id(&mut self, id: u64) -> &User {
        self.users.sort();
        if let Ok(v) = self.users.binary_search_by_key(&id, |x| x.id) {
            self.users.get(v).unwrap()
        } else {
            println!("Creating new item for user {id}");
            self.users.push(User::new(id));
            self.users.last().unwrap()
        }
    }
    fn get_mut_user_from_id(&mut self, id: u64) -> &mut User {
        self.users.sort();
        if let Ok(v) = self.users.binary_search_by_key(&id, |x| x.id) {
            self.users.get_mut(v).unwrap()
        } else {
            self.users.push(User::new(id));
            let output = self.users.last_mut().unwrap();
            output
        }
    }

    fn get_coin(&mut self, id: u64) -> Option<String> {
        if self.coin_present {
            self.get_mut_user_from_id(id).coins += 1;
            let user = self.get_user_from_id(id);
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
        if self.coin_present {
            Some("A coin already exists!".into())
        } else {
            self.coin_present = true;
            Some("A coin appeared!".into())
        }
    }
}

fn s_if(val: i64) -> String {
    match val {
        1 => "",
        _ => "s",
    }
    .into()
}
