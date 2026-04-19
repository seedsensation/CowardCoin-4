extern crate dotenv;

pub mod bot;
mod coin;
pub mod communication;
pub mod helpers;
mod rarity;
mod server;
mod user;

use dotenv::dotenv;
use serenity::prelude::*;
use std::env;
use tokio::{
    sync::mpsc::{Receiver, Sender, channel},
    task,
};

pub use coin::Coin;
pub use rarity::Rarity;

use bot::Handler;
use communication::Request;

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
