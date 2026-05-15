use std::ops::Range;

use rand::prelude::*;

pub fn random_between(low: i64, high: i64) -> i64 {
    (low..high)
        .collect::<Vec<i64>>()
        .choose(&mut rand::rng())
        .unwrap()
        .clone()
}

pub fn random_from<'a, T>(vals: &'a Vec<T>) -> &'a T {
    vals.choose(&mut rand::rng()).unwrap()
}

pub fn random_from_owned<T>(vals: &Vec<T>) -> T
where
    T: Clone,
{
    (vals.choose(&mut rand::rng()).unwrap()).clone()
}

pub fn s_if(val: i64) -> String {
    match val {
        1 => "",
        _ => "s",
    }
    .into()
}
