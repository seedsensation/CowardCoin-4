use crate::communication::*;
use crate::helpers::random_between;

use serenity::all::{ChannelId, CreateMessage, GuildId, MessageBuilder, Ready, User};
use serenity::futures::future::join_all;
use serenity::model::channel::Message;
use serenity::prelude::*;
use serenity::{Result, async_trait};
use std::cmp::Ordering;

use tokio::sync::mpsc::{Sender, channel};
use tokio::task;
use tokio::time::{self, Duration};

// debug
//const COIN_CHANNEL: u64 = 1368929242229903360;
// active
const COIN_CHANNEL: u64 = 813898229368094760;

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
            let user_object = BotUser::from_user(&msg.author, &ctx.http, msg.guild_id).await;
            let message_content = msg.content.to_lowercase();
            let msg_words = message_content.split(" ").collect::<Vec<&str>>();
            let second_word = msg_words.get(1).clone();
            if let Some(message) = match second_word {
                // get coin
                Some(&"get") | Some(&"coin") | None => {
                    self.send_command(Command::GetCoin(user_object)).await
                }
                // coin count
                Some(&"count") => match msg.mentions.is_empty() {
                    true => self.send_command(Command::CoinCount(user_object)).await,
                    false => {
                        let _ = ctx.http.broadcast_typing(msg.channel_id).await;
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
                    let _ = ctx.http.broadcast_typing(msg.channel_id).await;
                    self.send_command(Command::CoinLeaderboard(user_object))
                        .await
                }
                Some(&"give") => match msg.mentions.len().cmp(&1) {
                    // check how many people the give coin command mentions
                    // mentions less than one person
                    Ordering::Less => {
                        Some("Please make sure you are giving someone a coin.".into())
                    }
                    // mentions more than one person
                    Ordering::Greater => Some("You can only give coins to one person!".into()),
                    // mentions one person
                    Ordering::Equal => match msg_words.len().cmp(&4) {
                        // if the command has less than 4 words
                        Ordering::Less => Some("You're forgetting something...".into()),
                        // 'give' label so we can return the result of the command with
                        //  a break command
                        _ => 'give: {
                            let _ = ctx.http.broadcast_typing(msg.channel_id).await;
                            // go through each word
                            for word in msg_words {
                                // if it's a valid number
                                if let Ok(val) = str::parse::<f64>(word) {
                                    if val > i64::MAX as f64 {
                                        break 'give Some("That number's too large!".into());
                                    } else
                                    // if the number isn't negative
                                    if val >= 0.0 {
                                        // send it
                                        break 'give self
                                            .send_command(Command::GiveCoin(
                                                msg.author.into(),
                                                msg.mentions.get(0).unwrap().into(),
                                                val as i64,
                                            ))
                                            .await;
                                    } else {
                                        // if the number's negative
                                        break 'give Some("You can't give negative coins!".into());
                                    }
                                }
                            }
                            // if no number is found
                            break 'give Some("Did you include a number?".into());
                        }
                    },
                },
                Some(&"arena") => {
                    self.send_command(Command::Arena(
                        user_object,
                        msg_words
                            .iter()
                            .map(|x| (*x).into())
                            .collect::<Vec<String>>(),
                    ))
                    .await
                }

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
    let mut start_time = time::Instant::now();
    let mut coin_timer = time::Duration::from_secs(random_between(1, 600) as u64);

    loop {
        interval.tick().await;
        if (time::Instant::now() - start_time) > coin_timer {
            // runs every second
            if let Some(coin_message) =
                Handler::send_command_isolated(&sender, Command::CreateCoin).await
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
            start_time = time::Instant::now();
            coin_timer = time::Duration::from_mins(random_between(5, 60) as u64);
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
