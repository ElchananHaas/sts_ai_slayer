use std::cmp::max;

use strum::VariantArray;

use crate::rng::Rng;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Card {
    pub body: CardBody,
    pub cost: Cost,
    pub assoc_data: CardAssoc,
    pub temp_cost: Option<i32>,
    upgraded: bool,
}

//In order to have the CardBody enum be trivially constructable the
//extra data associated with a card is stored in a different enum.
//This stores effects like Searing Blow's unlimited upgrades
//or Genetic Algorithm scaling.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CardAssoc {
    None,
    UnlimitedUpgrade(i32), //Used for Searing Blow
    BonusDamage(i32),      //Used for Rampage
}

impl CardAssoc {
    pub fn get_unlimited_upgrade(&self) -> i32 {
        let Self::UnlimitedUpgrade(amount) = self else {
            panic!("Expected unlimited upgrade data");
        };
        *amount
    }
    pub fn get_bonus_damage(&self) -> i32 {
        let Self::BonusDamage(amount) = self else {
            panic!("Expected bonus damage data");
        };
        *amount
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Cost {
    Unplayable,
    Fixed(i32),
    X,
    NumMinusHpLoss(i32), //This is for Blood for Blood
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, VariantArray)]
pub enum CardBody {
    Strike,
    Bash,
    Defend,
    Slimed,
    Anger,
    Armaments,
    BodySlam,
    Clash,
    Cleave,
    Clothesline,
    Flex,
    Havoc,
    Headbutt,
    HeavyBlade,
    IronWave,
    PerfectedStrike,
    PommelStrike,
    ShrugItOff,
    SwordBoomerang,
    Thunderclap,
    TrueGrit,
    TwinStrike,
    Warcry,
    WildStrike,
    Wound,
    BattleTrance,
    BloodForBlood,
    Bloodletting,
    BurningPact,
    SearingBlow,
    Carnage,
    Combust,
    DarkEmbrace,
    Disarm,
    Dropkick,
    DualWield,
    Entrench,
    Evolve,
    FeelNoPain,
    FireBreathing,
    FlameBarrier,
    GhostlyArmor,
    Hemokinesis,
    InfernalBlade,
    Inflame,
    Intimidate,
    Metallicize,
    PowerThrough,
    Pummel,
    Rage,
    Rampage,
    RecklessCharge,
    Dazed,
    Rupture,
    SecondWind,
    SeeingRed,
    Sentinel,
    SeverSoul,
    Shockwave,
    SpotWeakness,
    Uppercut,
    Whirlwind,
    Barricade,
    Berserk,
    Bludgeon,
    Brutality,
    Corruption,
    DemonForm,
    DoubleTap,
    Exhume,
    Feed,
    FiendFire,
    Immolate,
    Burn,
    Impervious,
    Juggernaut,
    LimitBreak,
    Offering,
    Reaper,
    Regret,
    Injury,
    Parasite,
    Doubt,
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
    ShuffleInCard(CardBody),
    LoseHP(i32),
    GainEnergy(i32),
    DropkickDraw,
    DoubleBlock,
    GenerateAttackInfernal,
    AddCardToHand(CardBody),
    IncreaseDamage(i32),
    ExhaustNonAttackForBlock(i32),
    ExhaustNonAttack,
    SpotWeakness(i32),
    AttackAllX(i32),
    AttackLethalEffect(i32, LethalEffect),
    AttackFiendFire(i32),
    AddCardToDiscard(CardBody),
    DoubleStrength,
    AttackAllForHP(i32),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LethalEffect {
    //This is done to keep the enum size small.
    //An additional i32 would raise the overall size of the containing enum.
    Gain3MaxHP,
    Gain4MaxHP,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SelectCardEffect {
    UpgradeCardInHand,
    DiscardToTop,
    ExhaustChosen,
    HandToTop,
    DuplicatePowerOrAttack(i32),
    ExhaustToHand,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Debuff {
    Vulnerable(i32),
    Weak(i32),
    Frail(i32),
    StrengthDown(i32),
    DexterityDown(i32),
    MinusStrength(i32),
    MinusDexterity(i32),
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
    EvolveBuff(i32),
    FNPBuff(i32),
    FireBreathingBuff(i32),
    TempSpikes(i32),
    Metallicize(i32),
    RageBuff(i32),
    RuptureBuff(i32),
    BarricadeBuff,
    EnergyEveryTurn,
    BrutalityBuff,
    CorruptionBuff,
    DoubleTap(i32),
    Juggernaut(i32),
    Enrage(i32),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CardType {
    Attack,
    Skill,
    Power,
    Status,
    Curse,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CardCharachter {
    IRONCLAD,
    SILENT,
    DEFECT,
    WATCHER,
    COLORLESS,
}

struct CardProps {
    actions: &'static [PlayEffect],
    cost: Cost,
    requires_target: bool,
    card_type: CardType,
    upgraded_actions: &'static [PlayEffect],
    upgraded_cost: Cost,
    upgraded_requires_target: bool,
    ethereal: Ethereal,
    charachter: CardCharachter,
    removable: bool,
    starter: bool,
    innate: Innate,
}

enum Ethereal {
    No,
    Yes,
    NotUpgraded,
}

enum Innate {
    No,
    Yes,
    Upgraded,
}
impl CardProps {
    const fn new(
        actions: &'static [PlayEffect],
        upgraded_actions: &'static [PlayEffect],
        cost: Cost,
        requires_target: bool,
        card_type: CardType,
        charachter: CardCharachter,
    ) -> Self {
        Self {
            actions,
            cost,
            requires_target,
            card_type,
            upgraded_actions,
            upgraded_cost: cost,
            upgraded_requires_target: requires_target,
            ethereal: Ethereal::No,
            innate: Innate::No,
            removable: true,
            charachter,
            starter: false,
        }
    }
    const fn with_starter(self) -> Self {
        Self {
            starter: true,
            ..self
        }
    }
    const fn with_upgraded_cost(self, cost: Cost) -> Self {
        Self {
            upgraded_cost: cost,
            ..self
        }
    }
    const fn with_upgraded_requires_target(self, upgraded_requires_target: bool) -> Self {
        Self {
            upgraded_requires_target,
            ..self
        }
    }
    const fn with_ethereal(self, ethereal: Ethereal) -> Self {
        Self { ethereal, ..self }
    }
    const fn with_innate(self, innate: Innate) -> Self {
        Self { innate, ..self }
    }
}

macro_rules! const_card {
    ($expression:expr) => {{
        const CARD_PROPS: &'static CardProps = $expression;
        CARD_PROPS
    }};
}
impl CardBody {
    const fn props(&self) -> &'static CardProps {
        return match self {
            CardBody::Strike => const_card!(
                &CardProps::new(
                    &[PlayEffect::Attack(6)],
                    &[PlayEffect::Attack(9)],
                    Cost::Fixed(1),
                    true,
                    CardType::Attack,
                    CardCharachter::IRONCLAD //This can be found on all charachters but is
                                             //a starter so it doesn't matter.
                )
                .with_starter()
            ),
            CardBody::Bash => const_card!(
                &CardProps::new(
                    &[
                        PlayEffect::Attack(8),
                        PlayEffect::DebuffEnemy(Debuff::Vulnerable(2)),
                    ],
                    &[
                        PlayEffect::Attack(10),
                        PlayEffect::DebuffEnemy(Debuff::Vulnerable(3)),
                    ],
                    Cost::Fixed(2),
                    true,
                    CardType::Attack,
                    CardCharachter::IRONCLAD
                )
                .with_starter()
            ),
            CardBody::Defend => const_card!(
                &CardProps::new(
                    &[PlayEffect::Block(5)],
                    &[PlayEffect::Block(8)],
                    Cost::Fixed(1),
                    false,
                    CardType::Skill,
                    CardCharachter::IRONCLAD //This can be found on all charachters but is
                                             //a starter so it doesn't matter.
                )
                .with_starter()
            ),
            CardBody::Slimed => const_card!(&CardProps::new(
                &[PlayEffect::MarkExhaust],
                &[PlayEffect::MarkExhaust],
                Cost::Fixed(1),
                false,
                CardType::Status,
                CardCharachter::COLORLESS
            )),
            CardBody::Anger => const_card!(&CardProps::new(
                &[PlayEffect::Attack(6), PlayEffect::AddCopyToDiscard],
                &[PlayEffect::Attack(8), PlayEffect::AddCopyToDiscard],
                Cost::Fixed(0),
                true,
                CardType::Attack,
                CardCharachter::IRONCLAD
            )),
            CardBody::Armaments => const_card!(&CardProps::new(
                &[
                    PlayEffect::Block(5),
                    PlayEffect::SelectCardEffect(SelectCardEffect::UpgradeCardInHand),
                ],
                &[PlayEffect::Block(5), PlayEffect::UpgradeAllCardsInHand],
                Cost::Fixed(1),
                false,
                CardType::Skill,
                CardCharachter::IRONCLAD
            )),
            CardBody::BodySlam => const_card!(
                &CardProps::new(
                    &[PlayEffect::AttackEqualBlock],
                    &[PlayEffect::AttackEqualBlock],
                    Cost::Fixed(1),
                    true,
                    CardType::Attack,
                    CardCharachter::IRONCLAD
                )
                .with_upgraded_cost(Cost::Fixed(0))
            ),
            CardBody::Clash => const_card!(&CardProps::new(
                &[PlayEffect::Attack(14)],
                &[PlayEffect::Attack(18)],
                Cost::Fixed(0),
                true,
                CardType::Attack,
                CardCharachter::IRONCLAD
            )),
            CardBody::Cleave => const_card!(&CardProps::new(
                &[PlayEffect::AttackAll(8)],
                &[PlayEffect::AttackAll(11)],
                Cost::Fixed(1),
                false,
                CardType::Attack,
                CardCharachter::IRONCLAD
            )),
            CardBody::Clothesline => const_card!(&CardProps::new(
                &[
                    PlayEffect::Attack(12),
                    PlayEffect::DebuffEnemy(Debuff::Weak(2)),
                ],
                &[
                    PlayEffect::Attack(14),
                    PlayEffect::DebuffEnemy(Debuff::Weak(3)),
                ],
                Cost::Fixed(2),
                true,
                CardType::Attack,
                CardCharachter::IRONCLAD
            )),
            CardBody::Flex => const_card!(&CardProps::new(
                &[
                    PlayEffect::Buff(Buff::Strength(2)),
                    PlayEffect::DebuffSelf(Debuff::StrengthDown(2)),
                ],
                &[
                    PlayEffect::Buff(Buff::Strength(4)),
                    PlayEffect::DebuffSelf(Debuff::StrengthDown(4)),
                ],
                Cost::Fixed(0),
                false,
                CardType::Skill,
                CardCharachter::IRONCLAD
            )),
            CardBody::Havoc => const_card!(
                &CardProps::new(
                    &[PlayEffect::PlayExhaustTop],
                    &[PlayEffect::PlayExhaustTop],
                    Cost::Fixed(1),
                    false,
                    CardType::Skill,
                    CardCharachter::IRONCLAD
                )
                .with_upgraded_cost(Cost::Fixed(0))
            ),
            CardBody::Headbutt => const_card!(&CardProps::new(
                &[
                    PlayEffect::Attack(9),
                    PlayEffect::SelectCardEffect(SelectCardEffect::DiscardToTop),
                ],
                &[
                    PlayEffect::Attack(12),
                    PlayEffect::SelectCardEffect(SelectCardEffect::DiscardToTop),
                ],
                Cost::Fixed(1),
                true,
                CardType::Attack,
                CardCharachter::IRONCLAD
            )),
            CardBody::HeavyBlade => const_card!(&CardProps::new(
                &[PlayEffect::Attack(14)],
                &[PlayEffect::Attack(14)],
                Cost::Fixed(2),
                true,
                CardType::Attack,
                CardCharachter::IRONCLAD
            )),
            CardBody::IronWave => const_card!(&CardProps::new(
                &[PlayEffect::Block(5), PlayEffect::Attack(5)],
                &[PlayEffect::Block(7), PlayEffect::Attack(7)],
                Cost::Fixed(1),
                true,
                CardType::Attack,
                CardCharachter::IRONCLAD
            )),
            CardBody::SearingBlow => const_card!(&CardProps::new(
                &[PlayEffect::Attack(12)],
                &[PlayEffect::Attack(12)],
                Cost::Fixed(2),
                true,
                CardType::Attack,
                CardCharachter::IRONCLAD
            )),
            CardBody::PerfectedStrike => const_card!(&CardProps::new(
                &[PlayEffect::Attack(6)],
                &[PlayEffect::Attack(6)],
                Cost::Fixed(2),
                true,
                CardType::Attack,
                CardCharachter::IRONCLAD
            )),
            CardBody::PommelStrike => const_card!(&CardProps::new(
                &[PlayEffect::Attack(9), PlayEffect::Draw(1)],
                &[PlayEffect::Attack(10), PlayEffect::Draw(2)],
                Cost::Fixed(1),
                true,
                CardType::Attack,
                CardCharachter::IRONCLAD
            )),
            CardBody::ShrugItOff => const_card!(&CardProps::new(
                &[PlayEffect::Block(8), PlayEffect::Draw(1)],
                &[PlayEffect::Block(11), PlayEffect::Draw(1)],
                Cost::Fixed(1),
                false,
                CardType::Skill,
                CardCharachter::IRONCLAD
            )),
            CardBody::SwordBoomerang => const_card!(&CardProps::new(
                &[
                    PlayEffect::AttackRandomEnemy(3),
                    PlayEffect::AttackRandomEnemy(3),
                    PlayEffect::AttackRandomEnemy(3),
                ],
                &[
                    PlayEffect::AttackRandomEnemy(3),
                    PlayEffect::AttackRandomEnemy(3),
                    PlayEffect::AttackRandomEnemy(3),
                    PlayEffect::AttackRandomEnemy(3),
                ],
                Cost::Fixed(1),
                false,
                CardType::Attack,
                CardCharachter::IRONCLAD
            )),
            CardBody::Thunderclap => const_card!(&CardProps::new(
                &[
                    PlayEffect::AttackAll(4),
                    PlayEffect::DebuffAll(Debuff::Vulnerable(1)),
                ],
                &[
                    PlayEffect::AttackAll(7),
                    PlayEffect::DebuffAll(Debuff::Vulnerable(1)),
                ],
                Cost::Fixed(1),
                false,
                CardType::Attack,
                CardCharachter::IRONCLAD
            )),
            CardBody::TrueGrit => const_card!(&CardProps::new(
                &[PlayEffect::Block(7), PlayEffect::ExhaustRandomInHand],
                &[
                    PlayEffect::Block(7),
                    PlayEffect::SelectCardEffect(SelectCardEffect::ExhaustChosen),
                ],
                Cost::Fixed(1),
                false,
                CardType::Skill,
                CardCharachter::IRONCLAD
            )),
            CardBody::TwinStrike => const_card!(&CardProps::new(
                &[PlayEffect::Attack(5), PlayEffect::Attack(5)],
                &[PlayEffect::Attack(7), PlayEffect::Attack(7)],
                Cost::Fixed(1),
                true,
                CardType::Attack,
                CardCharachter::IRONCLAD
            )),
            CardBody::Warcry => const_card!(&CardProps::new(
                &[
                    PlayEffect::Draw(1),
                    PlayEffect::SelectCardEffect(SelectCardEffect::HandToTop),
                    PlayEffect::MarkExhaust,
                ],
                &[
                    PlayEffect::Draw(2),
                    PlayEffect::SelectCardEffect(SelectCardEffect::HandToTop),
                    PlayEffect::MarkExhaust,
                ],
                Cost::Fixed(0),
                false,
                CardType::Skill,
                CardCharachter::IRONCLAD
            )),
            CardBody::WildStrike => const_card!(&CardProps::new(
                &[
                    PlayEffect::Attack(12),
                    PlayEffect::ShuffleInCard(CardBody::Wound),
                ],
                &[
                    PlayEffect::Attack(17),
                    PlayEffect::ShuffleInCard(CardBody::Wound),
                ],
                Cost::Fixed(1),
                true,
                CardType::Attack,
                CardCharachter::IRONCLAD
            )),
            CardBody::Wound => const_card!(&CardProps::new(
                &[PlayEffect::MarkExhaust],
                &[PlayEffect::MarkExhaust],
                Cost::Unplayable,
                false,
                CardType::Status,
                CardCharachter::COLORLESS
            )),
            CardBody::BattleTrance => const_card!(&CardProps::new(
                &[PlayEffect::Draw(3), PlayEffect::DebuffSelf(Debuff::NoDraw)],
                &[PlayEffect::Draw(4), PlayEffect::DebuffSelf(Debuff::NoDraw)],
                Cost::Fixed(0),
                false,
                CardType::Skill,
                CardCharachter::IRONCLAD
            )),
            CardBody::BloodForBlood => const_card!(
                &CardProps::new(
                    &[PlayEffect::Attack(18)],
                    &[PlayEffect::Attack(22)],
                    Cost::NumMinusHpLoss(4),
                    true,
                    CardType::Attack,
                    CardCharachter::IRONCLAD
                )
                .with_upgraded_cost(Cost::NumMinusHpLoss(3))
            ),
            CardBody::Bloodletting => const_card!(&CardProps::new(
                &[PlayEffect::LoseHP(3), PlayEffect::GainEnergy(2)],
                &[PlayEffect::LoseHP(3), PlayEffect::GainEnergy(3)],
                Cost::Fixed(0),
                false,
                CardType::Skill,
                CardCharachter::IRONCLAD
            )),
            CardBody::BurningPact => const_card!(&CardProps::new(
                &[
                    PlayEffect::SelectCardEffect(SelectCardEffect::ExhaustChosen),
                    PlayEffect::Draw(2),
                ],
                &[
                    PlayEffect::SelectCardEffect(SelectCardEffect::ExhaustChosen),
                    PlayEffect::Draw(3),
                ],
                Cost::Fixed(1),
                false,
                CardType::Skill,
                CardCharachter::IRONCLAD
            )),
            CardBody::Carnage => const_card!(
                &CardProps::new(
                    &[PlayEffect::Attack(20)],
                    &[PlayEffect::Attack(28)],
                    Cost::Fixed(2),
                    true,
                    CardType::Attack,
                    CardCharachter::IRONCLAD
                )
                .with_ethereal(Ethereal::Yes)
            ),
            CardBody::Combust => const_card!(&CardProps::new(
                &[
                    PlayEffect::Buff(Buff::EndTurnLoseHP(1)),
                    PlayEffect::Buff(Buff::EndTurnDamageAllEnemies(5)),
                ],
                &[
                    PlayEffect::Buff(Buff::EndTurnLoseHP(1)),
                    PlayEffect::Buff(Buff::EndTurnDamageAllEnemies(7)),
                ],
                Cost::Fixed(1),
                false,
                CardType::Power,
                CardCharachter::IRONCLAD
            )),
            CardBody::DarkEmbrace => const_card!(
                &CardProps::new(
                    &[PlayEffect::Buff(Buff::DarkEmbraceBuff)],
                    &[PlayEffect::Buff(Buff::DarkEmbraceBuff)],
                    Cost::Fixed(2),
                    false,
                    CardType::Power,
                    CardCharachter::IRONCLAD
                )
                .with_upgraded_cost(Cost::Fixed(1))
            ),
            CardBody::Disarm => const_card!(&CardProps::new(
                &[
                    PlayEffect::DebuffEnemy(Debuff::StrengthDown(2)),
                    PlayEffect::MarkExhaust,
                ],
                &[
                    PlayEffect::DebuffEnemy(Debuff::StrengthDown(3)),
                    PlayEffect::MarkExhaust,
                ],
                Cost::Fixed(1),
                true,
                CardType::Skill,
                CardCharachter::IRONCLAD
            )),
            CardBody::Dropkick => const_card!(&CardProps::new(
                &[PlayEffect::Attack(5), PlayEffect::DropkickDraw],
                &[PlayEffect::Attack(8), PlayEffect::DropkickDraw],
                Cost::Fixed(1),
                true,
                CardType::Attack,
                CardCharachter::IRONCLAD
            )),
            CardBody::DualWield => const_card!(&CardProps::new(
                &[PlayEffect::SelectCardEffect(
                    SelectCardEffect::DuplicatePowerOrAttack(1),
                )],
                &[PlayEffect::SelectCardEffect(
                    SelectCardEffect::DuplicatePowerOrAttack(2),
                )],
                Cost::Fixed(1),
                false,
                CardType::Skill,
                CardCharachter::IRONCLAD
            )),
            CardBody::Entrench => const_card!(
                &CardProps::new(
                    &[PlayEffect::DoubleBlock],
                    &[PlayEffect::DoubleBlock],
                    Cost::Fixed(2),
                    false,
                    CardType::Skill,
                    CardCharachter::IRONCLAD
                )
                .with_upgraded_cost(Cost::Fixed(1))
            ),
            CardBody::Evolve => const_card!(&CardProps::new(
                &[PlayEffect::Buff(Buff::EvolveBuff(1))],
                &[PlayEffect::Buff(Buff::EvolveBuff(2))],
                Cost::Fixed(1),
                false,
                CardType::Power,
                CardCharachter::IRONCLAD
            )),
            CardBody::FeelNoPain => const_card!(&CardProps::new(
                &[PlayEffect::Buff(Buff::FNPBuff(3))],
                &[PlayEffect::Buff(Buff::FNPBuff(4))],
                Cost::Fixed(1),
                false,
                CardType::Power,
                CardCharachter::IRONCLAD
            )),
            CardBody::FireBreathing => const_card!(&CardProps::new(
                &[PlayEffect::Buff(Buff::FireBreathingBuff(6))],
                &[PlayEffect::Buff(Buff::FireBreathingBuff(10))],
                Cost::Fixed(1),
                false,
                CardType::Power,
                CardCharachter::IRONCLAD
            )),
            CardBody::FlameBarrier => const_card!(&CardProps::new(
                &[PlayEffect::Block(12), PlayEffect::Buff(Buff::TempSpikes(4))],
                &[PlayEffect::Block(16), PlayEffect::Buff(Buff::TempSpikes(6))],
                Cost::Fixed(2),
                false,
                CardType::Skill,
                CardCharachter::IRONCLAD
            )),
            CardBody::GhostlyArmor => const_card!(
                &CardProps::new(
                    &[PlayEffect::Block(10)],
                    &[PlayEffect::Block(13)],
                    Cost::Fixed(1),
                    false,
                    CardType::Skill,
                    CardCharachter::IRONCLAD
                )
                .with_ethereal(Ethereal::Yes)
            ),
            CardBody::Hemokinesis => const_card!(&CardProps::new(
                &[PlayEffect::LoseHP(2), PlayEffect::Attack(15)],
                &[PlayEffect::LoseHP(2), PlayEffect::Attack(20)],
                Cost::Fixed(1),
                true,
                CardType::Attack,
                CardCharachter::IRONCLAD
            )),
            CardBody::InfernalBlade => const_card!(
                &CardProps::new(
                    &[PlayEffect::GenerateAttackInfernal],
                    &[PlayEffect::GenerateAttackInfernal],
                    Cost::Fixed(1),
                    false,
                    CardType::Skill,
                    CardCharachter::IRONCLAD
                )
                .with_upgraded_cost(Cost::Fixed(0))
            ),
            CardBody::Inflame => const_card!(&CardProps::new(
                &[PlayEffect::Buff(Buff::Strength(2))],
                &[PlayEffect::Buff(Buff::Strength(3))],
                Cost::Fixed(1),
                false,
                CardType::Power,
                CardCharachter::IRONCLAD
            )),
            CardBody::Intimidate => const_card!(&CardProps::new(
                &[
                    PlayEffect::DebuffAll(Debuff::Weak(1)),
                    PlayEffect::MarkExhaust
                ],
                &[
                    PlayEffect::DebuffAll(Debuff::Weak(2)),
                    PlayEffect::MarkExhaust
                ],
                Cost::Fixed(0),
                false,
                CardType::Skill,
                CardCharachter::IRONCLAD
            )),
            CardBody::Metallicize => const_card!(&CardProps::new(
                &[PlayEffect::Buff(Buff::Metallicize(3))],
                &[PlayEffect::Buff(Buff::Metallicize(4))],
                Cost::Fixed(1),
                false,
                CardType::Power,
                CardCharachter::IRONCLAD
            )),
            CardBody::PowerThrough => const_card!(&CardProps::new(
                &[
                    PlayEffect::Block(15),
                    PlayEffect::AddCardToHand(CardBody::Wound),
                    PlayEffect::AddCardToHand(CardBody::Wound)
                ],
                &[
                    PlayEffect::Block(20),
                    PlayEffect::AddCardToHand(CardBody::Wound),
                    PlayEffect::AddCardToHand(CardBody::Wound)
                ],
                Cost::Fixed(1),
                false,
                CardType::Skill,
                CardCharachter::IRONCLAD
            )),
            CardBody::Pummel => const_card!(&CardProps::new(
                &[
                    PlayEffect::Attack(2),
                    PlayEffect::Attack(2),
                    PlayEffect::Attack(2),
                    PlayEffect::Attack(2),
                ],
                &[
                    PlayEffect::Attack(2),
                    PlayEffect::Attack(2),
                    PlayEffect::Attack(2),
                    PlayEffect::Attack(2),
                    PlayEffect::Attack(2),
                ],
                Cost::Fixed(1),
                true,
                CardType::Attack,
                CardCharachter::IRONCLAD
            )),
            CardBody::Rage => const_card!(&CardProps::new(
                &[PlayEffect::Buff(Buff::RageBuff(3))],
                &[PlayEffect::Buff(Buff::RageBuff(5))],
                Cost::Fixed(0),
                false,
                CardType::Skill,
                CardCharachter::IRONCLAD
            )),
            CardBody::Rampage => const_card!(&CardProps::new(
                &[PlayEffect::Attack(8), PlayEffect::IncreaseDamage(5)],
                &[PlayEffect::Attack(8), PlayEffect::IncreaseDamage(5)],
                Cost::Fixed(1),
                true,
                CardType::Attack,
                CardCharachter::IRONCLAD
            )),
            CardBody::RecklessCharge => const_card!(&CardProps::new(
                &[
                    PlayEffect::Attack(7),
                    PlayEffect::ShuffleInCard(CardBody::Dazed)
                ],
                &[
                    PlayEffect::Attack(10),
                    PlayEffect::ShuffleInCard(CardBody::Dazed)
                ],
                Cost::Fixed(0),
                true,
                CardType::Attack,
                CardCharachter::IRONCLAD
            )),
            CardBody::Dazed => const_card!(
                &CardProps::new(
                    &[PlayEffect::MarkExhaust],
                    &[PlayEffect::MarkExhaust],
                    Cost::Unplayable,
                    false,
                    CardType::Status,
                    CardCharachter::COLORLESS
                )
                .with_ethereal(Ethereal::Yes)
            ),
            CardBody::Rupture => const_card!(&CardProps::new(
                &[PlayEffect::Buff(Buff::RuptureBuff(1)),],
                &[PlayEffect::Buff(Buff::RuptureBuff(2)),],
                Cost::Fixed(1),
                false,
                CardType::Power,
                CardCharachter::IRONCLAD
            )),
            CardBody::SecondWind => const_card!(&CardProps::new(
                &[PlayEffect::ExhaustNonAttackForBlock(5)],
                &[PlayEffect::ExhaustNonAttackForBlock(7)],
                Cost::Fixed(1),
                false,
                CardType::Skill,
                CardCharachter::IRONCLAD
            )),
            CardBody::SeeingRed => const_card!(
                &CardProps::new(
                    &[PlayEffect::GainEnergy(2), PlayEffect::MarkExhaust],
                    &[PlayEffect::GainEnergy(2), PlayEffect::MarkExhaust],
                    Cost::Fixed(1),
                    false,
                    CardType::Skill,
                    CardCharachter::IRONCLAD
                )
                .with_upgraded_cost(Cost::Fixed(0))
            ),
            CardBody::Sentinel => const_card!(&CardProps::new(
                &[PlayEffect::Block(5)],
                &[PlayEffect::Block(8)],
                Cost::Fixed(1),
                false,
                CardType::Skill,
                CardCharachter::IRONCLAD
            )),
            CardBody::SeverSoul => const_card!(&CardProps::new(
                &[PlayEffect::Attack(16), PlayEffect::ExhaustNonAttack],
                &[PlayEffect::Attack(22), PlayEffect::ExhaustNonAttack],
                Cost::Fixed(2),
                true,
                CardType::Attack,
                CardCharachter::IRONCLAD
            )),
            CardBody::Shockwave => const_card!(&CardProps::new(
                &[
                    PlayEffect::DebuffAll(Debuff::Weak(3)),
                    PlayEffect::DebuffAll(Debuff::Vulnerable(3)),
                    PlayEffect::MarkExhaust
                ],
                &[
                    PlayEffect::DebuffAll(Debuff::Weak(5)),
                    PlayEffect::DebuffAll(Debuff::Vulnerable(5)),
                    PlayEffect::MarkExhaust
                ],
                Cost::Fixed(2),
                false,
                CardType::Skill,
                CardCharachter::IRONCLAD
            )),
            CardBody::SpotWeakness => const_card!(&CardProps::new(
                &[PlayEffect::SpotWeakness(3),],
                &[PlayEffect::SpotWeakness(3),],
                Cost::Fixed(1),
                true,
                CardType::Skill,
                CardCharachter::IRONCLAD
            )),
            CardBody::Uppercut => const_card!(&CardProps::new(
                &[
                    PlayEffect::Attack(13),
                    PlayEffect::DebuffEnemy(Debuff::Weak(1)),
                    PlayEffect::DebuffEnemy(Debuff::Vulnerable(1))
                ],
                &[
                    PlayEffect::Attack(13),
                    PlayEffect::DebuffEnemy(Debuff::Weak(2)),
                    PlayEffect::DebuffEnemy(Debuff::Vulnerable(2))
                ],
                Cost::Fixed(2),
                true,
                CardType::Attack,
                CardCharachter::IRONCLAD
            )),
            CardBody::Whirlwind => const_card!(&CardProps::new(
                &[PlayEffect::AttackAllX(5)],
                &[PlayEffect::AttackAllX(8)],
                Cost::X,
                false,
                CardType::Attack,
                CardCharachter::IRONCLAD
            )),
            CardBody::Barricade => const_card!(
                &CardProps::new(
                    &[PlayEffect::Buff(Buff::BarricadeBuff)],
                    &[PlayEffect::Buff(Buff::BarricadeBuff)],
                    Cost::Fixed(3),
                    false,
                    CardType::Power,
                    CardCharachter::IRONCLAD
                )
                .with_upgraded_cost(Cost::Fixed(2))
            ),
            CardBody::Berserk => const_card!(&CardProps::new(
                &[
                    PlayEffect::Buff(Buff::EnergyEveryTurn),
                    PlayEffect::DebuffSelf(Debuff::Vulnerable(2))
                ],
                &[
                    PlayEffect::Buff(Buff::EnergyEveryTurn),
                    PlayEffect::DebuffSelf(Debuff::Vulnerable(2))
                ],
                Cost::Fixed(0),
                false,
                CardType::Power,
                CardCharachter::IRONCLAD
            )),
            CardBody::Bludgeon => const_card!(&CardProps::new(
                &[PlayEffect::Attack(32)],
                &[PlayEffect::Attack(42)],
                Cost::Fixed(3),
                true,
                CardType::Attack,
                CardCharachter::IRONCLAD
            )),
            CardBody::Brutality => const_card!(
                &CardProps::new(
                    &[PlayEffect::Buff(Buff::BrutalityBuff),],
                    &[PlayEffect::Buff(Buff::BrutalityBuff),],
                    Cost::Fixed(0),
                    false,
                    CardType::Power,
                    CardCharachter::IRONCLAD
                )
                .with_innate(Innate::Upgraded)
            ),
            CardBody::Corruption => const_card!(
                &CardProps::new(
                    &[PlayEffect::Buff(Buff::CorruptionBuff)],
                    &[PlayEffect::Buff(Buff::CorruptionBuff)],
                    Cost::Fixed(3),
                    false,
                    CardType::Power,
                    CardCharachter::IRONCLAD
                )
                .with_upgraded_cost(Cost::Fixed(2))
            ),
            CardBody::DemonForm => const_card!(&CardProps::new(
                &[PlayEffect::Buff(Buff::Ritual(2))],
                &[PlayEffect::Buff(Buff::Ritual(3))],
                Cost::Fixed(3),
                false,
                CardType::Power,
                CardCharachter::IRONCLAD
            )),
            CardBody::DoubleTap => const_card!(&CardProps::new(
                &[PlayEffect::Buff(Buff::DoubleTap(1))],
                &[PlayEffect::Buff(Buff::DoubleTap(2))],
                Cost::Fixed(1),
                false,
                CardType::Skill,
                CardCharachter::IRONCLAD
            )),
            CardBody::Exhume => const_card!(
                &CardProps::new(
                    &[
                        PlayEffect::SelectCardEffect(SelectCardEffect::ExhaustToHand),
                        PlayEffect::MarkExhaust
                    ],
                    &[
                        PlayEffect::SelectCardEffect(SelectCardEffect::ExhaustToHand),
                        PlayEffect::MarkExhaust
                    ],
                    Cost::Fixed(1),
                    false,
                    CardType::Skill,
                    CardCharachter::IRONCLAD
                )
                .with_upgraded_cost(Cost::Fixed(0))
            ),
            CardBody::Feed => const_card!(&CardProps::new(
                &[PlayEffect::AttackLethalEffect(10, LethalEffect::Gain3MaxHP)],
                &[PlayEffect::AttackLethalEffect(12, LethalEffect::Gain4MaxHP)],
                Cost::Fixed(1),
                true,
                CardType::Attack,
                CardCharachter::IRONCLAD
            )),
            CardBody::FiendFire => const_card!(&CardProps::new(
                &[PlayEffect::AttackFiendFire(7), PlayEffect::MarkExhaust],
                &[PlayEffect::AttackFiendFire(10), PlayEffect::MarkExhaust],
                Cost::Fixed(2),
                true,
                CardType::Attack,
                CardCharachter::IRONCLAD
            )),
            CardBody::Immolate => const_card!(&CardProps::new(
                &[
                    PlayEffect::AttackAll(21),
                    PlayEffect::AddCardToDiscard(CardBody::Burn)
                ],
                &[
                    PlayEffect::AttackAll(28),
                    PlayEffect::AddCardToDiscard(CardBody::Burn)
                ],
                Cost::Fixed(2),
                false,
                CardType::Attack,
                CardCharachter::IRONCLAD
            )),
            CardBody::Burn => const_card!(&CardProps::new(
                &[PlayEffect::MarkExhaust],
                &[PlayEffect::MarkExhaust],
                Cost::Unplayable,
                false,
                CardType::Status,
                CardCharachter::COLORLESS
            )),
            CardBody::Impervious => const_card!(&CardProps::new(
                &[PlayEffect::Block(30), PlayEffect::MarkExhaust],
                &[PlayEffect::Block(40), PlayEffect::MarkExhaust],
                Cost::Fixed(2),
                false,
                CardType::Skill,
                CardCharachter::IRONCLAD
            )),
            CardBody::Juggernaut => const_card!(&CardProps::new(
                &[PlayEffect::Buff(Buff::Juggernaut(5))],
                &[PlayEffect::Buff(Buff::Juggernaut(7))],
                Cost::Fixed(2),
                false,
                CardType::Power,
                CardCharachter::IRONCLAD
            )),
            CardBody::LimitBreak => const_card!(&CardProps::new(
                &[PlayEffect::DoubleStrength, PlayEffect::MarkExhaust],
                &[PlayEffect::DoubleStrength],
                Cost::Fixed(1),
                false,
                CardType::Skill,
                CardCharachter::IRONCLAD
            )),
            CardBody::Offering => const_card!(&CardProps::new(
                &[
                    PlayEffect::LoseHP(6),
                    PlayEffect::GainEnergy(2),
                    PlayEffect::Draw(3),
                    PlayEffect::MarkExhaust
                ],
                &[
                    PlayEffect::LoseHP(6),
                    PlayEffect::GainEnergy(2),
                    PlayEffect::Draw(5),
                    PlayEffect::MarkExhaust
                ],
                Cost::Fixed(0),
                false,
                CardType::Skill,
                CardCharachter::IRONCLAD
            )),
            CardBody::Reaper => const_card!(&CardProps::new(
                &[PlayEffect::AttackAllForHP(4), PlayEffect::MarkExhaust],
                &[PlayEffect::AttackAllForHP(5), PlayEffect::MarkExhaust],
                Cost::Fixed(2),
                false,
                CardType::Attack,
                CardCharachter::IRONCLAD
            )),
            CardBody::Regret => const_card!(&CardProps::new(
                &[PlayEffect::MarkExhaust],
                &[PlayEffect::MarkExhaust],
                Cost::Unplayable,
                false,
                CardType::Curse,
                CardCharachter::COLORLESS
            )),
            CardBody::Injury => const_card!(&CardProps::new(
                &[PlayEffect::MarkExhaust],
                &[PlayEffect::MarkExhaust],
                Cost::Unplayable,
                false,
                CardType::Curse,
                CardCharachter::COLORLESS
            )),
            CardBody::Parasite => const_card!(&CardProps::new(
                &[PlayEffect::MarkExhaust],
                &[PlayEffect::MarkExhaust],
                Cost::Unplayable,
                false,
                CardType::Curse,
                CardCharachter::COLORLESS
            )),
            CardBody::Doubt => const_card!(&CardProps::new(
                &[PlayEffect::MarkExhaust],
                &[PlayEffect::MarkExhaust],
                Cost::Unplayable,
                false,
                CardType::Curse,
                CardCharachter::COLORLESS
            )),
        };
    }
    pub const fn to_card(&self) -> Card {
        let assoc_data = match self {
            Self::SearingBlow => CardAssoc::UnlimitedUpgrade(0),
            _ => CardAssoc::None,
        };
        Card {
            cost: self.default_cost(),
            body: *self,
            assoc_data,
            upgraded: false,
            temp_cost: None,
        }
    }
    pub const fn default_cost(&self) -> Cost {
        self.props().cost
    }
    pub fn card_type(&self) -> CardType {
        self.props().card_type
    }
    pub fn removable(&self) -> bool {
        self.props().removable
    }
    pub fn is_strike(&self) -> bool {
        match self {
            Self::Strike
            | Self::PerfectedStrike
            | Self::PommelStrike
            | Self::TwinStrike
            | Self::WildStrike => true,
            _ => false,
        }
    }
}
macro_rules! filtered_cards {
    ($expression:expr) => {{
        const fn get_num_variants() -> usize {
            let num_variants = CardBody::VARIANTS.len();
            let mut i = 0;
            let mut matching = 0;
            while i < num_variants {
                let variant = CardBody::VARIANTS[i];
                if ($expression)(variant.props()) {
                    matching += 1;
                }
                i += 1;
            }
            matching
        }
        const NUM_MATCHING: usize = get_num_variants();
        const fn get_filtered_arr() -> [CardBody; NUM_MATCHING] {
            let num_variants = CardBody::VARIANTS.len();
            let mut i = 0;
            let mut output = [CardBody::Strike; NUM_MATCHING];
            let mut out_pos = 0;
            while i < num_variants {
                let variant = CardBody::VARIANTS[i];
                if ($expression)(variant.props()) {
                    output[out_pos] = variant;
                    out_pos += 1;
                }
                i += 1;
            }
            output
        }
        &get_filtered_arr()
    }};
}
const fn ironclad_attack_filter(props: &'static CardProps) -> bool {
    matches!(props.card_type, CardType::Attack)
        && matches!(props.charachter, CardCharachter::IRONCLAD)
        && !props.starter
}
pub const IRONCLAD_ATTACK_CARDS: &'static [CardBody] = filtered_cards!(ironclad_attack_filter);

const fn ironclad_filter(props: &'static CardProps) -> bool {
    matches!(props.charachter, CardCharachter::IRONCLAD) && !props.starter
}
pub const IRONCLAD_CARDS: &'static [CardBody] = filtered_cards!(ironclad_filter);

const fn curse_filter(props: &'static CardProps) -> bool {
    matches!(props.card_type, CardType::Curse)
}
pub const CURSE_CARDS: &'static [CardBody] = filtered_cards!(curse_filter);

const fn colorless_filter(props: &'static CardProps) -> bool {
    (matches!(props.card_type, CardType::Power)
        || matches!(props.card_type, CardType::Skill)
        || matches!(props.card_type, CardType::Attack))
        && matches!(props.charachter, CardCharachter::COLORLESS)
}

pub const COLORLESS_CARDS: &'static [CardBody] = filtered_cards!(curse_filter);

pub fn sample_card(cards: &'static [CardBody], rng: &mut Rng) -> CardBody {
    let idx = rng.sample(cards.len());
    cards[idx]
}

impl Card {
    fn props(&self) -> &'static CardProps {
        self.body.props()
    }
    pub fn ethereal(&self) -> bool {
        match self.body.props().ethereal {
            Ethereal::No => false,
            Ethereal::Yes => true,
            Ethereal::NotUpgraded => !self.upgraded,
        }
    }
    pub fn is_upgraded(&self) -> bool {
        self.upgraded
    }

    pub fn can_upgrade(&self) -> bool {
        let t = self.body.props().card_type;
        t != CardType::Status && t != CardType::Curse && !self.upgraded
    }

    pub fn upgrade(&mut self) {
        assert!(self.can_upgrade());
        if self.body == CardBody::SearingBlow {
            let amount = self.assoc_data.get_unlimited_upgrade();
            self.assoc_data = CardAssoc::UnlimitedUpgrade(amount + 1);
        }
        let props = self.props();
        if let Cost::Fixed(old) = props.cost
            && let Cost::Fixed(new) = props.upgraded_cost
            && let Cost::Fixed(current) = self.cost
        {
            self.cost = Cost::Fixed(max(current + new - old, 0));
        }
        //TODO - handle Blood for Blood and temp upgrades
        //TODO - Blood for Blood is incorrect when there is Snecko Eye.
        if self.body == CardBody::BloodForBlood {
            self.cost = props.upgraded_cost;
        }
        self.upgraded = true;
    }
    pub fn actions(&self) -> &'static [PlayEffect] {
        if self.upgraded {
            self.props().upgraded_actions
        } else {
            self.props().actions
        }
    }

    pub fn requires_target(&self) -> bool {
        if self.upgraded {
            self.props().upgraded_requires_target
        } else {
            self.props().requires_target
        }
    }

    pub fn innate(&self) -> bool {
        match self.body.props().innate {
            Innate::No => false,
            Innate::Yes => true,
            Innate::Upgraded => self.upgraded,
        }
    }

    pub fn charachter(&self) -> CardCharachter {
        self.body.props().charachter
    }
}
