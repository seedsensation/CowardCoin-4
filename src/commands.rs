use std::sync::Arc;
use std::time::SystemTime;

use serenity::all::Http;
use serenity::all::Message;

use crate::communication::CoinMessage;
use crate::environment;
use crate::games::*;
use crate::prelude::*;
use crate::server::Server;

pub trait CoinCommands {
    fn get_coin(&mut self, user: BotUser) -> impl Future<Output = Option<String>>;
    fn coin_count(&mut self, users: Vec<BotUser>) -> String;
    fn give_coin(&mut self, sender: BotUser, recipient: BotUser, amount: i64) -> String;
    fn create_coin(&mut self) -> Option<String>;
    fn coin_leaderboard(&mut self, id: BotUser) -> String;
    fn set_coin_message(&mut self, message: Message, http: Arc<Http>) -> impl Future<Output = ()>;
    fn update_coins(&mut self);
}

impl CoinCommands for Server {
    async fn get_coin(&mut self, user: BotUser) -> Option<String> {
        if self.coin.is_none() {
            return None;
        }

        self.get_mut_user_from_id(&user).coins += self.coin.value;
        self.clear_coin();

        if let Some(msg) = self.coin_message.as_mut() {
            if let Err(why) = msg.delete().await {
                println!("Error deleting coin message: {why:?}");
            }
        }

        self.coin_message = None;
        if let Err(_) = self.save() {
            return Some("There was an error saving to file.".to_string());
        }

        let user = self.get_user_from_id(&user);

        Some(format!(
            "You got a coin!\nYou now have {} coin{}.",
            user.coins,
            s_if(user.coins)
        ))
    }

