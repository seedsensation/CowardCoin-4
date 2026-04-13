use crate::coin_type::CoinType;
use rand::prelude::IndexedRandom;

pub struct Coin {
    pub coin_type: CoinType,
    pub value: i32,
}

impl Coin {
    pub fn new(coin_type: CoinType) -> Coin {
        Coin {
            coin_type: coin_type.clone(),
            value: coin_type.generate_value(),
        }
    }

    pub fn coin_arrival_message(&self) -> String {
        let mut rng = rand::rng();
        format!(
            "{}, it's {}{}\nIt's worth {} coin{}.",
            {
                *vec![
                    "Riding into the room on a skateboard",
                    "Lying, alone and unloved, on the ground",
                    "Throwing rocks at passing cars",
                    "Doing a sick-ass kickflip into the room",
                    "Sleeping in until 2pm",
                    "Look out",
                    "For the love of god, help me",
                    "Get the wife and kids somewhere safe",
                ]
                .choose(&mut rng)
                .unwrap()
            },
            self.coin_type.coin_descriptor(),
            {
                *vec![".", "!", "?", "!?", "!?!?!?!?!?!?"]
                    .choose(&mut rng)
                    .unwrap()
            },
            self.value,
            { if self.value == 1 { "" } else { "s" } }
        )
    }
}
