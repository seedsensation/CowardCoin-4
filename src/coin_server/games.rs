use std::time::SystemTime;

use super::Server;
use crate::choose_message;
use crate::discord_bot::BotUser;
use crate::helpers::*;

pub trait Games {
    fn arena(&mut self, bot_user: BotUser, command: Vec<String>) -> String;
    fn trick(&mut self, bot_user: BotUser, amount: i64) -> String;
    fn invest(&mut self, bot_user: BotUser, amount: i64) -> String;
}

enum TrickState {
    Err,
    Fail,
    Bad,
    Good,
    Crit,
}

impl From<i64> for TrickState {
    fn from(value: i64) -> Self {
        use TrickState::*;
        if !(0..=100).contains(&value) {
            Err
        } else if value >= 85 {
            Crit
        } else if value > 60 {
            Good
        } else if value >= 20 {
            Bad
        } else {
            Fail
        }
    }
}

impl Games for Server {
    fn arena(&mut self, bot_user: BotUser, input: Vec<String>) -> String {
        let user = self.get_mut_user_from_id(&bot_user);
        if let Some(command_upper) = input.get(2) {
            let command = command_upper.to_lowercase();
            if command == "train" {
                if let Some(amount_str) = input.get(3) {
                    if let Ok(amount) = str::parse::<i64>(amount_str) {
                        if amount > user.coins {
                            return "You can't afford to train _that_ hard in the arena."
                                .to_string();
                        } else if amount < 0 {
                            return "Nice try...".to_string();
                        } else {
                            user.coins -= amount;
                            let response = user.add_xp_with_response(amount);
                            if let Err(why) = self.save() {
                                return format!("Error saving to file: {why:?}");
                            }
                            return format!(
                                "You pay {amount} CowardCoin{} to {}. You gain {amount} XP.{}",
                                s_if(amount),
                                choose_message![
                                    "the altar of progress",
                                    "the shrine of Capitalism",
                                    "the Gym Gods",
                                    "the Minotaur",
                                    "the Coins themselves"
                                ],
                                response
                            );
                        }
                    } else {
                        return "Please format your message `coin arena train <number>`."
                            .to_string();
                    }
                } else {
                    return "Please format your message `coin arena train <number>`.".to_string();
                }
            }
        }
        self.arena_intro(bot_user)
    }
    fn trick(&mut self, bot_user: BotUser, amount: i64) -> String {
        // figure out whether a trick is legal
        let user = self.get_mut_user_from_id(&bot_user);

        if user.coins == 0 {
            "You need to have at least 1 coin to do a trick.".to_string()
        } else if user.level >= (required_level_for_trick(amount, user.coins) - 1) {
            if SystemTime::now()
                .duration_since(user.time_of_last_trick)
                .unwrap()
                < crate::environment::TIME_BETWEEN_TRICKS
            {
                let full_time = (crate::environment::TIME_BETWEEN_TRICKS
                    - SystemTime::now()
                        .duration_since(user.time_of_last_trick)
                        .unwrap())
                .as_secs() as i64;
                return format!(
                    "You're too weak after your last trick! Try again in {}.",
                    crate::helpers::seconds_to_string(full_time)
                );
            }
            // generate coin trick message
            let points = if amount == 0 {
                0
            } else {
                random_between(1, 100)
            };
            user.style_points += points;
            user.time_of_last_trick = SystemTime::now();
            let trick_state: TrickState = points.into();
            let thrown_item = choose_message![
                "Christian baby",
                "rotten tomato",
                "Oscar Trophy",
                "Nuclear Bomb",
                "Gold CowardCoin",
            ];

            let distraction = choose_message![
                "women",
                "men",
                "Planet 51: The Game",
                "CowardCoins",
                "pigs, hogs and slop",
                "Dark Admiral Chex Mix",
                "dave strider",
                "the meaning of life",
                "fruit loops",
                "The Notorious B.U.B.",
                "World of Warcraft"
            ];
            let dangerous_hazard = choose_message![
                "speeding train",
                "rake on the ground",
                "oil spill",
                "ten-tonne truck",
                "woke dog",
                "pride parade",
                "baked beans on the ground",
                "shellfish",
                "crab",
                "orca",
                "goldfish",
                "bowl of Chex Mix (sponsored by Chex Quest)",
                "fax machine"
            ];

            // how do i make it choose from a list of strings,
            // but have those strings also be able to have substitutions?
            // i think the key is to have it be Strings rather than &str
            format!(
                "You {}, and {}\n{}\n{}You gained {points} StylePoints™.",
                choose_message![
                    "launch yourself into the air gracefully",
                    format!(
                        "do a{} off a {}{}",
                        choose_message![
                            "n awesome kickflip",
                            " frontflip for the ages",
                            " quintuple grind rail with 100x score multiplier"
                        ],
                        choose_message!["little puppy", "wide-eyed baby", "gnome", "pedestrian"],
                        choose_message![
                            "",
                            "",
                            "",
                            "",
                            "'s head",
                            "'s propeller hat",
                            "'s car",
                            "'s super-awesome mecha suit"
                        ]
                    ),
                    format!(
                        "open a {} of {}",
                        choose_message!["can", "tin", "bottle", "cardboard box", "packet"],
                        choose_message![
                            "baked beans",
                            "barbecue sauce",
                            "tomato ketchup",
                            "footy scran",
                            "happy thoughts",
                            "mayonnaise"
                        ]
                    ),
                    "explode messily all over the place",
                    format!(
                        "fall out of the {} (with{} a parachute)",
                        choose_message![
                            "International Space Station",
                            "Eiffel Tower",
                            "Burj Khalifa",
                            "Museum From **Planet 51: The Game**"
                        ],
                        choose_message!["", "out"]
                    )
                ],
                match trick_state {
                    TrickState::Err => "something goes terribly wrong...".to_string(),
                    TrickState::Fail => choose_message![
                        "unfortunately die in the attempt...",
                        "embarrass yourself horrifically. You'll never be able to recover from this.",
                        "land head-first on the ground...",
                        "land on the back of a horse! The horse bucks you off, and you land in a crumpled heap on the ground.",
                        format!(
                            "are too distracted thinking about {distraction} to see the {dangerous_hazard} right in front of you! {}",
                            choose_message![
                                "You are crushed instantly.",
                                "It slams into your face, and your nose hurts.",
                                "Oh no! Your wallet's been stolen!",
                                "You die instantly."
                            ]
                        ),
                        format!(
                            "break all of the bones in your {}...",
                            choose_message![
                                "arm", "leg", "head", "chest", "knee", "elbow", "groin", "foot",
                                "feet", "hand", "hands"
                            ]
                        )
                    ],
                    TrickState::Crit => choose_message![
                        "land gracefully on one leg!",
                        format!(
                            "do a {} flip before you land!",
                            choose_message![
                                "double",
                                "triple",
                                "quadruple",
                                "quintuple",
                                "sextuple"
                            ]
                        ),
                        "almost fall, but recover stylishly in a way that makes it look planned!",
                        "miraculously survive without a scratch!",
                        "land on the back of a horse! The horse looks at you, and winks.",
                        format!(
                            "avoid the temptation of {distraction}, and take the gold medal at the Olympics!"
                        )
                    ],
                    TrickState::Bad | TrickState::Good => choose_message![
                        "it's pretty mediocre...",
                        "awkwardly stumble as you land.",
                        "land on the back of a horse! The horse stares at you in disappointment.",
                        "fall to your knees, shedding a single tear at what could have been.",
                        "sigh, thinking about the good times that you'll never be able to return to.",
                        format!(
                            "are too distracted thinking about {distraction} to see the {dangerous_hazard} right in front of you! {}",
                            choose_message![
                                "You stub your toe!",
                                "You dodge out of the way just in time, narrowly avoiding a grisly fate.",
                                "You barely escape with your life!",
                                "You survive by the skin of your teeth."
                            ]
                        ),
                    ],
                },
                match trick_state {
                    TrickState::Crit => choose_message![
                        "The crowd jump out of their seats. Every single one of them does a backflip.",
                        "The crowd levitate off the ground ominously. Their eyes glow white.",
                        "The crowd storms the field, and murders the pitcher.",
                        "The crowd pelts you with perfectly ripe tomatoes, and you eat every single one of them.",
                        format!(
                            "One member of the crowd throws a {thrown_item} at you. {}",
                            choose_message![
                                "You hit it out of the park, and score a home run!",
                                "You catch it, and become a national hero!",
                                "You catch it in your mouth, and swallow it whole!",
                                "You catch it, and dedicate the rest of your life to caring for it!"
                            ]
                        ),
                    ],
                    TrickState::Good | TrickState::Bad => choose_message![
                        "The crowd goes 'huh?'.",
                        "The crowd doesn't notice.",
                        "The crowd cheers, but you can tell that their heart isn't really in it.",
                        "One member of the crowd throws a raw egg at you. He swears it should have been hard-boiled.",
                        format!(
                            "One member of the crowd throws a {thrown_item} at you. {}",
                            choose_message![
                                "It flies past you, into the stands.",
                                "It flies past you, into the street.",
                                "It flies past you, landing in the grass.",
                                "You try to catch it, and miss."
                            ]
                        ),
                        format!(
                            "You can see {} in the crowd. They seem disappointed in you.",
                            choose_message![
                                "your parents",
                                "your entire extended family",
                                "all of Radio TV Solutions",
                                "dave strider",
                                "the Jolly Green Giant",
                                "the Gerber Daby",
                                "The Game Chimp",
                                "PSP",
                                "UMD",
                                "Dr. Harold Pontiff Coomer",
                                "Tommy Coolatta"
                            ],
                        )
                    ],
                    TrickState::Fail => choose_message![
                        "The crowd goes wild with dismay, then commits mass suicide.",
                        "The crowd sighs in relief. They'll be able to get a refund, at least.",
                        format!(
                            "Someone throws a {thrown_item} at you. {}",
                            choose_message![
                                "It explodes as it hits your head.",
                                "It splatters on the ground.",
                                "It bounces off your head.",
                                "It hits you in your stomach, winding you.",
                                "It hits you in the back of the knee, forcing you to the ground."
                            ]
                        )
                    ],

                    TrickState::Err =>
                        "The crowd glitches. Something has gone terribly wrong.".to_string(),
                },
                match trick_state {
                    TrickState::Fail => {
                        user.coins -= amount;
                        format!(
                            "You lose {amount} coin{}...\nYou now have {} coins.\n",
                            s_if(amount),
                            user.coins
                        )
                    }
                    TrickState::Crit => {
                        user.coins += amount;
                        format!(
                            "You gain {amount} coin{}!!\nYou now have {} coins.\n",
                            s_if(amount),
                            user.coins
                        )
                    }
                    _ => "".to_string(),
                }
            )
        } else {
            format!(
                "You aren't powerful enough for a trick as dangerous as that!\nTry getting to level {} in the **COIN ARENA** first...",
                required_level_for_trick(amount, user.coins)
            )
        }
    }

