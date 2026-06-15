use crate::choose_message;
use crate::prelude::*;
use serde::{Deserialize, Serialize};
use serenity::{
    Result,
    all::{ChannelId, CreateMessage},
    prelude::*,
};
use std::ops::{Add, Sub};

use tokio::{
    sync::mpsc::Sender,
    time::{self, Duration},
};

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
            "{} | {} {}{}{}",
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
                COMMON => format!(""),
                _ => format!(
                    "\n{} | {} {} {}",
                    self.rarity.emoji(),
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
                    match self.rarity {
                        GNOME => String::from("coins lol"),
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
                GNOME => "",
                _ => "coin ",
            },
            match self.rarity {
                COMMON => "".to_string(),
                _ => format!(
                    "(worth {} coin{}) ",
                    match self.rarity {
                        GNOME => "probably infinite".to_string(),
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

    pub fn a_name(&self) -> String {
        use Rarity::*;
        format!(
            "{}{}",
            match self {
                NONE => "",
                UNCOMMON => "an ",
                _ => "a ",
            },
            self.name()
        )
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

pub async fn coin_creation_check(
    period: Duration,
    sender: Sender<Request>,
    ctx: Context,
) -> Result<()> {
    let mut interval = time::interval(period);
    let mut start_time = time::Instant::now();
    let mut coin_timer = time::Duration::from_secs(random_between(
        crate::environment::COIN_MIN_TIME as i64,
        crate::environment::COIN_MAX_TIME as i64,
    ) as u64);

    loop {
        interval.tick().await;
        Handler::send_command_isolated(&sender, Command::UpdateCoins).await;
        if (time::Instant::now() - start_time) > coin_timer {
            // runs every second
            if let Some(coin_message) =
                Handler::send_command_isolated(&sender, Command::CreateCoin).await
            {
                let message = Into::<ChannelId>::into(crate::environment::coin_channel())
                    .send_message(&ctx.http, CreateMessage::new().content(coin_message))
                    .await?;
                // how do i get this message out there?
                // pass it through
                Handler::send_command_isolated(
                    &sender,
                    Command::CoinCreateNotification(message, ctx.http.clone()),
                )
                .await;
                eprintln!("Coin message sent!");
                tokio::task::spawn(coin_timer_func(sender.clone()));
            }
            start_time = time::Instant::now();
            coin_timer = time::Duration::from_secs(random_between(
                crate::environment::COIN_MIN_TIME as i64,
                crate::environment::COIN_MAX_TIME as i64,
            ) as u64);
        }
    }
}

async fn coin_timer_func(sender: Sender<Request>) {
    eprintln!(
        "Starting {} counter to coin timing out.",
        crate::environment::COIN_TIMEOUT
    );
    tokio::time::sleep(tokio::time::Duration::from_secs(
        crate::environment::COIN_TIMEOUT,
    ))
    .await;
    eprintln!("Coin timer gone off!");
    Handler::send_command_isolated(&sender, Command::CoinEscape).await;
}
