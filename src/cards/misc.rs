use rand::prelude::*;

// generates a number in the range from 0 to max (exlusive on max)
pub fn range_rand(max: usize) -> usize {
    let mut rng = rand::thread_rng();
    return rng.gen_range(0..max);
}

fn main() {}
