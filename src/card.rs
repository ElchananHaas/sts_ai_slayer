#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Card {
    pub effect: CardEffect,
    pub cost: Option<i32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CardEffect {
    Strike,
    StrikePlus,
    Bash,
    BashPlus,
    Defend,
    DefendPlus,
    Slimed,
    Anger,
    AngerPlus,
    Armaments,
    ArmamentsPlus,
    BodySlam,
    BodySlamPlus,
    Clash,
    ClashPlus,
    Cleave,
    CleavePlus,
    Clothesline,
    ClotheslinePlus,
    Flex,
    FlexPlus,
    Havoc,
    HavocPlus,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PlayEffect {
    Attack(i32),
    DebuffEnemy(Debuff),
    Block(i32),
    AddCopyToDiscard,
    SelectCardEffect(SelectCardEffect),
    UpgradeAllCardsInHand,
    AttackEqualBlock, //Used for Body Slam
    AttackAll(i32),
    Buff(Buff),
    DebuffSelf(Debuff),
    PlayExhaustTop, //Used for Havoc.
    MarkExhaust,    //This is used for marking that a card exhausts itself.
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SelectCardEffect {
    UpgradeCardInHand,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Debuff {
    Vulnerable(i32),
    Weak(i32),
    Frail(i32),
    StrengthDown(i32),
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
    pub upgrade_to: Option<CardEffect>,
}

impl CardEffect {
    pub const fn props(&self) -> &'static CardProps {
        match self {
            CardEffect::Strike => &CardProps {
                actions: &[PlayEffect::Attack(6)],
                cost: Some(1),
                requires_target: true,
                card_type: CardType::Attack,
                upgrade_to: Some(CardEffect::StrikePlus),
            },

            CardEffect::StrikePlus => &CardProps {
                actions: &[PlayEffect::Attack(9)],
                cost: Some(1),
                requires_target: true,
                card_type: CardType::Attack,
                upgrade_to: None,
            },
            CardEffect::Bash => &CardProps {
                actions: &[
                    PlayEffect::Attack(8),
                    PlayEffect::DebuffEnemy(Debuff::Vulnerable(2)),
                ],
                cost: Some(2),
                requires_target: true,
                card_type: CardType::Attack,
                upgrade_to: Some(CardEffect::BashPlus),
            },
            CardEffect::BashPlus => &CardProps {
                actions: &[
                    PlayEffect::Attack(10),
                    PlayEffect::DebuffEnemy(Debuff::Vulnerable(3)),
                ],
                cost: Some(2),
                requires_target: true,
                card_type: CardType::Attack,
                upgrade_to: None,
            },
            CardEffect::Defend => &CardProps {
                actions: &[PlayEffect::Block(5)],
                cost: Some(1),
                requires_target: false,
                card_type: CardType::Skill,
                upgrade_to: Some(CardEffect::DefendPlus),
            },
            CardEffect::DefendPlus => &CardProps {
                actions: &[PlayEffect::Block(8)],
                cost: Some(1),
                requires_target: false,
                card_type: CardType::Skill,
                upgrade_to: None,
            },
            CardEffect::Slimed => &CardProps {
                actions: &[],
                cost: Some(1),
                requires_target: false,
                card_type: CardType::Status,
                upgrade_to: None,
            },
            CardEffect::Anger => &CardProps {
                actions: &[PlayEffect::Attack(6), PlayEffect::AddCopyToDiscard],
                cost: Some(0),
                requires_target: true,
                card_type: CardType::Attack,
                upgrade_to: Some(CardEffect::AngerPlus),
            },
            CardEffect::AngerPlus => &CardProps {
                actions: &[PlayEffect::Attack(8), PlayEffect::AddCopyToDiscard],
                cost: Some(0),
                requires_target: true,
                card_type: CardType::Attack,
                upgrade_to: None,
            },
            CardEffect::Armaments => &CardProps {
                actions: &[
                    PlayEffect::Block(5),
                    PlayEffect::SelectCardEffect(SelectCardEffect::UpgradeCardInHand),
                ],
                cost: Some(1),
                requires_target: false,
                card_type: CardType::Skill,
                upgrade_to: Some(CardEffect::ArmamentsPlus),
            },
            CardEffect::ArmamentsPlus => &CardProps {
                actions: &[PlayEffect::Block(5), PlayEffect::UpgradeAllCardsInHand],
                cost: Some(1),
                requires_target: false,
                card_type: CardType::Skill,
                upgrade_to: None,
            },
            CardEffect::BodySlam => &CardProps {
                actions: &[PlayEffect::AttackEqualBlock],
                cost: Some(1),
                requires_target: true,
                card_type: CardType::Attack,
                upgrade_to: Some(CardEffect::BodySlamPlus),
            },
            CardEffect::BodySlamPlus => &CardProps {
                actions: &[PlayEffect::AttackEqualBlock],
                cost: Some(0),
                requires_target: true,
                card_type: CardType::Attack,
                upgrade_to: None,
            },
            CardEffect::Clash => &CardProps {
                actions: &[PlayEffect::Attack(14)],
                cost: Some(0),
                requires_target: true,
                card_type: CardType::Attack,
                upgrade_to: Some(CardEffect::ClashPlus),
            },
            CardEffect::ClashPlus => &CardProps {
                actions: &[PlayEffect::Attack(18)],
                cost: Some(0),
                requires_target: true,
                card_type: CardType::Attack,
                upgrade_to: None,
            },
            CardEffect::Cleave => &CardProps {
                actions: &[PlayEffect::AttackAll(8)],
                cost: Some(1),
                requires_target: false,
                card_type: CardType::Attack,
                upgrade_to: Some(CardEffect::CleavePlus),
            },
            CardEffect::CleavePlus => &CardProps {
                actions: &[PlayEffect::AttackAll(11)],
                cost: Some(1),
                requires_target: false,
                card_type: CardType::Attack,
                upgrade_to: None,
            },
            CardEffect::Clothesline => &CardProps {
                actions: &[
                    PlayEffect::Attack(12),
                    PlayEffect::DebuffEnemy(Debuff::Weak(2)),
                ],
                cost: Some(2),
                requires_target: true,
                card_type: CardType::Attack,
                upgrade_to: Some(CardEffect::ClotheslinePlus),
            },
            CardEffect::ClotheslinePlus => &CardProps {
                actions: &[
                    PlayEffect::Attack(14),
                    PlayEffect::DebuffEnemy(Debuff::Weak(3)),
                ],
                cost: Some(2),
                requires_target: true,
                card_type: CardType::Attack,
                upgrade_to: None,
            },
            CardEffect::Flex => &CardProps {
                actions: &[
                    PlayEffect::Buff(Buff::Strength(2)),
                    PlayEffect::DebuffSelf(Debuff::StrengthDown(2)),
                ],
                cost: Some(0),
                requires_target: false,
                card_type: CardType::Skill,
                upgrade_to: Some(CardEffect::FlexPlus),
            },
            CardEffect::FlexPlus => &CardProps {
                actions: &[
                    PlayEffect::Buff(Buff::Strength(4)),
                    PlayEffect::DebuffSelf(Debuff::StrengthDown(4)),
                ],
                cost: Some(0),
                requires_target: false,
                card_type: CardType::Skill,
                upgrade_to: None,
            },
            CardEffect::Havoc => &CardProps {
                actions: &[PlayEffect::PlayExhaustTop],
                cost: Some(1),
                requires_target: false,
                card_type: CardType::Skill,
                upgrade_to: Some(CardEffect::HavocPlus),
            },
            CardEffect::HavocPlus => &CardProps {
                actions: &[PlayEffect::PlayExhaustTop],
                cost: Some(1),
                requires_target: false,
                card_type: CardType::Skill,
                upgrade_to: None,
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
    pub const fn default_cost(&self) -> Option<i32> {
        self.props().cost
    }
    pub fn requires_target(&self) -> bool {
        self.props().requires_target
    }
    pub fn card_type(&self) -> CardType {
        self.props().card_type
    }
    pub fn upgraded(&self) -> Option<CardEffect> {
        self.props().upgrade_to
    }
}
