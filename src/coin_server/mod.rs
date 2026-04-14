// add existing modules
pub mod coin;
pub mod file_management;
pub mod rarity;
pub mod server;
pub mod user;

pub use self::{coin::Coin, file_management::Serialisable, rarity::Rarity, user::User};
