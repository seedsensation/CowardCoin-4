use tokio::sync::mpsc::Receiver;

use crate::communication::{Command, Request};

pub struct Server {}

impl Server {
    pub async fn start(mut receiver: Receiver<Request>) {
        let mut server = Self {};
        loop {
            println!("Server running!");
            if let Some(request) = receiver.recv().await {
                println!("doing something...");
                if let Err(why) = request
                    .reply_to
                    .send(match request.command {
                        // get coin - not implemented
                        Command::GetCoin => None,

                        // coin count
                        Command::CoinCount(id) => Some(server.coin_count(vec![id]).await),
                        Command::CoinCountMultiple(ids) => Some(server.coin_count(ids).await),

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

    async fn coin_count(&self, ids: Vec<u64>) -> String {
        let mut output: String = "".into();
        for id in ids {
            output.push_str(&id.to_string());
            output.push_str("\n");
        }
        output
    }
}
