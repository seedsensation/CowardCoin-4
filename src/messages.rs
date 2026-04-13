use std::fmt;

use poise::serenity_prelude::CacheHttp;

use crate::serenity::ChannelId;

/// Send a message to a given channel
pub async fn send_message<T, H>(channel_id: ChannelId, http: H, message: T)
where
    T: fmt::Display,
    H: CacheHttp,
{
    if let Err(why) = channel_id.say(http, message.to_string()).await {
        println!("Error sending message: {why:?}");
    }
}
