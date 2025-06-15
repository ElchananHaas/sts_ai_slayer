#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Card {
    pub effect: CardBody,
    pub cost: Cost,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Cost {
    Unplayable,
    Fixed(i32),
    X,
    NumMinusHpLoss(i32), //This is for Blood for Blood
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
    BattleTrance,
    BattleTrancePlus,
    BloodForBlood,
    BloodForBloodPlus,
    Bloodletting,
    BloodlettingPlus,
    BurningPact,
    BurningPactPlus,
    SearingBlow(i32),
    Carnage,
    CarnagePlus,
    Combust,
    CombustPlus,
    DarkEmbrace,
    DarkEmbracePlus,
    Disarm,
    DisarmPlus,
    Dropkick,
    DropkickPlus,
    DualWield,
    DualWieldPlus,
    Entrench,
    EntrenchPlus,
    Evolve,
    EvolvePlus,
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
    LoseHP(i32),
    GainEnergy(i32),
    DropkickDraw,
    DoubleBlock,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SelectCardEffect {
    UpgradeCardInHand,
    DiscardToTop,
    ExhaustChosen,
    HandToTop,
    DuplicatePowerOrAttack(i32),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Debuff {
    Vulnerable(i32),
    Weak(i32),
    Frail(i32),
    StrengthDown(i32),
    Entangled,
    NoDraw,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Buff {
    Strength(i32),
    Ritual(i32),
    RitualSkipFirst(i32),
    EndTurnLoseHP(i32),
    EndTurnDamageAllEnemies(i32),
    DarkEmbraceBuff,
    EvolveBuff(i32)
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
    pub cost: Cost,
    pub requires_target: bool,
    pub card_type: CardType,
}

impl CardBody {
    pub const fn props(&self) -> &'static CardProps {
        match self {
            CardBody::Strike => &CardProps {
                actions: &[PlayEffect::Attack(6)],
                cost: Cost::Fixed(1),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::StrikePlus => &CardProps {
                actions: &[PlayEffect::Attack(9)],
                cost: Cost::Fixed(1),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::Bash => &CardProps {
                actions: &[
                    PlayEffect::Attack(8),
                    PlayEffect::DebuffEnemy(Debuff::Vulnerable(2)),
                ],
                cost: Cost::Fixed(2),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::BashPlus => &CardProps {
                actions: &[
                    PlayEffect::Attack(10),
                    PlayEffect::DebuffEnemy(Debuff::Vulnerable(3)),
                ],
                cost: Cost::Fixed(2),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::Defend => &CardProps {
                actions: &[PlayEffect::Block(5)],
                cost: Cost::Fixed(1),
                requires_target: false,
                card_type: CardType::Skill,
            },
            CardBody::DefendPlus => &CardProps {
                actions: &[PlayEffect::Block(8)],
                cost: Cost::Fixed(1),
                requires_target: false,
                card_type: CardType::Skill,
            },
            CardBody::Slimed => &CardProps {
                actions: &[PlayEffect::MarkExhaust],
                cost: Cost::Fixed(1),
                requires_target: false,
                card_type: CardType::Status,
            },
            CardBody::Anger => &CardProps {
                actions: &[PlayEffect::Attack(6), PlayEffect::AddCopyToDiscard],
                cost: Cost::Fixed(0),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::AngerPlus => &CardProps {
                actions: &[PlayEffect::Attack(8), PlayEffect::AddCopyToDiscard],
                cost: Cost::Fixed(0),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::Armaments => &CardProps {
                actions: &[
                    PlayEffect::Block(5),
                    PlayEffect::SelectCardEffect(SelectCardEffect::UpgradeCardInHand),
                ],
                cost: Cost::Fixed(1),
                requires_target: false,
                card_type: CardType::Skill,
            },
            CardBody::ArmamentsPlus => &CardProps {
                actions: &[PlayEffect::Block(5), PlayEffect::UpgradeAllCardsInHand],
                cost: Cost::Fixed(1),
                requires_target: false,
                card_type: CardType::Skill,
            },
            CardBody::BodySlam => &CardProps {
                actions: &[PlayEffect::AttackEqualBlock],
                cost: Cost::Fixed(1),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::BodySlamPlus => &CardProps {
                actions: &[PlayEffect::AttackEqualBlock],
                cost: Cost::Fixed(0),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::Clash => &CardProps {
                actions: &[PlayEffect::Attack(14)],
                cost: Cost::Fixed(0),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::ClashPlus => &CardProps {
                actions: &[PlayEffect::Attack(18)],
                cost: Cost::Fixed(0),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::Cleave => &CardProps {
                actions: &[PlayEffect::AttackAll(8)],
                cost: Cost::Fixed(1),
                requires_target: false,
                card_type: CardType::Attack,
            },
            CardBody::CleavePlus => &CardProps {
                actions: &[PlayEffect::AttackAll(11)],
                cost: Cost::Fixed(1),
                requires_target: false,
                card_type: CardType::Attack,
            },
            CardBody::Clothesline => &CardProps {
                actions: &[
                    PlayEffect::Attack(12),
                    PlayEffect::DebuffEnemy(Debuff::Weak(2)),
                ],
                cost: Cost::Fixed(2),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::ClotheslinePlus => &CardProps {
                actions: &[
                    PlayEffect::Attack(14),
                    PlayEffect::DebuffEnemy(Debuff::Weak(3)),
                ],
                cost: Cost::Fixed(2),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::Flex => &CardProps {
                actions: &[
                    PlayEffect::Buff(Buff::Strength(2)),
                    PlayEffect::DebuffSelf(Debuff::StrengthDown(2)),
                ],
                cost: Cost::Fixed(0),
                requires_target: false,
                card_type: CardType::Skill,
            },
            CardBody::FlexPlus => &CardProps {
                actions: &[
                    PlayEffect::Buff(Buff::Strength(4)),
                    PlayEffect::DebuffSelf(Debuff::StrengthDown(4)),
                ],
                cost: Cost::Fixed(0),
                requires_target: false,
                card_type: CardType::Skill,
            },
            CardBody::Havoc => &CardProps {
                actions: &[PlayEffect::PlayExhaustTop],
                cost: Cost::Fixed(1),
                requires_target: false,
                card_type: CardType::Skill,
            },
            CardBody::HavocPlus => &CardProps {
                actions: &[PlayEffect::PlayExhaustTop],
                cost: Cost::Fixed(0),
                requires_target: false,
                card_type: CardType::Skill,
            },
            CardBody::Headbutt => &CardProps {
                actions: &[
                    PlayEffect::Attack(9),
                    PlayEffect::SelectCardEffect(SelectCardEffect::DiscardToTop),
                ],
                cost: Cost::Fixed(1),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::HeadbuttPlus => &CardProps {
                actions: &[
                    PlayEffect::Attack(12),
                    PlayEffect::SelectCardEffect(SelectCardEffect::DiscardToTop),
                ],
                cost: Cost::Fixed(1),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::HeavyBlade => &CardProps {
                actions: &[
                    PlayEffect::Attack(14), //There is special code for handling this card.
                ],
                cost: Cost::Fixed(2),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::HeavyBladePlus => &CardProps {
                actions: &[
                    PlayEffect::Attack(14), //There is special code for handling this card.
                ],
                cost: Cost::Fixed(2),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::IronWave => &CardProps {
                actions: &[PlayEffect::Block(5), PlayEffect::Attack(5)],
                cost: Cost::Fixed(1),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::IronWavePlus => &CardProps {
                actions: &[PlayEffect::Block(7), PlayEffect::Attack(7)],
                cost: Cost::Fixed(1),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::SearingBlow(_) => &CardProps {
                actions: &[PlayEffect::Attack(12)],
                cost: Cost::Fixed(2),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::PerfectedStrike => &CardProps {
                actions: &[PlayEffect::Attack(6)],
                cost: Cost::Fixed(2),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::PerfectedStrikePlus => &CardProps {
                actions: &[PlayEffect::Attack(6)],
                cost: Cost::Fixed(2),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::PommelStrike => &CardProps {
                actions: &[PlayEffect::Attack(9), PlayEffect::Draw(1)],
                cost: Cost::Fixed(1),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::PommelStrikePlus => &CardProps {
                actions: &[PlayEffect::Attack(10), PlayEffect::Draw(2)],
                cost: Cost::Fixed(1),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::ShrugItOff => &CardProps {
                actions: &[PlayEffect::Block(8), PlayEffect::Draw(1)],
                cost: Cost::Fixed(1),
                requires_target: false,
                card_type: CardType::Skill,
            },
            CardBody::ShrugItOffPlus => &CardProps {
                actions: &[PlayEffect::Block(11), PlayEffect::Draw(1)],
                cost: Cost::Fixed(1),
                requires_target: false,
                card_type: CardType::Skill,
            },
            CardBody::SwordBoomerang => &CardProps {
                actions: &[
                    PlayEffect::AttackRandomEnemy(3),
                    PlayEffect::AttackRandomEnemy(3),
                    PlayEffect::AttackRandomEnemy(3),
                ],
                cost: Cost::Fixed(1),
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
                cost: Cost::Fixed(1),
                requires_target: false,
                card_type: CardType::Attack,
            },
            CardBody::Thunderclap => &CardProps {
                actions: &[
                    PlayEffect::AttackAll(4),
                    PlayEffect::DebuffAll(Debuff::Vulnerable(1)),
                ],
                cost: Cost::Fixed(1),
                requires_target: false,
                card_type: CardType::Attack,
            },
            CardBody::ThunderclapPlus => &CardProps {
                actions: &[
                    PlayEffect::AttackAll(7),
                    PlayEffect::DebuffAll(Debuff::Vulnerable(1)),
                ],
                cost: Cost::Fixed(1),
                requires_target: false,
                card_type: CardType::Attack,
            },
            CardBody::TrueGrit => &CardProps {
                actions: &[PlayEffect::Block(7), PlayEffect::ExhaustRandomInHand],
                cost: Cost::Fixed(1),
                requires_target: false,
                card_type: CardType::Skill,
            },
            CardBody::TrueGritPlus => &CardProps {
                actions: &[
                    PlayEffect::Block(7),
                    PlayEffect::SelectCardEffect(SelectCardEffect::ExhaustChosen),
                ],
                cost: Cost::Fixed(1),
                requires_target: false,
                card_type: CardType::Skill,
            },
            CardBody::TwinStrike => &CardProps {
                actions: &[PlayEffect::Attack(5), PlayEffect::Attack(5)],
                cost: Cost::Fixed(1),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::TwinStrikePlus => &CardProps {
                actions: &[PlayEffect::Attack(7), PlayEffect::Attack(7)],
                cost: Cost::Fixed(1),
                requires_target: false,
                card_type: CardType::Attack,
            },
            CardBody::Warcry => &CardProps {
                actions: &[
                    PlayEffect::Draw(1),
                    PlayEffect::SelectCardEffect(SelectCardEffect::HandToTop),
                    PlayEffect::MarkExhaust,
                ],
                cost: Cost::Fixed(0),
                requires_target: false,
                card_type: CardType::Skill,
            },
            CardBody::WarcryPlus => &CardProps {
                actions: &[
                    PlayEffect::Draw(2),
                    PlayEffect::SelectCardEffect(SelectCardEffect::HandToTop),
                    PlayEffect::MarkExhaust,
                ],
                cost: Cost::Fixed(0),
                requires_target: false,
                card_type: CardType::Skill,
            },
            CardBody::WildStrike => &CardProps {
                actions: &[
                    PlayEffect::Attack(12),
                    PlayEffect::ShuffleInStatus(CardBody::Wound),
                ],
                cost: Cost::Fixed(1),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::WildStrikePlus => &CardProps {
                actions: &[
                    PlayEffect::Attack(17),
                    PlayEffect::ShuffleInStatus(CardBody::Wound),
                ],
                cost: Cost::Fixed(1),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::Wound => &CardProps {
                actions: &[PlayEffect::MarkExhaust],
                cost: Cost::Unplayable,
                requires_target: false,
                card_type: CardType::Status,
            },
            CardBody::BattleTrance => &CardProps {
                actions: &[PlayEffect::Draw(3), PlayEffect::DebuffSelf(Debuff::NoDraw)],
                cost: Cost::Fixed(0),
                requires_target: false,
                card_type: CardType::Skill,
            },
            CardBody::BattleTrancePlus => &CardProps {
                actions: &[PlayEffect::Draw(4), PlayEffect::DebuffSelf(Debuff::NoDraw)],
                cost: Cost::Fixed(0),
                requires_target: false,
                card_type: CardType::Skill,
            },
            CardBody::BloodForBlood => &CardProps {
                actions: &[PlayEffect::Attack(18)],
                cost: Cost::NumMinusHpLoss(4),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::BloodForBloodPlus => &CardProps {
                actions: &[PlayEffect::Attack(22)],
                cost: Cost::NumMinusHpLoss(3),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::Bloodletting => &CardProps {
                actions: &[PlayEffect::LoseHP(3), PlayEffect::GainEnergy(2)],
                cost: Cost::Fixed(0),
                requires_target: false,
                card_type: CardType::Skill,
            },
            CardBody::BloodlettingPlus => &CardProps {
                actions: &[PlayEffect::LoseHP(3), PlayEffect::GainEnergy(3)],
                cost: Cost::Fixed(0),
                requires_target: false,
                card_type: CardType::Skill,
            },
            CardBody::BurningPact => &CardProps {
                actions: &[
                    PlayEffect::SelectCardEffect(SelectCardEffect::ExhaustChosen),
                    PlayEffect::Draw(2),
                ],
                cost: Cost::Fixed(1),
                requires_target: false,
                card_type: CardType::Skill,
            },
            CardBody::BurningPactPlus => &CardProps {
                actions: &[
                    PlayEffect::SelectCardEffect(SelectCardEffect::ExhaustChosen),
                    PlayEffect::Draw(3),
                ],
                cost: Cost::Fixed(1),
                requires_target: false,
                card_type: CardType::Skill,
            },
            CardBody::Carnage => &CardProps {
                actions: &[PlayEffect::Attack(20)],
                cost: Cost::Fixed(2),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::CarnagePlus => &CardProps {
                actions: &[PlayEffect::Attack(28)],
                cost: Cost::Fixed(2),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::Combust => &CardProps {
                actions: &[
                    PlayEffect::Buff(Buff::EndTurnLoseHP(1)),
                    PlayEffect::Buff(Buff::EndTurnDamageAllEnemies(5)),
                ],
                cost: Cost::Fixed(1),
                requires_target: false,
                card_type: CardType::Power,
            },
            CardBody::CombustPlus => &CardProps {
                actions: &[
                    PlayEffect::Buff(Buff::EndTurnLoseHP(1)),
                    PlayEffect::Buff(Buff::EndTurnDamageAllEnemies(7)),
                ],
                cost: Cost::Fixed(1),
                requires_target: false,
                card_type: CardType::Power,
            },
            CardBody::DarkEmbrace => &CardProps {
                actions: &[PlayEffect::Buff(Buff::DarkEmbraceBuff)],
                cost: Cost::Fixed(2),
                requires_target: false,
                card_type: CardType::Power,
            },
            CardBody::DarkEmbracePlus => &CardProps {
                actions: &[PlayEffect::Buff(Buff::DarkEmbraceBuff)],
                cost: Cost::Fixed(1),
                requires_target: false,
                card_type: CardType::Power,
            },
            CardBody::Disarm => &CardProps {
                actions: &[
                    PlayEffect::DebuffEnemy(Debuff::StrengthDown(2)),
                    PlayEffect::MarkExhaust,
                ],
                cost: Cost::Fixed(1),
                requires_target: true,
                card_type: CardType::Skill,
            },
            CardBody::DisarmPlus => &CardProps {
                actions: &[
                    PlayEffect::DebuffEnemy(Debuff::StrengthDown(3)),
                    PlayEffect::MarkExhaust,
                ],
                cost: Cost::Fixed(1),
                requires_target: true,
                card_type: CardType::Skill,
            },
            CardBody::Dropkick => &CardProps {
                actions: &[PlayEffect::Attack(5), PlayEffect::DropkickDraw],
                cost: Cost::Fixed(1),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::DropkickPlus => &CardProps {
                actions: &[PlayEffect::Attack(5), PlayEffect::DropkickDraw],
                cost: Cost::Fixed(1),
                requires_target: true,
                card_type: CardType::Attack,
            },
            CardBody::DualWield => &CardProps {
                actions: &[PlayEffect::SelectCardEffect(
                    SelectCardEffect::DuplicatePowerOrAttack(1),
                )],
                cost: Cost::Fixed(1),
                requires_target: false,
                card_type: CardType::Skill,
            },
            CardBody::DualWieldPlus => &CardProps {
                actions: &[PlayEffect::SelectCardEffect(
                    SelectCardEffect::DuplicatePowerOrAttack(2),
                )],
                cost: Cost::Fixed(1),
                requires_target: false,
                card_type: CardType::Skill,
            },
            CardBody::Entrench => &CardProps {
                actions: &[PlayEffect::DoubleBlock],
                cost: Cost::Fixed(2),
                requires_target: false,
                card_type: CardType::Skill,
            },
            CardBody::EntrenchPlus => &CardProps {
                actions: &[PlayEffect::DoubleBlock],
                cost: Cost::Fixed(1),
                requires_target: false,
                card_type: CardType::Skill,
            },
            CardBody::Evolve => &CardProps {
                actions: &[PlayEffect::Buff(Buff::EvolveBuff(1))],
                cost: Cost::Fixed(1),
                requires_target: false,
                card_type: CardType::Power,
            },
            CardBody::EvolvePlus => &CardProps {
                actions: &[PlayEffect::Buff(Buff::EvolveBuff(2))],
                cost: Cost::Fixed(1),
                requires_target: false,
                card_type: CardType::Power,
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
    pub const fn default_cost(&self) -> Cost {
        self.props().cost
    }
    pub fn requires_target(&self) -> bool {
        self.props().requires_target
    }
    pub fn card_type(&self) -> CardType {
        self.props().card_type
    }
    pub fn ethereal(&self) -> bool {
        match self {
            Self::Carnage | Self::CarnagePlus => true,
            _ => false,
        }
    }
    pub fn upgraded(&self) -> Option<Self> {
        match self {
            Self::Strike => Some(Self::StrikePlus),
            Self::StrikePlus => None,
            Self::Bash => Some(Self::BashPlus),
            Self::BashPlus => None,
            Self::Defend => Some(Self::DefendPlus),
            Self::DefendPlus => None,
            Self::Slimed => None,
            Self::Anger => Some(Self::AngerPlus),
            Self::AngerPlus => None,
            Self::Armaments => Some(Self::ArmamentsPlus),
            Self::ArmamentsPlus => None,
            Self::BodySlam => Some(Self::BodySlamPlus),
            Self::BodySlamPlus => None,
            Self::Clash => Some(Self::ClashPlus),
            Self::ClashPlus => None,
            Self::Cleave => Some(Self::CleavePlus),
            Self::CleavePlus => None,
            Self::Clothesline => Some(Self::ClotheslinePlus),
            Self::ClotheslinePlus => None,
            Self::Flex => Some(Self::FlexPlus),
            Self::FlexPlus => None,
            Self::Havoc => Some(Self::HavocPlus),
            Self::HavocPlus => None,
            Self::Headbutt => Some(Self::HeadbuttPlus),
            Self::HeadbuttPlus => None,
            Self::HeavyBlade => Some(Self::HeavyBladePlus),
            Self::HeavyBladePlus => None,
            Self::IronWave => Some(Self::IronWavePlus),
            Self::IronWavePlus => None,
            Self::SearingBlow(level) => Some(Self::SearingBlow(*level + 1)),
            Self::PerfectedStrike => Some(Self::PerfectedStrikePlus),
            Self::PerfectedStrikePlus => None,
            Self::PommelStrike => Some(Self::PommelStrikePlus),
            Self::PommelStrikePlus => None,
            Self::ShrugItOff => Some(Self::ShrugItOffPlus),
            Self::ShrugItOffPlus => None,
            Self::SwordBoomerang => Some(Self::SwordBoomerangPlus),
            Self::SwordBoomerangPlus => None,
            Self::Thunderclap => Some(Self::ThunderclapPlus),
            Self::ThunderclapPlus => None,
            Self::TrueGrit => Some(Self::TrueGritPlus),
            Self::TrueGritPlus => None,
            Self::TwinStrike => Some(Self::TwinStrikePlus),
            Self::TwinStrikePlus => None,
            Self::Warcry => Some(Self::WarcryPlus),
            Self::WarcryPlus => None,
            Self::WildStrike => Some(Self::WildStrikePlus),
            Self::WildStrikePlus => None,
            Self::Wound => None,
            Self::BattleTrance => Some(Self::BattleTrancePlus),
            Self::BattleTrancePlus => None,
            Self::BloodForBlood => Some(Self::BloodForBloodPlus),
            Self::BloodForBloodPlus => None,
            Self::Bloodletting => Some(Self::BloodlettingPlus),
            Self::BloodlettingPlus => None,
            Self::BurningPact => Some(Self::BurningPactPlus),
            Self::BurningPactPlus => None,
            Self::Carnage => Some(Self::CarnagePlus),
            Self::CarnagePlus => None,
            Self::Combust => Some(Self::CombustPlus),
            Self::CombustPlus => None,
            Self::DarkEmbrace => Some(Self::DarkEmbracePlus),
            Self::DarkEmbracePlus => None,
            Self::Disarm => Some(Self::DisarmPlus),
            Self::DisarmPlus => None,
            Self::Dropkick => Some(Self::DropkickPlus),
            Self::DropkickPlus => None,
            Self::DualWield => Some(Self::DualWieldPlus),
            Self::DualWieldPlus => None,
            Self::Entrench => Some(Self::Entrench),
            Self::EntrenchPlus => None,
            Self::Evolve => Some(Self::EvolvePlus),
            Self::EvolvePlus => None,
        }
    }
    pub fn is_strike(&self) -> bool {
        match self {
            Self::Strike
            | Self::StrikePlus
            | Self::PerfectedStrike
            | Self::PerfectedStrikePlus
            | Self::PommelStrike
            | Self::PommelStrikePlus
            | Self::TwinStrike
            | Self::TwinStrikePlus
            | Self::WildStrike
            | Self::WildStrikePlus => true,
            _ => false,
        }
    }
}
