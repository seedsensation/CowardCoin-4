use std::sync::Arc;

use serenity::all::Http;
use serenity::all::Message;

use crate::Coin;
use crate::communication::{BotUser, CoinMessage};
use crate::constants;
use crate::games::*;
use crate::helpers::s_if;
use crate::server::Server;

pub trait CoinCommands {
    fn get_coin(&mut self, user: BotUser) -> impl Future<Output = Option<String>>;
    fn coin_count(&mut self, users: Vec<BotUser>) -> String;
    fn give_coin(&mut self, sender: BotUser, recipient: BotUser, amount: i64) -> String;
    fn create_coin(&mut self) -> Option<String>;
    fn coin_leaderboard(&mut self, id: BotUser) -> String;
    fn set_coin_message(&mut self, message: Message, http: Arc<Http>) -> impl Future<Output = ()>;
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

        if recipient.id == constants::BOT_ID {
            let (sender_local, recipient_local) = self.get_mut_users_from_ids(&sender, &recipient);
        }

        // borrow sender mutably
        let sender_local = self.get_mut_user_from_id(&sender);
        if sender_local.coins < amount {
            return "You don't have enough coins!".into();
        }

        sender_local.coins -= amount;
        let sender_coins = sender_local.coins.clone();
        // sender_local is never referenced again, so it is dropped

        // borrow recipient mutably
        let recipient_local = self.get_mut_user_from_id(&recipient);
        recipient_local.coins += amount;
        let recipient_nickname = recipient_local
            .nickname
            .as_ref()
            .unwrap_or(&recipient.display_name)
            .clone();
        let recipient_coins = recipient_local.coins.clone();
        // recipient_local is never referenced again, so it is dropped

        if let Err(_) = self.save() {
            return "There was an error saving to file.".to_string();
        } else {
            if recipient.id == constants::BOT_ID {
                format!(
                    "You have invested {} coin{} in the CowardCoin Bank!\nYou now have {} coin{}.\nThere are now {} coin{} in the CowardCoin Bank.",
                    amount,
                    s_if(amount),
                    sender_coins,
                    s_if(sender_coins),
                    recipient_coins,
                    s_if(recipient_coins),
                )
            } else {
                format!(
                    "You gave {} coin{} to {}!\nYou now have {} coin{}.\n{} now has {} coin{}.",
                    amount,
                    s_if(amount),
                    recipient_nickname,
                    sender_coins,
                    s_if(sender_coins),
                    recipient_nickname,
                    recipient_coins,
                    s_if(recipient_coins)
                )
            }
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
}
