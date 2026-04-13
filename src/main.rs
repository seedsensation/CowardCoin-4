extern crate dotenv;

mod coin;
mod coin_server;
mod coin_type;
mod messages;

use dotenv::dotenv;
use poise::serenity_prelude::{self as serenity};
use serenity::model::channel::Message;
use serenity::{async_trait, prelude::*};
use std::env;

struct Handler {}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.to_lowercase() == "get coin" || msg.content.to_lowercase() == "coin get" {
            println!("get coin message received");
            messages::send_message(msg.channel_id, &ctx.http, "hi").await;
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    for _ in 1..100 {
        let coin = coin::Coin::new(coin_type::CoinType::choose_coin());
        println!(
            "{}\n{}\n{}\n",
            coin.coin_type.value_range().min().unwrap(),
            coin.coin_type.value_range().max().unwrap(),
            coin.coin_arrival_message()
        )
    }

    let token = env::var("DISCORD_TOKEN").expect("No token supplied");

    // set intents
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // create client, logging in with the token
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler {})
        .await
        .expect("Error creating client.");
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