    /// Invest coins into the CowardCoin Bank
    fn invest(&mut self, bot_user: BotUser, amount: i64) -> String {
        enum InvestmentStatus {
            Success,
            Dividends(i64),
            NotTimeYet(SystemTime),
        }

        let user_coins: i64;
        let bank_coins: i64;

        // SAFETY: Do not sort self.users while this scope is active
        match unsafe {
            let (sender_local, bank) =
                self.get_two_mut_users(&bot_user, &(crate::environment::BOT_ID.into()));
            sender_local.coins -= amount;
            bank.coins += amount * 5;

            user_coins = sender_local.coins;
            bank_coins = bank.coins;

            if SystemTime::now()
                .duration_since(sender_local.time_of_last_investment)
                .unwrap()
                < crate::environment::INVESTMENT_TIMER
            {
                InvestmentStatus::NotTimeYet(sender_local.time_of_last_investment)
            } else {
                sender_local.time_of_last_investment = SystemTime::now();

                let chance = crate::helpers::random_between(0, 100);
                if chance >= 80 {
                    let diff = bank.coins / 2;
                    sender_local.coins += diff;
                    InvestmentStatus::Dividends(diff)
                } else {
                    InvestmentStatus::Success
                }
            }
        } {
            InvestmentStatus::Success => {
                format!(
                    "You have invested {} coin{} in the CowardCoin Bank™.\nYou now have {} coin{}.\nThere are now {} coin{} in the CowardCoin Bank™.",
                    amount,
                    s_if(amount),
                    user_coins,
                    s_if(user_coins),
                    bank_coins,
                    s_if(bank_coins)
                )
            }
            InvestmentStatus::Dividends(dividends) => {
                format!(
                    "Congratulations! Your investments have paid off! \nYou receive {} coin{} in dividends from the CowardCoin Bank™.",
                    dividends,
                    s_if(dividends)
                )
            }
            InvestmentStatus::NotTimeYet(system_time) => format!(
                "The stock market's still shifting... You can't make any more investments for another {}.",
                crate::helpers::seconds_to_string(
                    if SystemTime::now().duration_since(system_time).unwrap()
                        < crate::environment::INVESTMENT_TIMER
                    {
                        (crate::environment::INVESTMENT_TIMER
                            - SystemTime::now().duration_since(system_time).unwrap())
                        .as_secs() as i64
                    } else {
                        0i64
                    }
                )
            ),
        }
    }
}

