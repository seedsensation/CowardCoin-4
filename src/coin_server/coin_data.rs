use crate::choose_message;
use crate::helpers::*;
use serde::{Deserialize, Serialize};
use std::ops::{Add, Sub};

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
            rarity: Rarity::None,
            value: 0,
        }
    }

    pub fn is_none(&self) -> bool {
        self.rarity == Rarity::None
    }

    pub fn arrival_message(&self) -> String {
        use Rarity::*;
        format!(
            "{} | {} {}{}{}",
            self.rarity.emoji(),
            // coin arrival prefix
            //  e.g. "A"
            match self.rarity {
                Uncommon => "An",
                Gnome => "A bell tolls in the distance.\nThe",
                Rare | Legendary | Mythical => {
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
                Gnome => "makes his entrance...",
                _ => "coin appeared!",
            },
            // coin count prefix
            //  e.g. "it's worth"
            match self.rarity {
                Common => String::new(),
                _ => format!(
                    "\n{} | {} {} {}",
                    self.rarity.emoji(),
                    match self.rarity {
                        Gnome => "He's worth",
                        _ => "It's worth",
                    },
                    // coin value
                    match self.rarity {
                        Gnome => String::from("probably infinite"),
                        _ => self.value.to_string(),
                    },
                    // coin count suffix
                    // e.g. "coins!"
                    match self.rarity {
                        Gnome => String::from("coins lol"),
                        _ => format!("coin{}!", s_if(self.value)),
                    }
                ),
            }
        )
    }

    pub fn escape_message(&self) -> String {
        use Rarity::*;
        format!(
            "{} The {} {}{}{}{}",
            choose_message!(
                "🏃‍♂️",
                "🏃‍♂️‍➡️",
                "🏃",
                "🏃‍➡️",
                "🏃‍♀️",
                "🏃‍♀️‍➡️",
                "<a:baaulpSpin:1059203066018156675>",
                "<a:cainedansandpemaneleLOL:751118345335603264>",
                "<a:violence:726858564509106216>",
            ),
            self.rarity.emoji(),
            self.rarity.name(),
            match self.rarity {
                Gnome => "",
                _ => "coin ",
            },
            match self.rarity {
                Common => "".to_string(),
                _ => format!(
                    "(worth {} coin{}) ",
                    match self.rarity {
                        Gnome => "probably infinite".to_string(),
                        _ => self.value.to_string(),
                    },
                    s_if(self.value)
                ),
            },
            choose_message!(
                "ran away...",
                "escaped!",
                "made a dramatic getaway.",
                "climbs in a car and drives away!",
                "saw weakness in you, and leaves.",
                "catches on fire, and dies!"
            )
        )
    }
}

#[derive(Hash, Eq, PartialEq, PartialOrd, Clone, Copy, Serialize, Deserialize, Debug, Default)]
pub enum Rarity {
    #[default]
    None = 0,
    Common = 1,
    Uncommon = 2,
    Rare = 3,
    Legendary = 4,
    Mythical = 5,
    Gnome = 6,
}

impl Rarity {
    /// Get the coin's ID
    pub fn id(&self) -> i64 {
        use Rarity::*;
        match self {
            None => 0,
            Common => 1,
            Uncommon => 2,
            Rare => 3,
            Legendary => 4,
            Mythical => 5,
            Gnome => 6,
        }
    }

    /// Get the coin's name
    pub fn name(&self) -> &str {
        use Rarity::*;
        match self {
            None => "none ",
            Common => "",
            Uncommon => "uncommon ",
            Rare => "_rare_ ",
            Legendary => "**legendary** ",
            Mythical => "***mythical*** ",
            Gnome => "gnome ",
        }
    }

    pub fn a_name(&self) -> String {
        use Rarity::*;
        format!(
            "{}{}",
            match self {
                None => "",
                Uncommon => "an ",
                _ => "a ",
            },
            self.name()
        )
    }

    pub fn emoji(&self) -> &str {
        use Rarity::*;
        match self {
            Common => "<a:bronzecoin:844545666201288755>",
            Uncommon => "<a:silvercoin:844545665911881788>",
            Rare => "<a:gold:1038495846074941440>",
            Legendary => "<a:redcoin:844545670709772290>",
            Mythical => "<a:white:1136340312529318018>",
            Gnome => "<:gnomeZoom:1079734801831047248>",
            _ => "<:mastergirBubby:757618593725808710>",
        }
    }

    /// Get a coin's rarity from its ID
    pub fn from_id(id: i64) -> Rarity {
        use Rarity::*;
        match id {
            1 => Common,
            2 => Uncommon,
            3 => Rare,
            4 => Legendary,
            5 => Mythical,
            6 => Gnome,
            _ => None,
        }
    }

    /// Get $10^{\text{item ID}}$
    pub fn get_exponent(&self) -> i64 {
        if self.id() == 0 {
            return 0;
        }
        10i64.pow(self.id() as u32)
    }

    /// Get the highest value rarity
    pub fn max_rarity() -> Rarity {
        Rarity::Gnome
    }

    /// Decide a random rarity
    pub fn generate() -> Rarity {
        let maximum: i64 = Rarity::max_rarity().get_exponent();
        let minimum = 1i64;

        let x = random_between(minimum, maximum);
        Rarity::from_id((6u32 - x.ilog10()) as i64)
    }

    pub fn calculate_value(&self) -> i64 {
        use Rarity::*;
        match self {
            Common => 1,
            Gnome => -50,
            _ => random_between(self.get_exponent() / 20, (*self + 1).get_exponent() / 20) / 2,
        }
    }
}

impl Add<i64> for Rarity {
    type Output = Self;

    fn add(self, rhs: i64) -> Self {
        Self::from_id(self.id() + rhs)
    }
}

impl Add<Rarity> for Rarity {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        self + rhs.id()
    }
}

impl Sub for Rarity {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::from_id(self.id() - rhs.id())
    }
}
