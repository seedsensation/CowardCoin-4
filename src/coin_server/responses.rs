use super::Coin;
use super::Rarity;
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

/**
An enum that can contain multiple different types, for handling responses.

More ResponseTypes can be easily added.
When you match ResponseType, make sure you add:
```
match v {
    // ...
    _ => Err(ResponseTypeError::UnhandledResponseType)
}
```
*/
pub enum ResponseType {
    MsgText(String),
    MsgCoin(Coin),
    MsgRarity(Rarity),
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
    let (tx, rx) = channel::<ResponseType>();

    to.send(Request {
        contents: message,
        reply: tx,
    });
    Ok(rx)
}

impl From<String> for ResponseType {
    fn from(value: String) -> Self {
        ResponseType::MsgText(value)
    }
}

impl fmt::Display for ResponseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ResponseType::*;
        write!(
            f,
            "{}",
            match self {
                MsgText(s) => s.clone(),
                MsgCoin(c) => c.rarity.name().to_string(),
                MsgRarity(r) => r.name().to_string(),
                _ => "Unknown Response Type".into(),
            }
        )
    }
}
