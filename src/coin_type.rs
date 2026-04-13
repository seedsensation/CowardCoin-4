use rand::prelude::IndexedRandom;
use std::fmt;
use std::ops::Range;

#[derive(Clone)]
pub enum CoinType {
    NONE,
    COMMON,
    UNCOMMON,
    RARE,
    LEGENDARY,
    MYTHICAL,
    GNOME,
}

impl CoinType {
    /// The basic integer value of the coin.
    pub fn value(&self) -> i32 {
        match *self {
            CoinType::NONE => 0,
            CoinType::COMMON => 1,
            CoinType::UNCOMMON => 2,
            CoinType::RARE => 3,
            CoinType::LEGENDARY => 4,
            CoinType::MYTHICAL => 5,
            CoinType::GNOME => 6,
        }
    }

    /// Returns a coin value based on the integer submitted
    fn from_value(val: i32) -> CoinType {
        match val {
            1 => CoinType::COMMON,
            2 => CoinType::UNCOMMON,
            3 => CoinType::RARE,
            4 => CoinType::LEGENDARY,
            5 => CoinType::MYTHICAL,
            6 => CoinType::GNOME,
            _ => CoinType::NONE,
        }
    }

    /// The maximum value of the enum
    fn max() -> i32 {
        CoinType::GNOME.value()
    }

    /// 10 raised to the power of `value()`.
    fn power_zeros(&self) -> i32 {
        10i32.pow(self.value() as u32)
    }

    /// The range with which a random value can be calculated
    pub fn value_range(&self) -> Range<i32> {
        match *self {
            CoinType::COMMON => 1..2,
            _ => {
                std::cmp::max((10i32.pow(self.value() as u32 - 1) as f32 / 2f32) as i32, 5)
                    ..((self.power_zeros() / 10) * 3)
            }
        }
    }

    /// Generate a random value using `value_range`.
    pub fn generate_value(&self) -> i32 {
        let mut rng = rand::rng();
        *(self.clone())
            .value_range()
            .collect::<Vec<i32>>()
            .choose(&mut rng)
            .unwrap()
    }

    /// Randomly decides one coin out of the list available.
    ///
    /// A random number is decided between 0 and 10^(`Coin::max()`) -
    /// and the coin decided on is `Coin::max() - floor(log^10)` of that number.
    ///
    /// *For example:*
    ///
    /// The maximum is 6. A random number between 1 and 1000000 is decided on.
    ///
    /// It lands on 5382. floor(log^10(5382)) = 3. 6 - 3 = 3, so
    /// a RARE coin is selected.
    pub fn choose_coin() -> CoinType {
        let mut rng = rand::rng();
        CoinType::from_value(
            // subtract from max
            Self::max() -
            // 0 - 10^max()
            (*(0..10i32.pow(Self::max() as u32))
                // collect into vector
                .collect::<Vec<i32>>()
                // choose at random
                .choose(&mut rng)
                // confirm selection
                .unwrap())
            // log10
            .ilog10() as i32,
        )
    }

    pub fn coin_descriptor(&self) -> String {
        match *self {
            CoinType::NONE => "NO COIN !?!?!?!?!?!?",
            CoinType::COMMON => "a coin",
            CoinType::UNCOMMON => "an *Uncommon Coin*",
            CoinType::RARE => "a **Rare Coin**",
            CoinType::LEGENDARY => "a ***Legendary Coin***",
            CoinType::MYTHICAL => "a ***MYTHICAL COIN***",
            CoinType::GNOME => "the GNOME from GAMES",
        }
        .to_string()
    }
}

impl fmt::Display for CoinType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                CoinType::NONE => "no coin",
                CoinType::COMMON => "coin",
                CoinType::UNCOMMON => "uncommon coin",
                CoinType::RARE => "rare coin",
                CoinType::LEGENDARY => "legendary coin",
                CoinType::MYTHICAL => "mythical coin",
                CoinType::GNOME => "gnome",
            }
        )
    }
}
