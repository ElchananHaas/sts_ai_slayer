#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Card {
    pub cost: Option<i32>,
    pub effect: CardEffect,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CardEffect {
    Strike,
    Bash,
    Defend,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PlayEffect {
    Attack(i32),
    DebuffEnemy(Debuff),
    Block(i32),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Debuff {
    Vulnerable(i32),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Buff {
    Strength(i32),
}

impl CardEffect {
    pub fn actions(&self) -> &'static [PlayEffect] {
        match self {
            CardEffect::Strike => &[PlayEffect::Attack(6)],
            CardEffect::Bash => &[
                PlayEffect::Attack(8),
                PlayEffect::DebuffEnemy(Debuff::Vulnerable(2)),
            ],
            CardEffect::Defend => &[PlayEffect::Block(5)],
        }
    }
    fn default_cost(&self) -> i32 {
        match self {
            CardEffect::Strike => 1,
            CardEffect::Bash => 2,
            CardEffect::Defend => 1,
        }
    }
    pub fn requires_target(&self) -> bool {
        match self {
            CardEffect::Strike | CardEffect::Bash => true,
            _ => false,
        }
    }
}
