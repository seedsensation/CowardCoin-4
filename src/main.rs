use std::collections::HashMap;

use coin_server::Coin;
use coin_server::Rarity;

mod coin_server;
mod helpers;

fn main() {
    println!("Hello, world!");
    let mut values: HashMap<Rarity, i32> = HashMap::new();
    for _ in 1..Rarity::max_rarity().get_exponent() / 10 {
        let chosen = Rarity::generate();
        let value = values.get_mut(&chosen);
        if value.is_none() {
            values.insert(chosen, 1);
        } else {
            *value.unwrap() += 1;
        }

        if chosen > Rarity::MYTHICAL {
            println!(
                "{}",
                Coin {
                    rarity: chosen,
                    value: chosen.calculate_value()
                }
                .arrival_message()
            );
        } else {
            Coin {
                rarity: chosen,
                value: chosen.calculate_value(),
            }
            .arrival_message();
        }
    }
    for (key, val) in values.drain() {
        println!("{}: {}", key.name(), val);
    }

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
