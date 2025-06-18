use std::cmp::max;

use strum::VariantArray;

use crate::game::Charachter;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Card {
    pub body: CardBody,
    pub cost: Cost,
    pub assoc_data: CardAssoc,
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
}

impl CardAssoc {
    pub fn get_unlimited_upgrade(&self) -> i32 {
        let Self::UnlimitedUpgrade(amount) = self else {
            panic!("Expected unlimited upgrade data");
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
    GenerateAttackInfernal,
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
    EvolveBuff(i32),
    FNPBuff(i32),
    FireBreathingBuff(i32),
    TempSpikes(i32),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CardType {
    Attack,
    Skill,
    Power,
    Status,
    Curse,
}

struct CardProps {
    actions: &'static [PlayEffect],
    cost: Cost,
    requires_target: bool,
    card_type: CardType,
    upgradable: bool,
    upgraded_actions: &'static [PlayEffect],
    upgraded_cost: Cost,
    upgraded_requires_target: bool,
    ethereal: Ethereal,
}

enum Ethereal {
    No,
    Yes,
    NotUpgraded,
}

impl CardProps {
    const fn new(
        actions: &'static [PlayEffect],
        upgraded_actions: &'static [PlayEffect],
        cost: Cost,
        requires_target: bool,
        card_type: CardType,
    ) -> Self {
        Self {
            actions,
            cost,
            requires_target,
            card_type,
            upgraded_actions,
            upgraded_cost: cost,
            upgraded_requires_target: requires_target,
            upgradable: true,
            ethereal: Ethereal::No,
        }
    }
    const fn not_upgradable(self) -> Self {
        Self {
            upgradable: false,
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
            CardBody::Strike => const_card!(&CardProps::new(
                &[PlayEffect::Attack(6)],
                &[PlayEffect::Attack(9)],
                Cost::Fixed(1),
                true,
                CardType::Attack,
            )),
            CardBody::Bash => const_card!(&CardProps::new(
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
            )),
            CardBody::Defend => const_card!(&CardProps::new(
                &[PlayEffect::Block(5)],
                &[PlayEffect::Block(8)],
                Cost::Fixed(1),
                false,
                CardType::Skill,
            )),
            CardBody::Slimed => const_card!(
                &CardProps::new(
                    &[PlayEffect::MarkExhaust],
                    &[PlayEffect::MarkExhaust],
                    Cost::Fixed(1),
                    false,
                    CardType::Status,
                )
                .not_upgradable()
            ),
            CardBody::Anger => const_card!(&CardProps::new(
                &[PlayEffect::Attack(6), PlayEffect::AddCopyToDiscard],
                &[PlayEffect::Attack(8), PlayEffect::AddCopyToDiscard],
                Cost::Fixed(0),
                true,
                CardType::Attack,
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
            )),
            CardBody::BodySlam => const_card!(
                &CardProps::new(
                    &[PlayEffect::AttackEqualBlock],
                    &[PlayEffect::AttackEqualBlock],
                    Cost::Fixed(1),
                    true,
                    CardType::Attack,
                )
                .with_upgraded_cost(Cost::Fixed(0))
            ),
            CardBody::Clash => const_card!(&CardProps::new(
                &[PlayEffect::Attack(14)],
                &[PlayEffect::Attack(18)],
                Cost::Fixed(0),
                true,
                CardType::Attack,
            )),
            CardBody::Cleave => const_card!(&CardProps::new(
                &[PlayEffect::AttackAll(8)],
                &[PlayEffect::AttackAll(11)],
                Cost::Fixed(1),
                false,
                CardType::Attack,
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
            )),
            CardBody::Havoc => const_card!(
                &CardProps::new(
                    &[PlayEffect::PlayExhaustTop],
                    &[PlayEffect::PlayExhaustTop],
                    Cost::Fixed(1),
                    false,
                    CardType::Skill,
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
            )),
            CardBody::HeavyBlade => const_card!(&CardProps::new(
                &[PlayEffect::Attack(14)],
                &[PlayEffect::Attack(14)],
                Cost::Fixed(2),
                true,
                CardType::Attack,
            )),
            CardBody::IronWave => const_card!(&CardProps::new(
                &[PlayEffect::Block(5), PlayEffect::Attack(5)],
                &[PlayEffect::Block(7), PlayEffect::Attack(7)],
                Cost::Fixed(1),
                true,
                CardType::Attack,
            )),
            CardBody::SearingBlow => const_card!(&CardProps::new(
                &[PlayEffect::Attack(12)],
                &[PlayEffect::Attack(12)],
                Cost::Fixed(2),
                true,
                CardType::Attack,
            )),
            CardBody::PerfectedStrike => const_card!(&CardProps::new(
                &[PlayEffect::Attack(6)],
                &[PlayEffect::Attack(6)],
                Cost::Fixed(2),
                true,
                CardType::Attack,
            )),
            CardBody::PommelStrike => const_card!(&CardProps::new(
                &[PlayEffect::Attack(9), PlayEffect::Draw(1)],
                &[PlayEffect::Attack(10), PlayEffect::Draw(2)],
                Cost::Fixed(1),
                true,
                CardType::Attack,
            )),
            CardBody::ShrugItOff => const_card!(&CardProps::new(
                &[PlayEffect::Block(8), PlayEffect::Draw(1)],
                &[PlayEffect::Block(11), PlayEffect::Draw(1)],
                Cost::Fixed(1),
                false,
                CardType::Skill,
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
            )),
            CardBody::TwinStrike => const_card!(&CardProps::new(
                &[PlayEffect::Attack(5), PlayEffect::Attack(5)],
                &[PlayEffect::Attack(7), PlayEffect::Attack(7)],
                Cost::Fixed(1),
                true,
                CardType::Attack,
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
            )),
            CardBody::WildStrike => const_card!(&CardProps::new(
                &[
                    PlayEffect::Attack(12),
                    PlayEffect::ShuffleInStatus(CardBody::Wound),
                ],
                &[
                    PlayEffect::Attack(17),
                    PlayEffect::ShuffleInStatus(CardBody::Wound),
                ],
                Cost::Fixed(1),
                true,
                CardType::Attack,
            )),
            CardBody::Wound => const_card!(
                &CardProps::new(
                    &[PlayEffect::MarkExhaust],
                    &[PlayEffect::MarkExhaust],
                    Cost::Unplayable,
                    false,
                    CardType::Status,
                )
                .not_upgradable()
            ),
            CardBody::BattleTrance => const_card!(&CardProps::new(
                &[PlayEffect::Draw(3), PlayEffect::DebuffSelf(Debuff::NoDraw)],
                &[PlayEffect::Draw(4), PlayEffect::DebuffSelf(Debuff::NoDraw)],
                Cost::Fixed(0),
                false,
                CardType::Skill,
            )),
            CardBody::BloodForBlood => const_card!(
                &CardProps::new(
                    &[PlayEffect::Attack(18)],
                    &[PlayEffect::Attack(22)],
                    Cost::NumMinusHpLoss(4),
                    true,
                    CardType::Attack,
                )
                .with_upgraded_cost(Cost::NumMinusHpLoss(3))
            ),
            CardBody::Bloodletting => const_card!(&CardProps::new(
                &[PlayEffect::LoseHP(3), PlayEffect::GainEnergy(2)],
                &[PlayEffect::LoseHP(3), PlayEffect::GainEnergy(3)],
                Cost::Fixed(0),
                false,
                CardType::Skill,
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
            )),
            CardBody::Carnage => const_card!(
                &CardProps::new(
                    &[PlayEffect::Attack(20)],
                    &[PlayEffect::Attack(28)],
                    Cost::Fixed(2),
                    true,
                    CardType::Attack,
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
            )),
            CardBody::DarkEmbrace => const_card!(
                &CardProps::new(
                    &[PlayEffect::Buff(Buff::DarkEmbraceBuff)],
                    &[PlayEffect::Buff(Buff::DarkEmbraceBuff)],
                    Cost::Fixed(2),
                    false,
                    CardType::Power,
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
            )),
            CardBody::Dropkick => const_card!(&CardProps::new(
                &[PlayEffect::Attack(5), PlayEffect::DropkickDraw],
                &[PlayEffect::Attack(8), PlayEffect::DropkickDraw],
                Cost::Fixed(1),
                true,
                CardType::Attack,
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
            )),
            CardBody::Entrench => const_card!(
                &CardProps::new(
                    &[PlayEffect::DoubleBlock],
                    &[PlayEffect::DoubleBlock],
                    Cost::Fixed(2),
                    false,
                    CardType::Skill,
                )
                .with_upgraded_cost(Cost::Fixed(1))
            ),
            CardBody::Evolve => const_card!(&CardProps::new(
                &[PlayEffect::Buff(Buff::EvolveBuff(1))],
                &[PlayEffect::Buff(Buff::EvolveBuff(2))],
                Cost::Fixed(1),
                false,
                CardType::Power,
            )),
            CardBody::FeelNoPain => const_card!(&CardProps::new(
                &[PlayEffect::Buff(Buff::FNPBuff(3))],
                &[PlayEffect::Buff(Buff::FNPBuff(4))],
                Cost::Fixed(1),
                false,
                CardType::Power,
            )),
            CardBody::FireBreathing => const_card!(&CardProps::new(
                &[PlayEffect::Buff(Buff::FireBreathingBuff(6))],
                &[PlayEffect::Buff(Buff::FireBreathingBuff(10))],
                Cost::Fixed(1),
                false,
                CardType::Power,
            )),
            CardBody::FlameBarrier => const_card!(&CardProps::new(
                &[PlayEffect::Block(12), PlayEffect::Buff(Buff::TempSpikes(4))],
                &[PlayEffect::Block(16), PlayEffect::Buff(Buff::TempSpikes(6))],
                Cost::Fixed(2),
                false,
                CardType::Skill,
            )),
            CardBody::GhostlyArmor => const_card!(
                &CardProps::new(
                    &[PlayEffect::Block(10)],
                    &[PlayEffect::Block(13)],
                    Cost::Fixed(1),
                    false,
                    CardType::Skill,
                )
                .with_ethereal(Ethereal::Yes)
            ),
            CardBody::Hemokinesis => const_card!(&CardProps::new(
                &[PlayEffect::LoseHP(2), PlayEffect::Attack(15)],
                &[PlayEffect::LoseHP(2), PlayEffect::Attack(20)],
                Cost::Fixed(1),
                true,
                CardType::Attack,
            )),
            CardBody::InfernalBlade => const_card!(
                &CardProps::new(
                    &[PlayEffect::GenerateAttackInfernal],
                    &[PlayEffect::GenerateAttackInfernal],
                    Cost::Fixed(1),
                    false,
                    CardType::Skill,
                )
                .with_upgraded_cost(Cost::Fixed(0))
            ),
        };
    }
    pub const fn to_card(&self) -> Card {
        Card {
            cost: self.default_cost(),
            body: *self,
            assoc_data: CardAssoc::None,
            upgraded: false,
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
const fn attack_filter(props: &'static CardProps) -> bool {
    matches!(props.card_type, CardType::Attack)
}
pub const ATTACK_CARDS: &'static [CardBody] = filtered_cards!(attack_filter);

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
        self.body.props().upgradable && !self.upgraded
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
}
