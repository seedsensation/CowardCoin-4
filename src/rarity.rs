use serde::{Deserialize, Serialize};
use std::ops::{Add, Sub};

use crate::helpers::*;

#[derive(Hash, Eq, PartialEq, PartialOrd, Clone, Copy, Serialize, Deserialize, Debug, Default)]
pub enum Rarity {
    #[default]
    NONE = 0,
    COMMON = 1,
    UNCOMMON = 2,
    RARE = 3,
    LEGENDARY = 4,
    MYTHICAL = 5,
    GNOME = 6,
}

impl Rarity {
    /// Get the coin's ID
    pub fn id(&self) -> i64 {
        use Rarity::*;
        match self {
            NONE => 0,
            COMMON => 1,
            UNCOMMON => 2,
            RARE => 3,
            LEGENDARY => 4,
            MYTHICAL => 5,
            GNOME => 6,
        }
    }

    /// Get the coin's name
    pub fn name(&self) -> &str {
        use Rarity::*;
        match self {
            NONE => "none ",
            COMMON => "",
            UNCOMMON => "uncommon ",
            RARE => "_rare_ ",
            LEGENDARY => "**legendary** ",
            MYTHICAL => "***mythical*** ",
            GNOME => "gnome ",
        }
    }
    pub fn emoji(&self) -> &str {
        use Rarity::*;
        match self {
            COMMON => "<a:bronzecoin:844545666201288755>",
            UNCOMMON => "<a:silvercoin:844545665911881788>",
            RARE => "<a:gold:1038495846074941440>",
            LEGENDARY => "<a:redcoin:844545670709772290>",
            MYTHICAL => "<a:white:1136340312529318018>",
            GNOME => "<:gnomeZoom:1079734801831047248>",
            _ => "<:mastergirBubby:757618593725808710>",
        }
    }

    /// Get a coin's rarity from its ID
    pub fn from_id(id: i64) -> Rarity {
        use Rarity::*;
        match id {
            1 => COMMON,
            2 => UNCOMMON,
            3 => RARE,
            4 => LEGENDARY,
            5 => MYTHICAL,
            6 => GNOME,
            _ => NONE,
        }
    }

    /// Get $10^{\text{item ID}}$
    pub fn get_exponent(&self) -> i64 {
        if self.id() == 0 {
            return 0;
        }
        10i64.pow(self.id() as u32)
    }

    /// Get the highest ID
    fn max_id() -> i64 {
        Rarity::max_rarity().id()
    }

    /// Get the highest value rarity
    pub fn max_rarity() -> Rarity {
        Rarity::GNOME
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
            COMMON => 1,
            GNOME => -50,
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
        Self::from_id(self.id() + rhs.id())
    }
}
