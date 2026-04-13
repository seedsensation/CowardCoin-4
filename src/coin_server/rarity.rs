use std::ops::{Add, Sub};

use crate::helpers::*;

#[derive(Hash, Eq, PartialEq, PartialOrd, Clone, Copy)]
pub enum Rarity {
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
    pub fn id(&self) -> i32 {
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
            NONE => "none",
            COMMON => "common",
            UNCOMMON => "uncommon",
            RARE => "_rare_",
            LEGENDARY => "**legendary**",
            MYTHICAL => "***mythical***",
            GNOME => "gnome",
        }
    }

    /// Get a coin's rarity from its ID
    pub fn from_id(id: i32) -> Rarity {
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
    pub fn get_exponent(&self) -> i32 {
        if self.id() == 0 {
            return 0;
        }
        10i32.pow(self.id() as u32)
    }

    /// Get the highest ID
    fn max_id() -> i32 {
        Rarity::max_rarity().id()
    }

    /// Get the highest value rarity
    pub fn max_rarity() -> Rarity {
        Rarity::GNOME
    }

    /// Decide a random rarity
    pub fn generate() -> Rarity {
        let maximum: i32 = Rarity::max_rarity().get_exponent();
        let minimum = 1i32;

        let x = random_between(minimum, maximum);
        Rarity::from_id((6u32 - x.ilog10()) as i32)
    }

    pub fn calculate_value(&self) -> i32 {
        use Rarity::*;
        match self {
            COMMON => 1,
            GNOME => -50,
            _ => random_between(self.get_exponent() / 20, (*self + 1).get_exponent() / 20),
        }
    }
}

impl Add<i32> for Rarity {
    type Output = Self;

    fn add(self, rhs: i32) -> Self {
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
