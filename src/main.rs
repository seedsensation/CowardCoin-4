use std::collections::HashMap;

use coin_server::Coin;
use coin_server::Rarity;
use coin_server::Request;
use coin_server::Server;
use coin_server::User;

use std::sync::mpsc::{channel, sync_channel};

mod coin_server;
mod helpers;

fn rarity_test() {
    let mut values: HashMap<Rarity, i32> = HashMap::new();
    for _ in 1..Rarity::max_rarity().get_exponent() / 10 {
        let chosen = Coin::new();
        let value = values.get_mut(&chosen.rarity);
        if value.is_none() {
            values.insert(chosen.rarity, 1);
        } else {
            *value.unwrap() += 1;
        }
    }
    for (key, val) in values.drain() {
        println!("{}: {}", key.name(), val);
    }
}

fn generation_benchmark() {
    let start_free = std::time::Instant::now();
    for _ in 1..1000000 {
        Coin::new();
    }
    eprintln!("Time to run without output: {:?}", start_free.elapsed());
    let start_output = std::time::Instant::now();
    for _ in 1..1000000 {
        Coin::new().arrival_message();
    }
    eprintln!("Time to run with output: {:?}", start_output.elapsed());
}

fn main() {
    rarity_test();
    // generation_benchmark()

    let (tx, rx) = channel::<Request>();

    let mut server = Server::load(rx);
    server.users.push(User {
        user_id: 0,
        coin_count: 0,
    });
    server.save();
}
