// random_number_guessing/src/number_gen.rs

use rand::Rng;

pub fn generate_secret_number() -> i32 {
    rand::thread_rng().gen_range(1..=100)
}
