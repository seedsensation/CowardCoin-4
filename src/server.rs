use std::fs;

use tokio::sync::mpsc::Receiver;

use crate::Coin;
use crate::Rarity;
use crate::commands::CoinCommands;
use crate::communication::BotUser;
use crate::communication::{CoinMessage, Command, Request};
use crate::games::Games;
use crate::helpers::random_between;
use crate::helpers::s_if;
use crate::user::CoinUser;

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    pub users: Vec<CoinUser>,
    #[serde(skip)]
    pub coin: Coin,
    #[serde(skip)]
    pub coin_message: Option<CoinMessage>,
}

pub trait ExecuteCommands: CoinCommands {
    async fn execute_command(&mut self, command: Command) -> Option<String>;
}

impl<T> ExecuteCommands for T
where
    T: CoinCommands + Games,
{
    async fn execute_command(&mut self, command: Command) -> Option<String> {
        use Command::*;
        match command {
            GetCoin(bot_user) => self.get_coin(bot_user).await,
            CoinCount(bot_user) => Some(self.coin_count(vec![bot_user])),
            CoinCountMultiple(bot_users) => Some(self.coin_count(bot_users)),
            CoinLeaderboard(bot_user) => Some(self.coin_leaderboard(bot_user)),
            GiveCoin(sender, recipient, amount) => Some(self.give_coin(sender, recipient, amount)),
            CreateCoin => self.create_coin(),
            CoinCreateNotification(message, http) => {
                self.set_coin_message(message, http);
                None
            }
            Arena(bot_user, items) => Some(self.arena(bot_user, items)),
        }
    }
}

impl Server {
    pub fn new() -> Self {
        serde_json::from_reader(
            fs::OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open("./data.json")
                .unwrap(),
        )
        .unwrap_or_else(|_| Self {
            users: vec![],
            coin: Coin::none(),
            coin_message: None,
        })
    }

    pub fn save(&mut self) -> Result<()> {
        println!("Saving file...");
        self.users.sort();
        serde_json::to_writer_pretty(
            {
                fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open("./data.json")
                    .unwrap_or_else(|why| panic!("Error opening file: {why:?}"))
            },
            self,
        )
    }

    pub async fn start(mut receiver: Receiver<Request>) {
        let mut server = Self::new();
        let _ = server.save();
        loop {
            println!("Server running!");
            if let Some(request) = receiver.recv().await {
                if let Err(why) = request
                    .reply_to
                    .send(server.execute_command(request.command).await)
                    .await
                {
                    println!("Error sending message: {why:?}");
                }
            }
            server.save().ok();
        }
    }
    async fn coin_count(&mut self, users: Vec<BotUser>) -> String {
        self.users.sort();
        let mut output: String = "".into();
        for temp_user in users {
            let user = self.get_user_from_id(&temp_user);
            output.push_str(&format!(
                "{} has {} coin{}.\n",
                user.nickname.clone().unwrap_or(temp_user.display_name),
                user.coins,
                s_if(user.coins)
            ));
        }
        output
    }

    pub fn get_user_from_id(&mut self, user: &BotUser) -> &CoinUser {
        self.users.sort();
        if let Ok(v) = self.users.binary_search_by_key(&user.id, |x| x.id) {
            {
                let new_user = self.users.get_mut(v).unwrap();
                if user.nickname != new_user.nickname && user.nickname.is_some() {
                    new_user.nickname = user.nickname.clone();
                }
            }
            self.users.get(v).unwrap()
        } else {
            println!("Creating new item for user {}", user.id);
            self.users
                .push(CoinUser::new(user.id, user.nickname.clone()));
            self.users.last().unwrap()
        }
    }
    pub fn get_mut_user_from_id(&mut self, user: &BotUser) -> &mut CoinUser {
        self.users.sort();
        if let Ok(v) = self.users.binary_search_by_key(&user.id, |x| x.id) {
            let new_user = self.users.get_mut(v).unwrap();
            if user.nickname != new_user.nickname && user.nickname.is_some() {
                new_user.nickname = user.nickname.clone();
            }
            new_user
        } else {
            self.users
                .push(CoinUser::new(user.id, user.nickname.clone()));
            let output = self.users.last_mut().unwrap();
            output
        }
    }

