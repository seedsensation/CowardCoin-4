use crate::Rarity;
use crate::helpers::s_if;
use serde::{Deserialize, Serialize};

/// Struct for handling coins
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Coin {
    pub value: i64,
    pub rarity: Rarity,
}

impl Coin {
    pub fn new() -> Coin {
        let rarity = &Rarity::generate();
        Coin {
            rarity: *rarity,
            value: rarity.calculate_value(),
        }
    }

    pub fn none() -> Coin {
        Coin {
            rarity: Rarity::NONE,
            value: 0,
        }
    }

    pub fn is_none(&self) -> bool {
        self.rarity == Rarity::NONE
    }

    pub fn arrival_message(&self) -> String {
        use Rarity::*;
        format!(
            "{} | {} {}{}\n{} {} {}",
            self.rarity.emoji(),
            // coin arrival prefix
            //  e.g. "A"
            match self.rarity {
                UNCOMMON => "An",
                GNOME => "A bell tolls in the distance.\nThe",
                RARE | LEGENDARY | MYTHICAL => {
                    "**WOW**! A"
                }
                _ => "A",
            },
            // coin type
            //  e.g. "common"
            self.rarity.name(),
            // coin arrival suffix
            //  e.g. "coin appeared!"
            match self.rarity {
                GNOME => "makes his entrance...",
                _ => "coin appeared!",
            },
            // coin count prefix
            //  e.g. "it's worth"
            match self.rarity {
                GNOME => "He's worth",
                COMMON => "",
                _ => "It's worth",
            },
            // coin value
            match self.rarity {
                GNOME => String::from("probably infinite"),
                COMMON => "".into(),
                _ => self.value.to_string(),
            },
            // coin count suffix
            // e.g. "coins!"
            match self.rarity {
                GNOME => String::from("coins lol"),
                COMMON => "".into(),
                _ => format!("coin{}!", s_if(self.value)),
            }
        )
    }
}
