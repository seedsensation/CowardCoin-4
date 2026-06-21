use std::time::SystemTime;

use super::CoinUser;
use super::Games;
use super::Server;
use super::{Coin, Rarity};
use crate::communication::{CoinMessage, Command};
use crate::helpers::s_if;

pub trait CoinCommands {
    fn clear_coin(&mut self) -> impl Future<Output = ()>;
    fn update_coins(&mut self);

    fn run_command(
        &mut self,
        command: crate::communication::Command,
    ) -> impl Future<Output = Option<String>>;
}

impl CoinCommands for Server {
    /// Run a command from the `Command` enum.
    async fn run_command(&mut self, command: Command) -> Option<String> {
        match command {
            Command::GetCoin(bot_user) => {
                if self.coin.is_none() {
                    None
                } else {
                    // will be initialised with user's coin count
                    let coin_rarity = self.coin.rarity;
                    let coin_value = self.coin.value;
                    let user: &mut CoinUser;
                    let coins: i64;

                    // miniature scope here of user, so we can temporarily borrow it mutably
                    {
                        // get user
                        user = self.get_mut_user_from_id(&bot_user);
                        // give user coins
                        user.coins += coin_value;
                        // save the coin count to a variable
                        coins = user.coins;

                        // end of scope, user is placed back in the vec
                    }

                    if let Err(why) = self.save() {
                        return Some(format!("Error saving to file: {why:?}"));
                    }

                    self.clear_coin().await;

                    Some(format!(
                        "{} | You got {}coin!{}\n{} | You now have {} coin{}.",
                        coin_rarity.emoji(),
                        coin_rarity.a_name(),
                        match self.coin.rarity {
                            Rarity::Common => String::new(),
                            _ => format!(
                                "\n{} | You gained {} coin{}!",
                                coin_rarity.emoji(),
                                coin_value,
                                s_if(coin_value)
                            ),
                        },
                        coin_rarity.emoji(),
                        coins,
                        s_if(coins)
                    ))
                }
            }
            Command::CoinCount(bot_user) => {
                Some(self.get_user_from_id(&bot_user).coin_count_message())
            }
            Command::CoinCountMultiple(bot_users) => Some(
                bot_users
                    .iter()
                    .map(|x| self.get_user_from_id(x).coin_count_message())
                    .collect::<Vec<String>>()
                    .join("\n"),
            ),
            Command::CoinLeaderboard(bot_user) => {
                let mut sorted_users = self.users.clone();
                sorted_users.sort_by_key(|x| x.coins);
                sorted_users.reverse();

                let user = self.get_user_from_id(&bot_user);

                let mut users_vec = (0..(usize::min(10, sorted_users.len())))
                    .map(|n| {
                        let this_user = sorted_users
                            .get(n)
                            .expect("Fewer members of leaderboard than expected");
                        let bold = if this_user.id == user.id { "**" } else { "" };
                        format!(
                            "{}. {}{}{} - {} coin{}",
                            n + 1,
                            bold,
                            if this_user.id == crate::environment::BOT_ID {
                                "_The CowardCoin™ Bank_".to_string()
                            } else {
                                this_user
                                    .nickname
                                    .as_ref()
                                    .unwrap_or(&this_user.display_name)
                                    .to_owned()
                            },
                            bold,
                            this_user.coins,
                            s_if(this_user.coins)
                        )
                    })
                    .collect::<Vec<String>>();

                // add leaderboard position to the end of the message
                users_vec.push(format!(
                    "> You are at position {} on the leaderboard.",
                    sorted_users
                        .iter()
                        .enumerate()
                        .find_map(|(index, this_user)| if this_user.id == user.id {
                            Some(index)
                        } else {
                            None
                        })
                        .expect("Failed to find user in sorted_users")
                        + 1
                ));
                Some(users_vec.join("\n"))
            }
            Command::GiveCoin(sender, recipient, amount) => {
                if sender.id == recipient.id {
                    Some(self.trick(sender, amount))
                } else if self.get_user_from_id(&sender).coins < amount {
                    Some("You can't afford to give that many coins!".into())
                } else if recipient.id == crate::environment::BOT_ID {
                    Some(self.invest(sender, amount))
                } else {
                    let sender_coins: i64;
                    let recipient_coins: i64;
                    let recipient_nickname: String;
                    // SAFETY: DO NOT sort self.users inside of this scope.
                    unsafe {
                        let (sender_local, recipient_local) =
                            self.get_two_mut_users(&sender, &recipient);
                        assert!(
                            sender_local.id != recipient_local.id,
                            "Cannot borrow the same element twice."
                        );

                        sender_local.coins -= amount;
                        recipient_local.coins += amount;

                        sender_coins = sender_local.coins;
                        recipient_coins = recipient_local.coins;

                        recipient_nickname = recipient_local
                            .nickname
                            .as_ref()
                            .unwrap_or(&recipient_local.display_name)
                            .to_owned();
                    };

                    Some(format!(
                        "You gave {} coin{} to {}!\nYou now have {} coin{}.\n{} now has {} coin{}.",
                        amount,
                        s_if(amount),
                        recipient_nickname,
                        sender_coins,
                        s_if(sender_coins),
                        recipient_nickname,
                        recipient_coins,
                        s_if(recipient_coins)
                    ))
                }
            }
            Command::CreateCoin => {
                if self.coin_message.is_some() || !self.coin.is_none() {
                    None
                } else {
                    self.coin = Coin::new();
                    Some(self.coin.arrival_message())
                }
            }
            Command::CoinCreateNotification(msg, http) => {
                self.coin_message = Some(CoinMessage { msg: *msg, http });
                None
            }
            Command::Arena(bot_user, items) => Some(self.arena(bot_user, items)),
            Command::UpdateCoins => {
                self.update_coins();
                None
            }
            Command::ClearCoin => {
                if let Some(msg) = self.coin_message.as_mut()
                    && let Err(why) = msg.delete().await
                {
                    eprintln!("Error deleting coin message: {why:?}");
                }
                None
            }
            Command::CoinEscape => {
                if let Some(message) = self.coin_message.as_mut()
                    && let Err(why) = message
                        .msg
                        .edit(
                            &message.http,
                            serenity::builder::EditMessage::new()
                                .content(self.coin.escape_message()),
                        )
                        .await
                {
                    eprintln!("Error editing coin message: {why:?}");
                }
                self.coin_message = None;
                self.coin = Coin::none();
                None
            }
            Command::NoCommand => unimplemented!(),
            Command::Error(error_msg) => Some(error_msg.to_string()),
        }
    }
    #[inline]
    async fn clear_coin(&mut self) {
        if let Some(msg) = self.coin_message.as_mut()
            && let Err(why) = msg.delete().await
        {
            println!("Error deleting coin message: {why:?}");
        }

        self.coin_message = None;
        self.coin = Coin::none();
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