impl super::Server {
    fn arena_intro(&mut self, bot_user: BotUser) -> String {
        let user = self.get_user_from_id(&bot_user);
        format!(
            "
Welcome to the **COIN ARENA**!

{} {} - Lv. {} - {} Coin{}
[{}] - {}/{}
Give {} more coin{} to reach the next level.
To train, use `coin arena train [number]`.

Your current strength allows you to perform a **Coin Trick** worth {} coin{}.
	",
            user.arena_title(),
            user.nickname.clone().unwrap_or(bot_user.display_name),
            user.level,
            user.coins,
            s_if(user.coins),
            user.xp_bar(),
            user.xp,
            user.xp_cap(),
            100 - user.xp,
            s_if(100 - user.xp),
            user.max_coins_for_trick(),
            s_if(user.max_coins_for_trick())
        )
    }
}

#[inline]
fn required_level_for_trick(amount: i64, count: i64) -> i64 {
    // max(coins * (1.1 * (level - 1))) = amount
    // level = (level / coins / 1.1) + 1
    f64::ceil((amount as f64 / f64::max(i64::abs(count) as f64, 1.0) / 1.1) + 1.0) as i64
}

impl super::CoinUser {
    fn max_coins_for_trick(&self) -> i64 {
        if self.coins < 0 {
            i64::abs(self.coins)
        } else {
            let val =
                self.coins as f64 + f64::ceil(self.coins as f64 * 0.1 * (self.level - 1) as f64);
            if val < i64::MAX as f64 {
                val as i64
            } else {
                self.coins
            }
        }
    }

