use rand::{Rng, distributions::uniform::SampleUniform};

pub fn random_between<T>(low: T, high: T) -> T
where
    T: Ord + SampleUniform,
{
    rand::thread_rng().gen_range(low..high)
}

pub fn s_if(val: i64) -> String {
    match val {
        1 => "",
        _ => "s",
    }
    .into()
}
