use super::coin_server::Coin;
use super::coin_server::Rarity;
use super::coin_server::User;

use std::fmt;
use std::io;
use tokio::sync::mpsc::{Receiver, Sender, channel};

/// Possible errors for the ResponseType enum
#[derive(Debug, Clone)]
pub enum ResponseTypeError {
    InvalidTypeConversion,
    UnhandledResponseType,
}

impl fmt::Display for ResponseTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ResponseTypeError::*;
        write!(
            f,
            "{}",
            match self {
                InvalidTypeConversion => "Invalid ResponseType Conversion",
                _ => "Not Implemented Error",
            }
        )
    }
}

// COMPLETE THESE LATER - THIS IS A MENU
pub enum GetType {}

pub enum SendType {
    Coin(Coin),
    String(String),
    Rarity(Rarity),
}

pub enum EditType {
    User(i32, User),
}

pub enum RequestType {
    Get(GetType),
    Send(SendType),
    Edit(EditType),
}

/// A struct that holds a request, and a possible reply.
pub struct Request {
    contents: ResponseType,
    reply: Sender<ResponseType>,
}

/// Send a request, and return a Receiver if the message sends successfully
pub fn send_request(
    message: ResponseType,
    to: Sender<Request>,
) -> io::Result<Receiver<ResponseType>> {
    let (tx, rx) = channel::<ResponseType>(100);

    to.send(Request {
        contents: message,
        reply: tx,
    });
    Ok(rx)
}
