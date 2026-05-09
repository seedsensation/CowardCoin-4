use crate::communication::BotUser;
use crate::helpers::s_if;
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
        "hi".into()
    }
}

impl Server {
    fn arena_intro(&mut self, bot_user: BotUser) -> String {
        let user = self.get_user_from_id(&bot_user);
        let sender_coin_potential = std::cmp::max(
            (user.coins as f64 * (1.1 * (user.level as f64 - 1.0))) as i64,
            user.coins,
        );
        format!(
            "
Welcome to the **COIN ARENA**!

{} {} - Lv. {} - {} Coin{}
[{}] - {}/100
Give {} more coin{} to reach the next level.

You can currently do a trick worth {} coin{}.
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
            sender_coin_potential as i64,
            s_if(sender_coin_potential as i64)
        )
    }
}

impl CoinUser {
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
