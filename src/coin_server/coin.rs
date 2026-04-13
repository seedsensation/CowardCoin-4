use super::Rarity;

/// Struct for handling coins
pub struct Coin {
    pub value: i32,
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

    pub fn arrival_message(&self) -> String {
        use Rarity::*;
        format!(
            "{} {} {}\n{} {} {}",
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
                _ => "It's worth",
            },
            // coin value
            match self.rarity {
                GNOME => String::from("probably infinite"),
                _ => self.value.to_string(),
            },
            // coin count suffix
            // e.g. "coins!"
            if self.rarity == GNOME {
                "coins lol"
            } else if self.value == 1 {
                "coin!"
            } else {
                "coins!"
            }
        )
    }
}
