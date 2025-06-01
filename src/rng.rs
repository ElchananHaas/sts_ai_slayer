use std::cell::RefCell;
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;

use rand_chacha::rand_core::RngCore;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha8Rng;

pub struct Rng {
    //This RNG is not seedable. So it is fine to clone it. Clones should share an 
    //RNG state to ensure that in MCTS distinct numbers are generated.
    source: Rc<RefCell<ChaCha8Rng>>,
}

impl Debug for Rng {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Rng").field("source", &"[Redacted]").finish()
    }
}

impl Clone for Rng {
    fn clone(&self) -> Self {
        Self { source: Rc::clone(&self.source)}
    }
}

impl Rng {
    pub fn new() -> Self {
        Self {
            source:Rc::new(RefCell::new(ChaCha8Rng::from_os_rng())),
        }
    }
    //The samples is exclusive on max. It utilizes rejection sampling.
    pub fn sample(&mut self, max: usize) -> usize {
        if max == 0 {
            panic!("Invalid range: Max cannot be 0");
        }
        let next_pow_2 = max.next_power_of_two();
        let mask = next_pow_2 - 1;
        loop {
            //self.source.next_u64() as usize
            //usize::random(&mut DefaultRandomSource)
            let rand ={self.source.borrow_mut().next_u64()};
            //let rand = u64::random(&mut DefaultRandomSource);
            //let rand = getrandom::u64().expect("RNG call successful");
            //dbg!(rand);
            let rand = mask & (rand as usize);
            if rand < max {
                //dbg!((rand, max));
                return rand;
            }
        }
    }

    pub fn sample_i32(&mut self, max: i32) -> i32 {
        self.sample(max as usize) as i32
    }

    pub fn sample_u32(&mut self, max: u32) -> u32 {
        self.sample(max as usize) as u32
    }
    //Returns the index of the sampled item.
    pub fn sample_weighted(&mut self, weights: &'static [u32]) -> usize {
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