    async fn get_coin(&mut self, user: BotUser) -> Option<String> {
        if !self.coin.is_none() {
            {
                self.get_mut_user_from_id(&user).coins += self.coin.value;
                self.clear_coin();
            }

            if let Some(msg) = self.coin_message.as_mut() {
                if let Err(why) = msg.delete().await {
                    println!("Error deleting coin message: {why:?}");
                };
            }
            self.coin_message = None;

            let user = self.get_user_from_id(&user);
            Some(format!(
                "You got a coin!\nYou now have {} coin{}.",
                user.coins,
                s_if(user.coins)
            ))
        } else {
            None
        }
    }

    fn create_coin(&mut self) -> Option<String> {
        if !self.coin.is_none() {
            Some("A coin already exists!".into())
        } else {
            self.coin = Coin::new();
            Some(self.coin.arrival_message())
        }
    }
    pub fn clear_coin(&mut self) {
        self.coin = Coin::none();
    }

    fn coin_trick(&mut self, sender: BotUser, amount: i64) -> String {
        let sender_local = self.get_mut_user_from_id(&sender);
        if sender_local.coins >= amount
            || sender_local.level >= required_level_for_trick(amount, sender_local.coins)
        {
            let num = random_between(1, 100);
            if num > 90 {
                // trick crit, double coins, get xp
            } else if num <= 25 {
                // trick fails, lose all coins put in
            } else {
                // trick normal, get (coins put in / 10) xp
            }
            "done".into()
        } else {
            return format!(
                "That trick is _too powerful_ for your current level. You should try training in the **Coin Arena™** if you want to become stronger - to do this trick, {}.",
                if sender_local.coins > 0 {
                    format!(
                        "you'll need to be at least level {}",
                        required_level_for_trick(amount, sender_local.coins)
                    )
                } else {
                    "you'll need to stock up on coins first".into()
                }
            );
        }
    }

    fn give_coin(&mut self, sender: BotUser, recipient: BotUser, amount: i64) -> String {
        if sender.id == recipient.id {
            return self.coin_trick(sender, amount);
        }
        let sender_coins = {
            let sender_local = self.get_mut_user_from_id(&sender);
            if sender_local.coins < amount {
                return "You don't have enough coins!".into();
            } else {
                sender_local.coins -= amount;
            }
            sender_local.coins.clone()
        };

        let recipient_local = self.get_mut_user_from_id(&recipient);

        recipient_local.coins += amount;

        let recipient_nickname = recipient_local
            .nickname
            .as_ref()
            .unwrap_or(&recipient.display_name)
            .clone();
        format!(
            "You gave {} coin{} to {}!\nYou now have {} coin{}.\n{} now has {} coin{}.",
            amount,
            s_if(amount),
            recipient_nickname,
            sender_coins,
            s_if(sender_coins),
            recipient_nickname,
            recipient_local.coins,
            s_if(recipient_local.coins),
        )
    }

