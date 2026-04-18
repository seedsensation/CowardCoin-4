use tokio::sync::mpsc::Receiver;

use crate::communication::DiscordUser;
use crate::communication::{Command, Request};

pub struct Server {}

impl Server {
    pub async fn start(mut receiver: Receiver<Request>) {
        let mut server = Self {};
        loop {
            println!("Server running!");
            if let Some(request) = receiver.recv().await {
                if let Err(why) = request
                    .reply_to
                    .send(match request.command {
                        // get coin - not implemented
                        Command::GetCoin => None,

                        // coin count
                        Command::CoinCount(user) => Some(server.coin_count(vec![user]).await),
                        Command::CoinCountMultiple(users) => Some(server.coin_count(users).await),

                        // coin leaderboard
                        Command::CoinLeaderboard(_id) => None,

                        // give coin
                        Command::GiveCoin(_sender, _recipient) => None,
                    })
                    .await
                {
                    println!("Error sending message: {why:?}");
                }
            }
        }
    }
    async fn coin_count(&self, users: Vec<DiscordUser>) -> String {
        let mut output: String = "".into();
        for user in users {
            output.push_str(&format!("{} has 0 coins.\n", user.display_name));
        }
        output
    }
}
