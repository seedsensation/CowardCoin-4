// add existing modules
mod coin;
pub mod file_management;
mod rarity;
mod server;
mod user;

pub use self::{
    coin::Coin, file_management::Serialisable, rarity::Rarity, server::Server, user::User,
};
