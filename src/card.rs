#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
    Anger,
    Armaments,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PlayEffect {
    Attack(i32),
    DebuffEnemy(Debuff),
    Block(i32),
    AddCopyToDiscard,
    UpgradeCardInHand,
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

pub struct CardProps {
    pub actions: &'static [PlayEffect],
    pub cost: Option<i32>,
    pub requires_target: bool,
    pub card_type: CardType,
    //pub upgrade_to: CardEffect,
}

impl CardEffect {
    pub const fn props(&self) -> &'static CardProps {
        match self {
            CardEffect::Strike => &CardProps {
                actions: &[PlayEffect::Attack(6)],
                cost: Some(1),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardEffect::Bash => &CardProps {
                actions: &[
                    PlayEffect::Attack(8),
                    PlayEffect::DebuffEnemy(Debuff::Vulnerable(2)),
                ],
                cost: Some(2),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardEffect::Defend => &CardProps {
                actions: &[PlayEffect::Block(5)],
                cost: Some(1),
                requires_target: false,
                card_type: CardType::Skill,
            },
            CardEffect::Slimed => &CardProps {
                actions: &[],
                cost: Some(1),
                requires_target: false,
                card_type: CardType::Status,
            },
            CardEffect::Anger => &CardProps {
                actions: &[PlayEffect::Attack(6), PlayEffect::AddCopyToDiscard],
                cost: Some(0),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardEffect::Armaments => &CardProps {
                actions: &[PlayEffect::Block(5), PlayEffect::UpgradeCardInHand],
                cost: Some(1),
                requires_target: false,
                card_type: CardType::Skill,
            },
        }
    }
    pub const fn to_card(&self) -> Card {
        Card {
            cost: self.default_cost(),
            effect: *self,
        }
    }
    pub fn actions(&self) -> &'static [PlayEffect] {
        self.props().actions
    }
    const fn default_cost(&self) -> Option<i32> {
        self.props().cost
    }
    pub fn requires_target(&self) -> bool {
        self.props().requires_target
    }
    pub fn card_type(&self) -> CardType {
        self.props().card_type
    }
}
