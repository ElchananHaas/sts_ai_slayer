use crate::rng::Rng;

pub enum Relic {
    Placeholder1,
    Placeholder2,
    Placeholder3,
    Placeholder4,
    Placeholder5,
    Placeholder6,
    Placeholder7,
    Placeholder8,
    Placeholder9,
    GoldenIdol,
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RelicPool {}

impl RelicPool {
    pub fn new() -> Self {
        RelicPool {}
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Relics {}

impl Relics {
    pub fn new() -> Self {
        Relics {}
    }
    pub fn add(&mut self, relic: Relic) {
        todo!("Implement adding relics")
    }
}

pub enum RelicRarity {
    Common,
    Uncommon,
    Rare,
    Shop,
    Event,
}
impl RelicPool {
    pub fn get_relic(&mut self, rarity: RelicRarity) -> Relic {
        return Relic::Placeholder1;
    }
    pub fn get_random_tier_relic(&mut self, rng: &mut Rng) -> Relic {
        let sample = rng.sample_weighted(&[50, 33, 17]);
        let rarity = match sample {
            0 => RelicRarity::Common,
            1 => RelicRarity::Uncommon,
            2 => RelicRarity::Rare,
            _ => panic!("Invalid rarity returned by RNG"),
        };
        self.get_relic(rarity)
    }
}