    /// Output the number of coins that a vector of users has.
    ///
    /// Each user will be outputted into a separate line.
    fn coin_count(&mut self, users: Vec<BotUser>) -> String {
        self.users.sort();
        users
            .iter()
            .map(|x| {
                let user = self.get_user_from_id(&x);
                format!(
                    "{}**{}** {}\n> - {} coin{}{}{}",
                    if user.xp > 0 || user.level > 1 {
                        format!("{} ", user.arena_title())
                    } else {
                        String::new()
                    },
                    user.nickname.clone().unwrap_or(x.display_name.clone()),
                    if user.xp > 0 || user.level > 1 {
                        format!("(Lv. {})", user.level)
                    } else {
                        String::new()
                    },
                    user.coins,
                    s_if(user.coins),
                    if user.style_points > 0 {
                        format!(
                            "\n> - {} StylePoint{}™",
                            user.style_points,
                            s_if(user.style_points)
                        )
                    } else {
                        String::new()
                    },
                    if user.xp > 0 || user.level > 1 {
                        format!("\n> - [{}] - {}/{}", user.xp_bar(), user.xp, user.xp_cap())
                    } else {
                        String::new()
                    },
                )
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn give_coin(&mut self, sender: BotUser, recipient: BotUser, amount: i64) -> String {
        if sender.id == recipient.id {
            let output = self.trick(sender, amount);
            if let Err(_) = self.save() {
                return "There was an error saving to file.".to_string();
            }
            return output;
        }

        self.sort_by_ids();

        // SAFETY: Do not sort `server.users` while `sender_local` or `recipient_local` are in scope.
        let ptr = self.users.as_mut_ptr();
        let sender_local = unsafe { &mut *ptr.add(crate::get_index_from_id!(sender in self)) };
        let recipient_local =
            unsafe { &mut *ptr.add(crate::get_index_from_id!(recipient in self)) };

        assert!(
            sender_local.id != recipient_local.id,
            "Cannot borrow the same element twice."
        );

        if sender_local.coins < amount {
            eprintln!(
                "has {} coins, wants to give {} coins",
                sender_local.coins, amount
            );
            return "You don't have enough coins!".into();
        }

        sender_local.coins -= amount;
        recipient_local.coins += amount;

        let message = if recipient_local.id == environment::BOT_ID {
            if SystemTime::now()
                .duration_since(sender_local.time_of_last_investment)
                .unwrap()
                < crate::environment::INVESTMENT_TIMER
            {
                format!(
                    "The stock market's still shifting... You can't make any more investments for another {}.",
                    crate::helpers::seconds_to_string(
                        if SystemTime::now()
                            .duration_since(sender_local.time_of_last_investment)
                            .unwrap()
                            < crate::environment::INVESTMENT_TIMER
                        {
                            (crate::environment::INVESTMENT_TIMER
                                - SystemTime::now()
                                    .duration_since(sender_local.time_of_last_investment)
                                    .unwrap())
                            .as_secs() as i64
                        } else {
                            0i64
                        }
                    )
                )
            } else {
                sender_local.time_of_last_investment = SystemTime::now();
                let chance = crate::helpers::random_between(0, 100);
                if chance > 90 {
                    let diff = recipient_local.coins / 2;
                    sender_local.coins += diff;
                    recipient_local.coins -= diff;
                    format!(
                        "Congratulations! Your investments have paid off! You have received {} coin{} in dividends.\nYou now have {} coin{}.\n{} coin{} remain in the CowardCoin Bank.",
                        diff,
                        s_if(diff),
                        sender_local.coins,
                        s_if(sender_local.coins),
                        recipient_local.coins,
                        s_if(recipient_local.coins)
                    )
                } else {
                    format!(
                        "You have invested {} coin{} in the CowardCoin Bank!\nYou now have {} coin{}.\nThere are now {} coin{} in the CowardCoin Bank.\nThe market will shift in {}.",
                        amount,
                        s_if(amount),
                        sender_local.coins,
                        s_if(sender_local.coins),
                        recipient_local.coins,
                        s_if(recipient_local.coins),
                        crate::helpers::seconds_to_string(
                            if SystemTime::now()
                                .duration_since(self.time_of_last_interest)
                                .unwrap()
                                < crate::environment::MARKET_CHANGE_TIMER
                            {
                                (crate::environment::MARKET_CHANGE_TIMER
                                    - SystemTime::now()
                                        .duration_since(self.time_of_last_interest)
                                        .unwrap())
                                .as_secs() as i64
                            } else {
                                0i64
                            }
                        )
                    )
                }
            }
        } else {
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
                sender_local.coins,
                s_if(sender_local.coins),
                recipient_nickname,
                recipient_local.coins,
                s_if(recipient_local.coins)
            )
        };

        if let Err(_) = self.save() {
            "There was an error saving to file.".to_string()
        } else {
            message
        }
    }

    fn create_coin(&mut self) -> Option<String> {
        if !self.coin_message.is_none() || !self.coin.is_none() {
            return None;
        }

        self.coin = Coin::new();
        Some(self.coin.arrival_message())
    }

    fn coin_leaderboard(&mut self, id: BotUser) -> String {
        // get list of every member, sorted by coin count
        let mut sorted_users = self.users.clone();
        sorted_users.sort_by(|x, y| y.coins.cmp(&x.coins));
        let user = self.get_user_from_id(&id);

        (0..(usize::min(10, sorted_users.len())))
            .into_iter()
            .map(|n| {
                let this_user = sorted_users.get(n as usize).expect(
                    format!(
                        "Fewer members of leaderboard than expected - {}, {}",
                        n,
                        sorted_users.len()
                    )
                    .as_str(),
                );
                let bold = if this_user.id == user.id { "**" } else { "" };
                format!(
                    "{}. {}{}{} - {} coin{}\n",
                    n + 1,
                    bold,
                    this_user
                        .nickname
                        .clone()
                        .unwrap_or(this_user.display_name.clone()),
                    bold,
                    this_user.coins,
                    s_if(this_user.coins)
                )
            })
            .collect::<Vec<String>>()
            .join("")
    }

    async fn set_coin_message(&mut self, message: Message, http: Arc<Http>) {
        self.coin_message = Some(CoinMessage {
            msg: message,
            http: http,
        });
    }

    fn update_coins(&mut self) {
        if SystemTime::now()
            .duration_since(self.time_of_last_interest)
            .unwrap()
            > crate::environment::MARKET_CHANGE_TIMER
        {
            self.time_of_last_interest = SystemTime::now();
            match self
                .users
                .binary_search_by_key(&crate::environment::BOT_ID, |x| x.id)
            {
                Ok(v) => {
                    let user = self.users.get_mut(v).unwrap();
                    let coins_change = (user.coins as f64
                        * (crate::helpers::random_between(850, 1150) as f64 / 1000f64))
                        as i64;
                    eprintln!("Updating investments by {}", coins_change - user.coins);
                    user.coins = coins_change;
                }
                Err(_) => eprintln!("Failed to find self in server log"),
            };
        }
    }
}
