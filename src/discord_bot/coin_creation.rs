use crate::communication::{Command, Request};
use crate::discord_bot::Handler;
use crate::helpers::random_between;
use serenity::all::ChannelId;
use serenity::all::Context;
use serenity::all::CreateMessage;
use serenity::all::Result;
use tokio::sync::mpsc::Sender;
use tokio::time::{self, Duration};

pub async fn coin_creation_check(
    period: Duration,
    sender: Sender<Request>,
    ctx: Context,
) -> Result<()> {
    let mut interval = time::interval(period);
    let mut start_time = time::Instant::now();
    let mut coin_timer = time::Duration::from_secs(random_between(
        crate::environment::COIN_MIN_TIME as i64,
        crate::environment::COIN_MAX_TIME as i64,
    ) as u64);

    loop {
        interval.tick().await;
        Handler::send_command_isolated(&sender, Command::UpdateCoins).await;
        if (time::Instant::now() - start_time) > coin_timer {
            // runs every second
            if let Some(coin_message) =
                Handler::send_command_isolated(&sender, Command::CreateCoin).await
            {
                let message = Into::<ChannelId>::into(crate::environment::coin_channel())
                    .send_message(&ctx.http, CreateMessage::new().content(coin_message))
                    .await?;
                // how do i get this message out there?
                // pass it through
                Handler::send_command_isolated(
                    &sender,
                    Command::CoinCreateNotification(message, ctx.http.clone()),
                )
                .await;
                eprintln!("Coin message sent!");
                tokio::task::spawn(coin_timer_func(sender.clone()));
            }
            start_time = time::Instant::now();
            coin_timer = time::Duration::from_secs(random_between(
                crate::environment::COIN_MIN_TIME as i64,
                crate::environment::COIN_MAX_TIME as i64,
            ) as u64);
        }
    }
}

async fn coin_timer_func(sender: Sender<Request>) {
    eprintln!(
        "Starting {} counter to coin timing out.",
        crate::environment::COIN_TIMEOUT
    );
    tokio::time::sleep(tokio::time::Duration::from_secs(
        crate::environment::COIN_TIMEOUT,
    ))
    .await;
    eprintln!("Coin timer gone off!");
    Handler::send_command_isolated(&sender, Command::CoinEscape).await;
}
