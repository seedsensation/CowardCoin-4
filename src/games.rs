use rand::Rng;
use rand::seq::SliceRandom;

use crate::communication::BotUser;
use crate::helpers::*;
use crate::server::Server;
use crate::user::CoinUser;

pub trait Games {
    fn arena(&mut self, bot_user: BotUser, command: Vec<String>) -> String;
    fn trick(&mut self, bot_user: BotUser, amount: i64) -> String;
}

impl Games for Server {
    fn arena(&mut self, bot_user: BotUser, command: Vec<String>) -> String {
        self.arena_intro(bot_user)
    }
    fn trick(&mut self, bot_user: BotUser, amount: i64) -> String {
        // figure out whether a trick is legal
        let user = self.get_mut_user_from_id(&bot_user);

        if amount <= user.max_coins_for_trick() {
            // generate coin trick message
            let points = random_between(1, 100);
            let trick_crit = points > 90;
            let trick_fail = points < 25;
            let string_landing: String;
            let string_audience: String;
            let animal = *random_from::<&str>(&vec![
                "porpoise",
                "turtle",
                "dolphin",
                "whale",
                "small child's head",
                "horse",
                "unicorn",
                "goldfish",
                "hammerhead shark",
                "victorian child covered in coal dust",
                "christian baby",
            ]);
            let person = *random_from::<&str>(&vec![
                "a pensioner",
                "a major world leader",
                "a flesh clone of Barack Obama",
                "Barack Obama",
                "Joe Biden",
                "Dave Strider from Homestuck",
                "a [Walkin' Horse](https://me.rcury.com/walkan)",
                "Wayne's Radio TV",
                "Benny from Half Life Funny",
            ]);
            let thrown_item = *random_from::<&str>(&vec![
                "Christian baby",
                "rotten tomato",
                "Oscar Trophy",
                "Nuclear Bomb",
                "Gold CowardCoin",
            ]);

            //if trick_fail {
            //    user.coins -= amount;
            //} else if trick_crit {
            //    user.coins += amount;
            //}

            format!(
                "You {}, and {}.\n{}\n{}You gained {points} StylePoints™.",
                // initial trick
                random_from(&vec![
                    "launch yourself into the air gracefully",
                    (String::from("do a kickflip off a ") + animal).as_str(),
                    "open a can of baked beans",
                    "explode messily all over the place",
                    (String::from("run rings around ") + person).as_str(),
                    "fall out of the International Space Station (without a parachute)"
                ]),
                // success
                if trick_fail {
                    *random_from::<&str>(&vec![
                        "unfortunately die in the attempt...",
                        "embarrass yourself horrifically. You'll never be able to recover from this.",
                        "land head-first on the ground...",
                        "land on the back of a horse! The horse bucks you off, and you land in a crumpled heap on the ground...",
                    ])
                } else if trick_crit {
                    *random_from::<&str>(&vec![
                        "land gracefully on one leg!",
                        "do a quintuple flip before you land!",
                        "almost fall, but recover in a way that makes it look planned!",
                        "catch your coins in your mouth halfway through the trick!",
                        "miraculously survive without a scratch!",
                        "land on the back of a horse! The horse looks at you, and winks.",
                    ])
                } else {
                    *random_from::<&str>(&vec![
                        "it's pretty mediocre..",
                        "awkwardly stumble as you land.",
                        {
                            string_landing = format!(
                                "land on the back of a {}, and the {} stares at you in disappointment.",
                                animal, animal
                            );
                            string_landing.as_str()
                        },
                        "fall to your knees, shedding a single tear at what could have been.........",
                    ])
                },
                // crowd reaction
                if trick_fail {
                    *random_from::<&str>(&vec![
                        "The crowd goes wild with dismay, then commits mass suicide.",
                        "The crowd sighs in relief. They'll be able to get a refund.",
                        {
                            let collision = *random_from::<&str>(&vec![
                                "It explodes as it hits your head.",
                                "It splatters on the ground.",
                            ]);
                            string_audience = format!(
                                "One member of the crowd throws a {} at you. {}",
                                thrown_item, collision
                            );
                            string_audience.as_str()
                        },
                    ])
                } else if trick_crit {
                    *random_from::<&str>(&vec![
                        "The crowd jump out of their seats. Every single one of them does a backflip.",
                        "The crowd levitate off the ground ominously. Their eyes glow white.",
                        "The crowd storms the field, and murders the pitcher.",
                        "The crowd pelts you with perfectly ripe tomatoes, and you eat every single one of them.",
                        {
                            string_audience = format!(
                                "One member of the crowd throws a {} at you. You hit it out of the park, and score a home run!",
                                thrown_item
                            );
                            string_audience.as_str()
                        },
                    ])
                } else {
                    *random_from::<&str>(&vec![
                        "The crowd goes 'huh?'.",
                        "The crowd doesn't notice.",
                        "The crowd cheers, but you can tell that their heart isn't really in it.",
                        "One member of the crowd throws a raw egg at you. He swears it should have been hard-boiled.",
                        {
                            string_audience = format!(
                                "You see {} in the crowd. They stare at you, disappointed.",
                                person
                            );
                            string_audience.as_str()
                        },
                    ])
                },
                "hi"
            )

        // trick successful?
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
[{}] - {}/100
Give {} more coin{} to reach the next level.

Your current strength allows you to perform a **Coin Trick** worth {} coin{}.
	",
            user.arena_title(),
            user.nickname.clone().unwrap_or(bot_user.display_name),
            user.level,
            user.coins,
            s_if(user.coins),
            user.xp_bar(),
            user.xp,
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
    f64::ceil((amount as f64 / count as f64 / 1.1 as f64) + 1.0) as i64
}

impl CoinUser {
    fn max_coins_for_trick(&self) -> i64 {
        self.coins + f64::ceil(self.coins as f64 * 0.1 * (self.level - 1) as f64) as i64
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

    fn xp_bar(&self) -> String {
        return "▓".repeat((self.xp / 10) as usize)
            + "░".repeat(10 - (self.xp / 10) as usize).as_str();
    }
}
