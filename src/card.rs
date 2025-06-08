#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Card {
    pub effect: CardBody,
    pub cost: Option<i32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CardBody {
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
    Headbutt,
    HeadbuttPlus,
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
    DiscardToTop,
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
    pub upgrade_to: Option<CardBody>,
}

impl CardBody {
    pub const fn props(&self) -> &'static CardProps {
        match self {
            CardBody::Strike => &CardProps {
                actions: &[PlayEffect::Attack(6)],
                cost: Some(1),
                requires_target: true,
                card_type: CardType::Attack,
                upgrade_to: Some(CardBody::StrikePlus),
            },

            CardBody::StrikePlus => &CardProps {
                actions: &[PlayEffect::Attack(9)],
                cost: Some(1),
                requires_target: true,
                card_type: CardType::Attack,
                upgrade_to: None,
            },
            CardBody::Bash => &CardProps {
                actions: &[
                    PlayEffect::Attack(8),
                    PlayEffect::DebuffEnemy(Debuff::Vulnerable(2)),
                ],
                cost: Some(2),
                requires_target: true,
                card_type: CardType::Attack,
                upgrade_to: Some(CardBody::BashPlus),
            },
            CardBody::BashPlus => &CardProps {
                actions: &[
                    PlayEffect::Attack(10),
                    PlayEffect::DebuffEnemy(Debuff::Vulnerable(3)),
                ],
                cost: Some(2),
                requires_target: true,
                card_type: CardType::Attack,
                upgrade_to: None,
            },
            CardBody::Defend => &CardProps {
                actions: &[PlayEffect::Block(5)],
                cost: Some(1),
                requires_target: false,
                card_type: CardType::Skill,
                upgrade_to: Some(CardBody::DefendPlus),
            },
            CardBody::DefendPlus => &CardProps {
                actions: &[PlayEffect::Block(8)],
                cost: Some(1),
                requires_target: false,
                card_type: CardType::Skill,
                upgrade_to: None,
            },
            CardBody::Slimed => &CardProps {
                actions: &[],
                cost: Some(1),
                requires_target: false,
                card_type: CardType::Status,
                upgrade_to: None,
            },
            CardBody::Anger => &CardProps {
                actions: &[PlayEffect::Attack(6), PlayEffect::AddCopyToDiscard],
                cost: Some(0),
                requires_target: true,
                card_type: CardType::Attack,
                upgrade_to: Some(CardBody::AngerPlus),
            },
            CardBody::AngerPlus => &CardProps {
                actions: &[PlayEffect::Attack(8), PlayEffect::AddCopyToDiscard],
                cost: Some(0),
                requires_target: true,
                card_type: CardType::Attack,
                upgrade_to: None,
            },
            CardBody::Armaments => &CardProps {
                actions: &[
                    PlayEffect::Block(5),
                    PlayEffect::SelectCardEffect(SelectCardEffect::UpgradeCardInHand),
                ],
                cost: Some(1),
                requires_target: false,
                card_type: CardType::Skill,
                upgrade_to: Some(CardBody::ArmamentsPlus),
            },
            CardBody::ArmamentsPlus => &CardProps {
                actions: &[PlayEffect::Block(5), PlayEffect::UpgradeAllCardsInHand],
                cost: Some(1),
                requires_target: false,
                card_type: CardType::Skill,
                upgrade_to: None,
            },
            CardBody::BodySlam => &CardProps {
                actions: &[PlayEffect::AttackEqualBlock],
                cost: Some(1),
                requires_target: true,
                card_type: CardType::Attack,
                upgrade_to: Some(CardBody::BodySlamPlus),
            },
            CardBody::BodySlamPlus => &CardProps {
                actions: &[PlayEffect::AttackEqualBlock],
                cost: Some(0),
                requires_target: true,
                card_type: CardType::Attack,
                upgrade_to: None,
            },
            CardBody::Clash => &CardProps {
                actions: &[PlayEffect::Attack(14)],
                cost: Some(0),
                requires_target: true,
                card_type: CardType::Attack,
                upgrade_to: Some(CardBody::ClashPlus),
            },
            CardBody::ClashPlus => &CardProps {
                actions: &[PlayEffect::Attack(18)],
                cost: Some(0),
                requires_target: true,
                card_type: CardType::Attack,
                upgrade_to: None,
            },
            CardBody::Cleave => &CardProps {
                actions: &[PlayEffect::AttackAll(8)],
                cost: Some(1),
                requires_target: false,
                card_type: CardType::Attack,
                upgrade_to: Some(CardBody::CleavePlus),
            },
            CardBody::CleavePlus => &CardProps {
                actions: &[PlayEffect::AttackAll(11)],
                cost: Some(1),
                requires_target: false,
                card_type: CardType::Attack,
                upgrade_to: None,
            },
            CardBody::Clothesline => &CardProps {
                actions: &[
                    PlayEffect::Attack(12),
                    PlayEffect::DebuffEnemy(Debuff::Weak(2)),
                ],
                cost: Some(2),
                requires_target: true,
                card_type: CardType::Attack,
                upgrade_to: Some(CardBody::ClotheslinePlus),
            },
            CardBody::ClotheslinePlus => &CardProps {
                actions: &[
                    PlayEffect::Attack(14),
                    PlayEffect::DebuffEnemy(Debuff::Weak(3)),
                ],
                cost: Some(2),
                requires_target: true,
                card_type: CardType::Attack,
                upgrade_to: None,
            },
            CardBody::Flex => &CardProps {
                actions: &[
                    PlayEffect::Buff(Buff::Strength(2)),
                    PlayEffect::DebuffSelf(Debuff::StrengthDown(2)),
                ],
                cost: Some(0),
                requires_target: false,
                card_type: CardType::Skill,
                upgrade_to: Some(CardBody::FlexPlus),
            },
            CardBody::FlexPlus => &CardProps {
                actions: &[
                    PlayEffect::Buff(Buff::Strength(4)),
                    PlayEffect::DebuffSelf(Debuff::StrengthDown(4)),
                ],
                cost: Some(0),
                requires_target: false,
                card_type: CardType::Skill,
                upgrade_to: None,
            },
            CardBody::Havoc => &CardProps {
                actions: &[PlayEffect::PlayExhaustTop],
                cost: Some(1),
                requires_target: false,
                card_type: CardType::Skill,
                upgrade_to: Some(CardBody::HavocPlus),
            },
            CardBody::HavocPlus => &CardProps {
                actions: &[PlayEffect::PlayExhaustTop],
                cost: Some(0),
                requires_target: false,
                card_type: CardType::Skill,
                upgrade_to: None,
            },
            CardBody::Headbutt => &CardProps {
                actions: &[
                    PlayEffect::Attack(9),
                    PlayEffect::SelectCardEffect(SelectCardEffect::DiscardToTop),
                ],
                cost: Some(1),
                requires_target: true,
                card_type: CardType::Attack,
                upgrade_to: Some(CardBody::HeadbuttPlus),
            },
            CardBody::HeadbuttPlus => &CardProps {
                actions: &[
                    PlayEffect::Attack(12),
                    PlayEffect::SelectCardEffect(SelectCardEffect::DiscardToTop),
                ],
                cost: Some(1),
                requires_target: true,
                card_type: CardType::Attack,
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
    pub fn upgraded(&self) -> Option<CardBody> {
        self.props().upgrade_to
    }
}
