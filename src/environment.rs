use crate::Duration;
pub const TIME_BETWEEN_TRICKS: Duration = Duration::from_hours(4);
pub const MARKET_CHANGE_TIMER: Duration = Duration::from_hours(1);
pub const INVESTMENT_TIMER: Duration = Duration::from_mins(10);
pub const BOT_ID: u64 = 813814751192809543;
//pub const BOT_ID: u64 = 1023717268624003113;

pub const COIN_MIN_TIME: u64 = 900;
pub const COIN_MAX_TIME: u64 = 1500;

pub fn coin_channel() -> u64 {
    dotenv::dotenv().ok();
    str::parse(&std::env::var("COIN_CHANNEL").expect("Expected COIN_CHANNEL in the environment"))
        .unwrap()
}
