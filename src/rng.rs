use std::cell::RefCell;
use std::fmt::Debug;
use std::hash::Hash;

use rand_chacha::ChaCha8Rng;
use rand_chacha::rand_core::RngCore;
use rand_chacha::rand_core::SeedableRng;

thread_local! {
    static RNG: RefCell<ChaCha8Rng> = RefCell::new(ChaCha8Rng::from_os_rng());
}
pub struct Rng {}

impl Debug for Rng {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Rng").finish()
    }
}

impl Clone for Rng {
    fn clone(&self) -> Self {
        Self {}
    }
}

impl Rng {
    pub fn new() -> Self {
        Self {}
    }
    //The samples is exclusive on max. It utilizes rejection sampling.
    pub fn sample(&mut self, bound: usize) -> usize {
        if bound == 0 {
            panic!("Invalid range: Max cannot be 0");
        }
        //The RNG is sometimes called with a max of 1, have a fast path for that.
        if bound == 1 {
            return 0;
        }
        let next_pow_2 = bound.next_power_of_two();
        let mask = next_pow_2 - 1;
        loop {
            let rand = { RNG.with_borrow_mut(|v| v.next_u64()) };
            let rand = mask & (rand as usize);
            if rand < bound {
                return rand;
            }
        }
    }

    pub fn sample_i32(&mut self, max: i32) -> i32 {
        self.sample(max as usize) as i32
    }

    pub fn sample_i32_inclusive(&mut self, min: i32, max: i32) -> i32 {
        min + self.sample((max - min + 1) as usize) as i32
    }

    pub fn sample_u32(&mut self, max: u32) -> u32 {
        self.sample(max as usize) as u32
    }
    //Returns the index of the sampled item.
    pub fn sample_weighted(&mut self, weights: &[u32]) -> usize {
        let total_weight: u32 = weights.into_iter().sum();
        assert!(total_weight > 0);
        let mut sample = self.sample_u32(total_weight);
        for i in 0..weights.len() {
            if sample < weights[i] {
                return i;
            } else {
                sample -= weights[i];
            }
        }
        panic!("A weight wasn't chosen!");
    }

    pub fn shuffle<T>(&mut self, v: &mut [T]) {
        for i in (1..v.len()).rev() {
            //Make sure the element can stay where it is.
            let idx = self.sample(i + 1);
            (&mut *v).swap(i, idx);
        }
    }
}

impl Hash for Rng {
    //The RNG is not part of the state to be hashed.
    fn hash<H: std::hash::Hasher>(&self, _: &mut H) {
        return;
    }
}

impl PartialEq for Rng {
    //The RNG's state is not used in comparisons.
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl Eq for Rng {}
