use std::sync::Arc;

use serenity::all::Http;
use serenity::all::Message;

use crate::Coin;
use crate::communication::{BotUser, CoinMessage};
use crate::games::*;
use crate::helpers::s_if;
use crate::server::Server;

pub trait CoinCommands {
    fn get_coin(&mut self, user: BotUser) -> impl Future<Output = Option<String>>;
    fn coin_count(&mut self, users: Vec<BotUser>) -> String;
    fn give_coin(&mut self, sender: BotUser, recipient: BotUser, amount: i64) -> String;
    fn create_coin(&mut self) -> Option<String>;
    fn coin_leaderboard(&self, id: BotUser) -> String;
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
                    "{} has {} coin{}.",
                    user.nickname.clone().unwrap_or(x.display_name.clone()),
                    user.coins,
                    s_if(user.coins)
                )
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn give_coin(&mut self, sender: BotUser, recipient: BotUser, amount: i64) -> String {
        if sender.id == recipient.id {
            return self.trick(sender, amount);
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

    fn create_coin(&mut self) -> Option<String> {
        if !self.coin_message.is_none() || !self.coin.is_none() {
            return None;
        }

        self.coin = Coin::new();
        Some(self.coin.arrival_message())
    }

    fn coin_leaderboard(&self, id: BotUser) -> String {
        "Coin leaderboard hasn't been implemented yet, sorry...".into()
    }

    async fn set_coin_message(&mut self, message: Message, http: Arc<Http>) {
        self.coin_message = Some(CoinMessage {
            msg: message,
            http: http,
        });
    }
}

//match request.command {
//           // get coin - not implemented
//           Command::GetCoin(user) => server.get_coin(user).await,
//
//           // coin count
//           Command::CoinCount(user) => Some(server.coin_count(vec![user]).await),
//           Command::CoinCountMultiple(users) => Some(server.coin_count(users).await),
//
//           // coin leaderboard
//           Command::CoinLeaderboard(id) => Some(server.coin_leaderboard(id)),
//
//           // give coin
//           Command::GiveCoin(sender, recipient, amount) => {
//               Some(server.give_coin(sender, recipient, amount))
//           }
//
//           Command::CreateCoin => server.create_coin(),
//
//           Command::CreateCoinCheck => {
//               if server.coin_message.is_none() {
//                   server.create_coin()
//               } else {
//                   None
//               }
//           }
//           Command::CoinCreateNotification(msg, http) => {
//               server.coin_message = Some(CoinMessage {
//                   msg: msg,
//                   http: http,
//               });
//               None
//           }
//           Command::DeleteCoinMessage => {
//               if let Some(mut message) = server.coin_message {
//                   let _ = message.delete().await;
//               };
//               server.coin_message = None;
//               None
//           }
//           Command::Arena(sender, msg_words) => {
//               Some(server.coin_arena(sender, msg_words))
//           }
//       }
