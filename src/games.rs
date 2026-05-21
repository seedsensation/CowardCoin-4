use rand::Rng;
use rand::seq::SliceRandom;

use crate::choose_message;
use crate::communication::BotUser;
use crate::helpers::*;
use crate::server::Server;
use crate::user::CoinUser;

pub trait Games {
    fn arena(&mut self, bot_user: BotUser, command: Vec<String>) -> String;
    fn trick(&mut self, bot_user: BotUser, amount: i64) -> String;
}

enum TrickState {
    ERR,
    FAIL,
    BAD,
    GOOD,
    CRIT,
}

impl From<i64> for TrickState {
    fn from(value: i64) -> Self {
        use TrickState::*;
        if value > 100 || value < 0 {
            ERR
        } else if value >= 85 {
            CRIT
        } else if value > 60 {
            GOOD
        } else if value >= 20 {
            BAD
        } else {
            FAIL
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
                        } else {
                            user.coins -= amount;
                            let response = user.add_xp_with_response(amount);
                            if let Err(_) = self.save() {
                                return "Error saving to file.".to_string();
                            }
                            return format!(
                                "You pay {amount} CowardCoins to {}. You gain {amount} XP.{}",
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

        if amount <= user.max_coins_for_trick() {
            // generate coin trick message
            let points = if amount == 0 {
                0
            } else {
                random_between(1, 100)
            };
            user.style_points += points;
            let trick_state: TrickState = points.into();
            let thrown_item = *random_from::<&str>(&vec![
                "Christian baby",
                "rotten tomato",
                "Oscar Trophy",
                "Nuclear Bomb",
                "Gold CowardCoin",
            ]);

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
                    TrickState::ERR => "something goes terribly wrong...".to_string(),
                    TrickState::FAIL => choose_message![
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
                    TrickState::CRIT => choose_message![
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
                    TrickState::BAD | TrickState::GOOD => choose_message![
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
                    TrickState::CRIT => choose_message![
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
                    TrickState::GOOD | TrickState::BAD => choose_message![
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
                    TrickState::FAIL => choose_message![
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

                    TrickState::ERR =>
                        "The crowd glitches. Something has gone terribly wrong.".to_string(),
                },
                match trick_state {
                    TrickState::FAIL => {
                        user.coins -= amount;
                        format!(
                            "You lose {amount} coins...\nYou now have {} coins.\n",
                            user.coins
                        )
                    }
                    TrickState::CRIT => {
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
}

impl Server {
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
            s_if(user.max_coins_for_trick() as i64)
        )
    }
}

fn required_level_for_trick(amount: i64, count: i64) -> i64 {
    // max(coins * (1.1 * (level - 1))) = amount
    // level = (level / coins / 1.1) + 1
    f64::ceil((amount as f64 / f64::max(count as f64, 1f64) / 1.1 as f64) + 1.0) as i64
}

impl CoinUser {
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
        return "▓".repeat(((self.xp / self.xp_cap()) * 10) as usize)
            + "░"
                .repeat(10 - ((self.xp / self.xp_cap()) * 10) as usize)
                .as_str();
    }
}
