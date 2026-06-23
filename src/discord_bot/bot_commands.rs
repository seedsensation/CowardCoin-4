use super::BotUser;
use crate::communication::Command;

pub fn read_message(
    words: Vec<&str>,
    mentions: Vec<serenity::all::User>,
    user: BotUser,
) -> Command {
    match words.first() {
        Some(word) if *word == "get" => return Command::GetCoin(user),
        Some(word) if *word == "coin" => (),
        _ => return Command::NoCommand,
    };

    if let Some(word) = words.get(1) {
        let word = *word;
        match word {
            "count" => match mentions.is_empty() {
                true => Command::CoinCount(user),
                false => Command::CoinCountMultiple(mentions.iter().map(|x| x.into()).collect()),
            },
            "leaderboard" => Command::CoinLeaderboard(user),
            "give" => {
                if mentions.is_empty() {
                    Command::Error("You haven't mentioned anyone to give coins to.")
                } else if mentions.len() > 1 {
                    Command::Error("You can only give coins to one person.")
                } else {
                    let coin_total = words
                        .iter()
                        .filter_map(|word| word.parse::<i64>().ok())
                        .sum();
                    if coin_total == 0 {
                        Command::Error("Please make sure you're giving at least one coin.")
                    } else if coin_total < 0 {
                        Command::Error("You can't give negative coins!")
                    } else {
                        Command::GiveCoin(user, mentions.first().unwrap().into(), coin_total)
                    }
                }
            }
            "invest" => {
                let coin_total = words
                    .iter()
                    .filter_map(|word| word.parse::<i64>().ok())
                    .sum();
                if coin_total == 0 {
                    Command::Error("Please make sure you're giving at least one coin.")
                } else if coin_total < 0 {
                    Command::Error("You can't give negative coins!")
                } else {
                    Command::Invest(user, coin_total)
                }
            }
            "arena" => Command::Arena(user, words.iter().map(|x| x.to_string()).collect()),
            "eat" => {
                let eaten_count = match words
                    .iter()
                    .filter_map(|word| word.parse::<i64>().ok())
                    .sum()
                {
                    0 => 1,
                    amount => amount,
                };
                Command::EatCoin(user, eaten_count)
            }
            command => {
                println!("Unrecognised command '{command}'");
                Command::GetCoin(user)
            }
        }
    } else {
        Command::GetCoin(user)
    }
}
