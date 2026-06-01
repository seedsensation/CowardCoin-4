extern crate dotenv;

pub mod bot;
mod coin;
mod commands;
pub mod communication;
mod games;
pub mod helpers;
mod rarity;
mod server;
mod user;

use dotenv::dotenv;
use serenity::prelude::*;
use std::env;
use std::time::Duration;
use tokio::{sync::mpsc::channel, task};

use bot::Handler;
use communication::Request;

pub use prelude::*;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let (tx, rx) = channel::<Request>(100);

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler { sender: tx })
        .await
        .expect("Error creating client...");

    task::spawn(server::Server::start(rx));
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }

    println!("Hello, world!");
}

// to make coins appear, make a new task
// that checks the time. if it's been long enough,
// send a CreateCoin command to the server.

pub mod environment {
    use crate::Duration;
    pub const TIME_BETWEEN_TRICKS: Duration = Duration::from_hours(4);
    pub const MARKET_CHANGE_TIMER: Duration = Duration::from_hours(1);
    pub const INVESTMENT_TIMER: Duration = Duration::from_mins(10);
    pub const BOT_ID: u64 = 813814751192809543;
    //pub const BOT_ID: u64 = 1023717268624003113;

    pub const COIN_TIME: u64 = 600;

    pub fn coin_channel() -> u64 {
        dotenv::dotenv().ok();
        str::parse(
            &std::env::var("COIN_CHANNEL").expect("Expected COIN_CHANNEL in the environment"),
        )
        .unwrap()
    }
}

pub mod prelude {
    pub use crate::coin::Coin;
    pub use crate::commands::CoinCommands;
    pub use crate::communication::BotUser;
    pub use crate::rarity::Rarity;
    pub use crate::user::CoinUser;
}
