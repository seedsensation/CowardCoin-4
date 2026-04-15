// add existing modules
mod coin;
pub mod file_management;
mod rarity;
pub mod responses;
mod server;
mod user;

pub use self::{
    coin::Coin,
    file_management::Serialisable,
    rarity::Rarity,
    responses::{Request, ResponseType},
    server::Server,
    user::User,
};
