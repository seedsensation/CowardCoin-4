use crate::communication::*;

use serenity::all::{ChannelId, CreateMessage, GuildId, MessageBuilder, Ready, User};
use serenity::futures::future::join_all;
use serenity::model::channel::Message;
use serenity::prelude::*;
use serenity::{Result, async_trait};

use tokio::sync::mpsc::{Sender, channel};
use tokio::task;
use tokio::time::{self, Duration};

// debug
const COIN_CHANNEL: u64 = 1368929242229903360;
// active
//const COIN_CHANNEL: u64 = 813898229368094760;

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
        if msg.content.to_lowercase().starts_with("coin")
            || msg.content.to_lowercase().starts_with("get")
        {
            ctx.http.broadcast_typing(msg.channel_id);
            let user_object = BotUser::from_user(&msg.author, &ctx.http, msg.guild_id).await;
            if let Some(message) = match msg
                .content
                .to_lowercase()
                .split(" ")
                .collect::<Vec<&str>>()
                .get(1)
            {
                Some(&"create") => self.send_command(Command::CreateCoin).await,
                Some(&"delete") => self.send_command(Command::DeleteCoinMessage).await,
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

async fn coin_creation_check(
    period: Duration,
    sender: Sender<Request>,
    ctx: Context,
) -> Result<()> {
    let mut interval = time::interval(period);
    loop {
        interval.tick().await;
        // runs every second
        if let Some(coin_message) =
            Handler::send_command_isolated(&sender, Command::CreateCoinCheck).await
        {
            let message = Into::<ChannelId>::into(COIN_CHANNEL)
                .send_message(&ctx.http, CreateMessage::new().content(coin_message))
                .await?;
            // how do i get this message out there?
            // pass it through
            Handler::send_command_isolated(
                &sender,
                Command::CoinCreateNotification(message, ctx.http.clone()),
            )
            .await;
        }
    }
}
impl Handler {
    async fn send_command_isolated(sender: &Sender<Request>, command: Command) -> Option<String> {
        let (tx, mut rx) = channel::<Option<String>>(100);
        if let Err(e) = sender
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
    async fn send_command(&self, command: Command) -> Option<String> {
        Self::send_command_isolated(&self.sender, command).await
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
