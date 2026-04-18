use crate::User;
use crate::communication::*;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;
use std::env;
use std::fmt::format;
use tokio::sync::mpsc::error::SendError;

use tokio::sync::mpsc::{Receiver, Sender, channel};

pub struct Handler {
    pub sender: Sender<Request>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        println!("Message received!");
        if msg.content.to_lowercase().starts_with("coin")
            || msg.content.to_lowercase().starts_with("get")
        {
            println!("command received");
            if let Some(message) = match msg
                .content
                .to_lowercase()
                .split(" ")
                .collect::<Vec<&str>>()
                .get(1)
            {
                // get coin
                Some(&"get") | Some(&"coin") | None => self.send_command(Command::GetCoin).await,
                // coin count
                Some(&"count") => match msg.mentions.is_empty() {
                    true => {
                        self.send_command(Command::CoinCount(msg.author.id.get()))
                            .await
                    }
                    false => {
                        self.send_command(Command::CoinCountMultiple(
                            msg.mentions
                                .iter()
                                .map(|x| x.id.get())
                                .collect::<Vec<u64>>(),
                        ))
                        .await
                    }
                },
                // coin leaderboard
                Some(&"leaderboard") => {
                    self.send_command(Command::CoinLeaderboard(msg.author.id.get()))
                        .await
                }
                // give coin
                Some(&"give") => match msg.mentions.is_empty() {
                    true => Some("Please make sure you are giving someone a coin.".into()),
                    false => {
                        self.send_command(Command::GiveCoin(
                            msg.author.id.get(),
                            msg.mentions
                                .iter()
                                .map(|x| x.id.get())
                                .collect::<Vec<u64>>(),
                        ))
                        .await
                    }
                },
                _ => {
                    println!("Unrecognised command...");
                    None
                }
            } {
                if let Err(why) = msg.channel_id.say(&ctx.http, message).await {
                    println!("Error sending message: {why:?}");
                }
            } else {
                println!("oh :(");
            }
        }
    }
}

impl Handler {
    async fn send_command(&self, command: Command) -> Option<String> {
        let (tx, mut rx) = channel::<Option<String>>(100);
        if let Err(e) = self
            .sender
            .send(Request {
                command: command,
                reply_to: tx,
            })
            .await
        {
            return Some(format!(
                "There was an error communicating with the server: {e:?}"
            ));
        };
        if let Some(message) = rx.recv().await {
            return message;
        } else {
            return Some("There was an error communicating with the server.".into());
        }
    }
}
