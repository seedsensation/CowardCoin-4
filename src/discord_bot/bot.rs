use super::BotUser;
use super::bot_commands::read_message;
use super::coin_creation::coin_creation_check;

use crate::communication::{Command, Request};

use serenity::{
    all::{Message, Ready},
    async_trait,
    prelude::*,
};
use tokio::{
    sync::mpsc::{Sender, channel},
    task,
    time::Duration,
};

pub struct Handler {
    pub sender: Sender<Request>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _: Ready) {
        task::spawn(coin_creation_check(
            Duration::from_secs(1),
            self.sender.clone(),
            ctx.clone(),
        ));
    }
    async fn message(&self, ctx: Context, msg: Message) {
        let message_content = msg.content.to_lowercase();
        let message_words = message_content.split(" ").collect::<Vec<&str>>();
        let bot_user = BotUser::from_user(&msg.author, &ctx.http, msg.guild_id).await;
        if let Some(message) = self
            .send_command(
                match read_message(message_words, msg.mentions, &self.sender, bot_user) {
                    Command::NoCommand => return,
                    command @ _ => command,
                },
            )
            .await
        {
            if let Err(why) = msg.channel_id.say(&ctx.http, message).await {
                println!("Error sending message: {why:?}");
            }
        } else {
            println!("oh :(");
        }
    }
}

impl Handler {
    pub async fn send_command_isolated(
        sender: &Sender<Request>,
        command: Command,
    ) -> Option<String> {
        let (tx, mut rx) = channel::<Option<String>>(100);
        if let Err(e) = sender
            .send(Request {
                command: command,
                reply_to: tx,
            })
            .await
        {
            panic!("CoinServer error: {e:?}");
        };
        if let Some(message) = rx.recv().await {
            return message;
        } else {
            return Some("There was an error communicating with the server.".into());
        }
    }
    async fn send_command(&self, command: Command) -> Option<String> {
        Self::send_command_isolated(&self.sender, command).await
    }
}
