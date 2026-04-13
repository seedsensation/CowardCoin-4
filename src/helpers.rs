use rand::{distributions::uniform::SampleUniform, Rng};

pub fn random_between<T>(low: T, high: T) -> T
where
    T: Ord + SampleUniform,
{
    rand::thread_rng().gen_range(low..high)
}
