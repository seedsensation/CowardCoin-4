extern crate dotenv;

pub mod communication;
pub mod environment;
pub mod helpers;

mod coin_server;
mod discord_bot;

use coin_server::Server;
use discord_bot::*;

use dotenv::dotenv;
use serenity::prelude::*;
use std::{env, time::Duration};
use tokio::{sync::mpsc::channel, task};

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

    task::spawn(Server::start(rx));
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }

    println!("Hello, world!");
}

// to make coins appear, make a new task
// that checks the time. if it's been long enough,
// send a CreateCoin command to the server.
