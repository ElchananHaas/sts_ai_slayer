#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Card {
    pub effect: CardEffect,
    pub cost: Option<i32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CardEffect {
    Strike,
    Bash,
    Defend,
    Slimed,
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
    Weak(i32),
    Frail(i32),
    Entangled,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Buff {
    Strength(i32),
    Ritual(i32),
    RitualSkipFirst(i32),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CardType {
    Attack,
    Skill,
    Power,
    Status,
    Curse,
}

impl CardEffect {
    pub const fn to_card(&self) -> Card {
        Card {
            cost: self.default_cost(),
            effect: *self,
        }
    }
    pub fn actions(&self) -> &'static [PlayEffect] {
        match self {
            CardEffect::Strike => &[PlayEffect::Attack(6)],
            CardEffect::Bash => &[
                PlayEffect::Attack(8),
                PlayEffect::DebuffEnemy(Debuff::Vulnerable(2)),
            ],
            CardEffect::Defend => &[PlayEffect::Block(5)],
            CardEffect::Slimed => todo!(),
        }
    }
    const fn default_cost(&self) -> Option<i32> {
        match self {
            CardEffect::Strike => Some(1),
            CardEffect::Bash => Some(2),
            CardEffect::Defend => Some(1),
            CardEffect::Slimed => Some(1),
        }
    }
    pub fn requires_target(&self) -> bool {
        match self {
            CardEffect::Strike | CardEffect::Bash => true,
            _ => false,
        }
    }
    pub fn card_type(&self) -> CardType {
        match self {
            CardEffect::Strike => CardType::Attack,
            CardEffect::Bash => CardType::Attack,
            CardEffect::Defend => CardType::Skill,
            CardEffect::Slimed => CardType::Status,
        }
    }
}
