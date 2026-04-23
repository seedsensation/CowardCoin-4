use crate::communication::*;

use serenity::all::{GuildId, User};
use serenity::async_trait;
use serenity::futures::future::join_all;
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
        if msg.content.to_lowercase().starts_with("coin")
            || msg.content.to_lowercase().starts_with("get")
        {
            let user_object = BotUser::from_user(&msg.author, &ctx.http, msg.guild_id).await;
            if let Some(message) = match msg
                .content
                .to_lowercase()
                .split(" ")
                .collect::<Vec<&str>>()
                .get(1)
            {
                Some(&"create") => self.send_command(Command::CreateCoin).await,
                // get coin
                Some(&"get") | Some(&"coin") | None => {
                    self.send_command(Command::GetCoin(user_object)).await
                }
                // coin count
                Some(&"count") => match msg.mentions.is_empty() {
                    true => self.send_command(Command::CoinCount(user_object)).await,
                    false => {
                        self.send_command(Command::CoinCountMultiple(
                            // mentions
                            msg.mentions
                                .iter()
                                .map(|x| x.into())
                                .collect::<Vec<BotUser>>(),
                        ))
                        .await
                    }
                },
                // coin leaderboard
                Some(&"leaderboard") => {
                    self.send_command(Command::CoinLeaderboard(user_object))
                        .await
                }
                // give coin
                Some(&"give") => match msg.mentions.is_empty() {
                    true => Some("Please make sure you are giving someone a coin.".into()),
                    false => {
                        self.send_command(Command::GiveCoin(
                            msg.author.into(),
                            msg.mentions
                                .iter()
                                .map(|x| x.into())
                                .collect::<Vec<BotUser>>(),
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

async fn get_usernames<T>(users: Vec<User>, http: T, guild_id: Option<GuildId>) -> Vec<BotUser>
where
    T: CacheHttp,
{
    join_all(
        users
            .iter()
            .map(|x| async { BotUser::from_user(x, &http, guild_id).await }),
    )
    .await
}