    fn coin_arena(&mut self, sender: BotUser, msg_words: Vec<String>) -> String {
        let sender_local = self.get_mut_user_from_id(&sender);
        // coin arena intro
        if msg_words.len() == 2 {
            // coins * (1.1 * (level -1)) = amount
            let sender_coin_potential = std::cmp::max(
                (sender_local.coins as f64 * (1.1 * (sender_local.level as f64 - 1.0))) as i64,
                sender_local.coins,
            );
            format!(
                "
Welcome to the **COIN ARENA**!

{} {} - Lv. {} - {} Coin{}
[{}] - {}/100
Give {} more coin{} to reach the next level.

You can currently do a trick worth {} coin{}.
	",
                match sender_local.level {
                    0..=4 => "Harmless",
                    5..=9 => "Mostly Harmless",
                    10..=14 => "Gambling Addict(?)",
                    15..=19 => "Gambling Addict(!)",
                    20..=24 => "Biggest Clown at the Circus",
                    25..=49 => "Lost All Their Money On The Horse Races",
                    50 => "Top Of The Bell Curve",
                    51..=74 => "Please Stop",
                    75..=79 => "I Will Pay You One United States Dollar To Stop Gambling",
                    80..=89 => "I Will Pay You Two United States Dollars To Stop Gambling",
                    90..=94 => "Okay Deal's Off I Ran Out Of Money",
                    95..=97 => "You Can Stop At 100",
                    98 => "Please Stop At 100",
                    99 => "One More Level",
                    100 => "SUPREME COIN CHAMPION FOREVER",
                    101 => "for fucks sake",
                    102 => "okay fuck it",
                    103 => "this next one's the last one",
                    104..=149 => "BEARER OF THE GAMBLING CROWN OF SHAME",
                    150..=199 => "*BEARER OF THE GAMBLING CROWN OF SHAME*",
                    200..=249 => "**BEARER OF THE GAMBLING CROWN OF SHAME**",
                    250..=499 =>
                        "**B E A R E R  O F  T H E  G A M B L I N G  C R O W N  O F  S H A M E**",
                    500..=999 => "Gambled Your Crown Away",
                    1000 => "won it back?",
                    1001 => "lost it again",
                    1002..=4999 => "Gambled Your Crown Away",
                    5000..=9999 =>
                        "I'll give you another supreme coin champion trophy at level 10,000. Will you stop then?",
                    10000 =>
                        "I WAS LYING. WELCOME TO THE COIN ASCENDANCY, FUCKER. HOW HIGH WILL YOU GO?",
                    10001..=99999 => "Coin Ascendancy Hopeful",
                    100000..=999999 => "Coin Ascendancy Initiate",
                    1000000..=9999999 => "Member of the Coin Ascendancy",
                    10000000..=99999999 => "Coin Ascendancy Nobility",
                    100000000..=999999999 => "Coin Ascendancy Royalty",
                    1000000000..=9999999999 => "Supreme Ruler of the Coin Ascendancy",
                    10000000000..=99999999999 => "Deity of the Coin Ascendancy",
                    100000000000 => "Betrayed by the Coin Ascendancy",
                    _ => "Buried in a Hole",
                },
                sender_local.nickname.clone().unwrap_or(sender.display_name),
                sender_local.level,
                sender_local.coins,
                s_if(sender_local.coins),
                // progress bar
                {
                    let bar1 = "▓".repeat((sender_local.xp / 10) as usize);
                    let bar2 = "░".repeat((10 - (sender_local.xp / 10)) as usize);
                    bar1 + &bar2
                },
                // currentxp
                sender_local.xp,
                // 100 - current xp
                100 - sender_local.xp,
                // s_if 100 - current xp
                s_if(100 - sender_local.xp),
                // amount * (1.1 * (level - 1))
                sender_coin_potential as i64,
                // s_if ^
                s_if(sender_coin_potential as i64)
            )
        } else {
            match msg_words.get(2).unwrap().as_str() {
                "train" => {
                    if msg_words.len() == 4 {
                        if let Ok(val) = str::parse::<i64>(msg_words.get(3).unwrap()) {
                            if sender_local.coins >= val {
                                sender_local.coins -= val;
                                let response = sender_local.add_xp_with_response(val);
                                format!(
                                    "You spend {} coin{} on rigorous training.{}",
                                    val,
                                    s_if(val),
                                    response
                                )
                            } else {
                                format!("You can't afford to train that hard!")
                            }
                        } else {
                            format!(
                                "Please format your command with a number as `coin arena train [amount to spend]`."
                            )
                        }
                    } else {
                        format!(
                            "Please format your command as `coin arena train [amount to spend]`."
                        )
                    }
                }
                _ => format!("bye"),
            }
        }
    }

    fn coin_leaderboard(&self, id: BotUser) -> String {
        "Leaderboard hasn't been implemented yet, sorry...".into()
    }
}

fn required_level_for_trick(amount: i64, count: i64) -> i64 {
    // max(coins * (1.1 * (level - 1))) = amount
    // level = (level / coins / 1.1) + 1
    f64::ceil((amount as f64 / count as f64 / 1.1 as f64) + 1.0) as i64
}
