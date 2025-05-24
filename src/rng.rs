use std::hash::Hash;
use std::random::DefaultRandomSource;
use std::random::Random;

#[derive(Debug, Clone)]
pub struct Rng {
    source: DefaultRandomSource,
}

impl Rng {
    pub fn new() -> Self {
        Self {
            source: DefaultRandomSource,
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
            let rand = mask & usize::random(&mut self.source);
            if rand < max {
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