    pub fn arena_title(&self) -> String {
        match self.level {
            0..=4 => "Harmless",
            5..=9 => "Mostly Harmless",
            10..=14 => "Gambling Addict(?)",
            15..=19 => "Gambling Addict(!)",
            20..=24 => "Biggest Clown at the Circus",
            25..=49 => "Lost All Their Money On The Horse Races",
            50 => "Top Of The Bell Curve",
            51..=74 => "Please Stop",
            75..=79 => "I Will Pay You One United States Dollar To Stop Gambling",
            80..=89 => "I Will Pay You Two United States Dollars To Stop Gambling",
            90..=94 => "Okay Deal's Off I Ran Out Of Money",
            95..=97 => "You Can Stop At 100",
            98 => "Please Stop At 100",
            99 => "One More Level",
            100 => "SUPREME COIN CHAMPION FOREVER",
            101 => "for fucks sake",
            102 => "okay fuck it",
            103 => "this next one's the last one",
            104..=149 => "BEARER OF THE GAMBLING CROWN OF SHAME",
            150..=199 => "*BEARER OF THE GAMBLING CROWN OF SHAME*",
            200..=249 => "**BEARER OF THE GAMBLING CROWN OF SHAME**",
            250..=499 => "**B E A R E R  O F  T H E  G A M B L I N G  C R O W N  O F  S H A M E**",
            500..=999 => "Gambled Your Crown Away",
            1000 => "won it back?",
            1001 => "lost it again",
            1002..=4999 => "Gambled Your Crown Away",
            5000..=9999 => {
                "I'll give you another supreme coin champion trophy at level 10,000. Will you stop then?"
            }
            10000 => "I WAS LYING. WELCOME TO THE COIN ASCENDANCY, FUCKER. HOW HIGH WILL YOU GO?",
            10001..=99999 => "Coin Ascendancy Hopeful",
            100000..=999999 => "Coin Ascendancy Initiate",
            1000000..=9999999 => "Member of the Coin Ascendancy",
            10000000..=99999999 => "Coin Ascendancy Nobility",
            100000000..=999999999 => "Coin Ascendancy Royalty",
            1000000000..=9999999999 => "Supreme Ruler of the Coin Ascendancy",
            10000000000..=99999999999 => "Deity of the Coin Ascendancy",
            100000000000 => "Betrayed by the Coin Ascendancy",
            _ => "Buried in a Hole",
        }.into()
    }

    pub fn xp_bar(&self) -> String {
        "▓".repeat(((self.xp / self.xp_cap()) * 10) as usize)
            + "░"
                .repeat(10 - ((self.xp / self.xp_cap()) * 10) as usize)
                .as_str()
    }
}
