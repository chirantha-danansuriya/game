use rand::{thread_rng, Rng};
pub fn guess() -> u32 {
    let mut rng = thread_rng();
    let machine_guess: u32 = rng.gen_range(1..101);
    machine_guess
}

