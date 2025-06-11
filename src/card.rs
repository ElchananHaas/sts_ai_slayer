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
    HeavyBlade,
    HeavyBladePlus,
    IronWave,
    IronWavePlus,
    SearingBlow(i32),
    PerfectedStrike,
    PerfectedStrikePlus,
    PommelStrike,
    PommelStrikePlus,
    ShrugItOff,
    ShrugItOffPlus,
    SwordBoomerang,
    SwordBoomerangPlus,
    Thunderclap,
    ThunderclapPlus,
    TrueGrit,
    TrueGritPlus,
    TwinStrike,
    TwinStrikePlus,
    Warcry,
    WarcryPlus,
    WildStrike,
    WildStrikePlus,
    Wound,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PlayEffect {
    Attack(i32),
    DebuffEnemy(Debuff),
    DebuffAll(Debuff),
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
    Draw(i32),
    AttackRandomEnemy(i32),
    ExhaustRandomInHand,
    ShuffleInStatus(CardBody),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SelectCardEffect {
    UpgradeCardInHand,
    DiscardToTop,
    ExhaustChosen,
    HandToTop,
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
}

impl CardBody {
    pub const fn props(&self) -> &'static CardProps {
        match self {
            CardBody::Strike => &CardProps {
                actions: &[PlayEffect::Attack(6)],
                cost: Some(1),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::StrikePlus => &CardProps {
                actions: &[PlayEffect::Attack(9)],
                cost: Some(1),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::Bash => &CardProps {
                actions: &[
                    PlayEffect::Attack(8),
                    PlayEffect::DebuffEnemy(Debuff::Vulnerable(2)),
                ],
                cost: Some(2),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::BashPlus => &CardProps {
                actions: &[
                    PlayEffect::Attack(10),
                    PlayEffect::DebuffEnemy(Debuff::Vulnerable(3)),
                ],
                cost: Some(2),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::Defend => &CardProps {
                actions: &[PlayEffect::Block(5)],
                cost: Some(1),
                requires_target: false,
                card_type: CardType::Skill,
            },
            CardBody::DefendPlus => &CardProps {
                actions: &[PlayEffect::Block(8)],
                cost: Some(1),
                requires_target: false,
                card_type: CardType::Skill,
            },
            CardBody::Slimed => &CardProps {
                actions: &[PlayEffect::MarkExhaust],
                cost: Some(1),
                requires_target: false,
                card_type: CardType::Status,
            },
            CardBody::Anger => &CardProps {
                actions: &[PlayEffect::Attack(6), PlayEffect::AddCopyToDiscard],
                cost: Some(0),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::AngerPlus => &CardProps {
                actions: &[PlayEffect::Attack(8), PlayEffect::AddCopyToDiscard],
                cost: Some(0),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::Armaments => &CardProps {
                actions: &[
                    PlayEffect::Block(5),
                    PlayEffect::SelectCardEffect(SelectCardEffect::UpgradeCardInHand),
                ],
                cost: Some(1),
                requires_target: false,
                card_type: CardType::Skill,
            },
            CardBody::ArmamentsPlus => &CardProps {
                actions: &[PlayEffect::Block(5), PlayEffect::UpgradeAllCardsInHand],
                cost: Some(1),
                requires_target: false,
                card_type: CardType::Skill,
            },
            CardBody::BodySlam => &CardProps {
                actions: &[PlayEffect::AttackEqualBlock],
                cost: Some(1),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::BodySlamPlus => &CardProps {
                actions: &[PlayEffect::AttackEqualBlock],
                cost: Some(0),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::Clash => &CardProps {
                actions: &[PlayEffect::Attack(14)],
                cost: Some(0),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::ClashPlus => &CardProps {
                actions: &[PlayEffect::Attack(18)],
                cost: Some(0),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::Cleave => &CardProps {
                actions: &[PlayEffect::AttackAll(8)],
                cost: Some(1),
                requires_target: false,
                card_type: CardType::Attack,
            },
            CardBody::CleavePlus => &CardProps {
                actions: &[PlayEffect::AttackAll(11)],
                cost: Some(1),
                requires_target: false,
                card_type: CardType::Attack,
            },
            CardBody::Clothesline => &CardProps {
                actions: &[
                    PlayEffect::Attack(12),
                    PlayEffect::DebuffEnemy(Debuff::Weak(2)),
                ],
                cost: Some(2),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::ClotheslinePlus => &CardProps {
                actions: &[
                    PlayEffect::Attack(14),
                    PlayEffect::DebuffEnemy(Debuff::Weak(3)),
                ],
                cost: Some(2),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::Flex => &CardProps {
                actions: &[
                    PlayEffect::Buff(Buff::Strength(2)),
                    PlayEffect::DebuffSelf(Debuff::StrengthDown(2)),
                ],
                cost: Some(0),
                requires_target: false,
                card_type: CardType::Skill,
            },
            CardBody::FlexPlus => &CardProps {
                actions: &[
                    PlayEffect::Buff(Buff::Strength(4)),
                    PlayEffect::DebuffSelf(Debuff::StrengthDown(4)),
                ],
                cost: Some(0),
                requires_target: false,
                card_type: CardType::Skill,
            },
            CardBody::Havoc => &CardProps {
                actions: &[PlayEffect::PlayExhaustTop],
                cost: Some(1),
                requires_target: false,
                card_type: CardType::Skill,
            },
            CardBody::HavocPlus => &CardProps {
                actions: &[PlayEffect::PlayExhaustTop],
                cost: Some(0),
                requires_target: false,
                card_type: CardType::Skill,
            },
            CardBody::Headbutt => &CardProps {
                actions: &[
                    PlayEffect::Attack(9),
                    PlayEffect::SelectCardEffect(SelectCardEffect::DiscardToTop),
                ],
                cost: Some(1),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::HeadbuttPlus => &CardProps {
                actions: &[
                    PlayEffect::Attack(12),
                    PlayEffect::SelectCardEffect(SelectCardEffect::DiscardToTop),
                ],
                cost: Some(1),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::HeavyBlade => &CardProps {
                actions: &[
                    PlayEffect::Attack(14), //There is special code for handling this card.
                ],
                cost: Some(2),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::HeavyBladePlus => &CardProps {
                actions: &[
                    PlayEffect::Attack(14), //There is special code for handling this card.
                ],
                cost: Some(2),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::IronWave => &CardProps {
                actions: &[PlayEffect::Block(5), PlayEffect::Attack(5)],
                cost: Some(1),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::IronWavePlus => &CardProps {
                actions: &[PlayEffect::Block(7), PlayEffect::Attack(7)],
                cost: Some(1),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::SearingBlow(_) => &CardProps {
                actions: &[PlayEffect::Attack(12)],
                cost: Some(2),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::PerfectedStrike => &CardProps {
                actions: &[PlayEffect::Attack(6)],
                cost: Some(2),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::PerfectedStrikePlus => &CardProps {
                actions: &[PlayEffect::Attack(6)],
                cost: Some(2),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::PommelStrike => &CardProps {
                actions: &[PlayEffect::Attack(9), PlayEffect::Draw(1)],
                cost: Some(1),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::PommelStrikePlus => &CardProps {
                actions: &[PlayEffect::Attack(10), PlayEffect::Draw(2)],
                cost: Some(1),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::ShrugItOff => &CardProps {
                actions: &[PlayEffect::Block(8), PlayEffect::Draw(1)],
                cost: Some(1),
                requires_target: false,
                card_type: CardType::Skill,
            },
            CardBody::ShrugItOffPlus => &CardProps {
                actions: &[PlayEffect::Block(11), PlayEffect::Draw(1)],
                cost: Some(1),
                requires_target: false,
                card_type: CardType::Skill,
            },
            CardBody::SwordBoomerang => &CardProps {
                actions: &[
                    PlayEffect::AttackRandomEnemy(3),
                    PlayEffect::AttackRandomEnemy(3),
                    PlayEffect::AttackRandomEnemy(3),
                ],
                cost: Some(1),
                requires_target: false,
                card_type: CardType::Attack,
            },
            CardBody::SwordBoomerangPlus => &CardProps {
                actions: &[
                    PlayEffect::AttackRandomEnemy(3),
                    PlayEffect::AttackRandomEnemy(3),
                    PlayEffect::AttackRandomEnemy(3),
                    PlayEffect::AttackRandomEnemy(3),
                ],
                cost: Some(1),
                requires_target: false,
                card_type: CardType::Attack,
            },
            CardBody::Thunderclap => &CardProps {
                actions: &[PlayEffect::AttackAll(4), PlayEffect::DebuffAll(Debuff::Vulnerable(1))],
                cost: Some(1),
                requires_target: false,
                card_type: CardType::Attack,
            },
            CardBody::ThunderclapPlus => &CardProps {
                actions: &[PlayEffect::AttackAll(7), PlayEffect::DebuffAll(Debuff::Vulnerable(1))],
                cost: Some(1),
                requires_target: false,
                card_type: CardType::Attack,
            },
            CardBody::TrueGrit => &CardProps {
                actions: &[PlayEffect::Block(7), PlayEffect::ExhaustRandomInHand],
                cost: Some(1),
                requires_target: false,
                card_type: CardType::Skill,
            },
            CardBody::TrueGritPlus => &CardProps {
                actions: &[
                    PlayEffect::Block(7),
                    PlayEffect::SelectCardEffect(SelectCardEffect::ExhaustChosen),
                ],
                cost: Some(1),
                requires_target: false,
                card_type: CardType::Skill,
            },
            CardBody::TwinStrike => &CardProps {
                actions: &[PlayEffect::Attack(5), PlayEffect::Attack(5)],
                cost: Some(1),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::TwinStrikePlus => &CardProps {
                actions: &[PlayEffect::Attack(7), PlayEffect::Attack(7)],
                cost: Some(1),
                requires_target: false,
                card_type: CardType::Attack,
            },
            CardBody::Warcry => &CardProps {
                actions: &[
                    PlayEffect::Draw(1),
                    PlayEffect::SelectCardEffect(SelectCardEffect::HandToTop),
                    PlayEffect::MarkExhaust,
                ],
                cost: Some(0),
                requires_target: false,
                card_type: CardType::Skill,
            },
            CardBody::WarcryPlus => &CardProps {
                actions: &[
                    PlayEffect::Draw(2),
                    PlayEffect::SelectCardEffect(SelectCardEffect::HandToTop),
                    PlayEffect::MarkExhaust,
                ],
                cost: Some(0),
                requires_target: false,
                card_type: CardType::Skill,
            },
            CardBody::WildStrike => &CardProps {
                actions: &[
                    PlayEffect::Attack(12),
                    PlayEffect::ShuffleInStatus(CardBody::Wound),
                ],
                cost: Some(1),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::WildStrikePlus => &CardProps {
                actions: &[
                    PlayEffect::Attack(17),
                    PlayEffect::ShuffleInStatus(CardBody::Wound),
                ],
                cost: Some(1),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::Wound => &CardProps {
                actions: &[PlayEffect::MarkExhaust],
                cost: None,
                requires_target: false,
                card_type: CardType::Status,
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
        match self {
            CardBody::Strike => Some(Self::StrikePlus),
            CardBody::StrikePlus => None,
            CardBody::Bash => Some(Self::BashPlus),
            CardBody::BashPlus => None,
            CardBody::Defend => Some(Self::DefendPlus),
            CardBody::DefendPlus => None,
            CardBody::Slimed => None,
            CardBody::Anger => Some(Self::AngerPlus),
            CardBody::AngerPlus => None,
            CardBody::Armaments => Some(Self::ArmamentsPlus),
            CardBody::ArmamentsPlus => None,
            CardBody::BodySlam => Some(Self::BodySlamPlus),
            CardBody::BodySlamPlus => None,
            CardBody::Clash => Some(Self::ClashPlus),
            CardBody::ClashPlus => None,
            CardBody::Cleave => Some(Self::CleavePlus),
            CardBody::CleavePlus => None,
            CardBody::Clothesline => Some(Self::ClotheslinePlus),
            CardBody::ClotheslinePlus => None,
            CardBody::Flex => Some(Self::FlexPlus),
            CardBody::FlexPlus => None,
            CardBody::Havoc => Some(Self::HavocPlus),
            CardBody::HavocPlus => None,
            CardBody::Headbutt => Some(Self::HeadbuttPlus),
            CardBody::HeadbuttPlus => None,
            CardBody::HeavyBlade => Some(Self::HeavyBladePlus),
            CardBody::HeavyBladePlus => None,
            CardBody::IronWave => Some(Self::IronWavePlus),
            CardBody::IronWavePlus => None,
            CardBody::SearingBlow(level) => Some(Self::SearingBlow(*level + 1)),
            CardBody::PerfectedStrike => Some(Self::PerfectedStrikePlus),
            CardBody::PerfectedStrikePlus => None,
            CardBody::PommelStrike => Some(Self::PommelStrikePlus),
            CardBody::PommelStrikePlus => None,
            CardBody::ShrugItOff => Some(Self::ShrugItOffPlus),
            CardBody::ShrugItOffPlus => None,
            CardBody::SwordBoomerang => Some(Self::SwordBoomerangPlus),
            CardBody::SwordBoomerangPlus => None,
            CardBody::Thunderclap => Some(Self::ThunderclapPlus),
            CardBody::ThunderclapPlus => None,
            CardBody::TrueGrit => Some(Self::TrueGritPlus),
            CardBody::TrueGritPlus => None,
            CardBody::TwinStrike => Some(Self::TwinStrikePlus),
            CardBody::TwinStrikePlus => None,
            CardBody::Warcry => Some(Self::WarcryPlus),
            CardBody::WarcryPlus => None,
            CardBody::WildStrike => Some(CardBody::WildStrikePlus),
            CardBody::WildStrikePlus => None,
            CardBody::Wound => None,
        }
    }
    pub fn is_strike(&self) -> bool {
        match self {
            CardBody::Strike
            | CardBody::StrikePlus
            | CardBody::PerfectedStrike
            | CardBody::PerfectedStrikePlus
            | CardBody::PommelStrike
            | CardBody::PommelStrikePlus
            | CardBody::TwinStrike
            | CardBody::TwinStrikePlus
            | CardBody::WildStrike
            | CardBody::WildStrikePlus => true,
            _ => false,
        }
    }
}
