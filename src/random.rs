#[cfg(not(target_family = "wasm"))]
use rand::{thread_rng, Rng};

#[cfg(not(target_family = "wasm"))]
pub fn random_range(min: usize, max: usize) -> usize {
    let mut rng = thread_rng();
    rng.gen_range(min..max)
}
