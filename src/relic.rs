use serde::{Deserialize, Serialize};

use crate::{game::Character, rng::Rng};
use paste::paste;

macro_rules! make_relics {
    ($($x:ident),* $(,)?) => {
        paste!{
            #[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
            pub enum Relic {
                $(
                    $x,
                )*
            }

            #[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
            pub struct RelicBar {
                $(
                    pub [<$x:lower>]: bool,
                )*
            }

            impl RelicBar {
                pub fn new() -> Self {
                    RelicBar {
                        $(
                            [<$x:lower>] : false,
                        )*
                    }
                }
                pub fn add(&mut self, relic: Relic) {
                    match relic {
                        $(
                            Relic::$x => {self.[<$x:lower>] = true;}
                        )*
                    }
                }
                pub fn has_relic(&self, relic: Relic) -> bool {
                    match relic {
                        $(
                            Relic::$x => self.[<$x:lower>],
                        )*
                    }
                }
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Relics {
    pub bar: RelicBar,
    pub pool: RelicPool,
}

impl Relics {
    pub fn new(character: Character) -> Self {
        Self {
            bar: RelicBar::new(),
            pool: RelicPool::new(character),
        }
    }
    pub fn add(&mut self, relic: Relic) {
        self.bar.add(relic);
    }
    pub fn has_relic(&self, relic: Relic) -> bool {
        self.bar.has_relic(relic)
    }
}

macro_rules! relic_segments {
    ($($common_all:ident),*;
     $($uncommon_all:ident),*;
     $($rare_all:ident),*;
     $($boss_all:ident),*;
     $($shop:ident),*;
     $($other:ident),*
    ) => {
        make_relics!(
            //Circlet is a placeholder relic that does nothing if you have all relics.
            Circlet,
            $($common_all,)*
            $($uncommon_all,)*
            $($rare_all,)*
            $($boss_all,)*
            $($shop,)*
            $($other,)*
        );

        #[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub struct RelicPool {
            common_relics: Vec<Relic>,
            uncommon_relics: Vec<Relic>,
            rare_relics: Vec<Relic>,
            boss_relics: Vec<Relic>,
            shop_relics: Vec<Relic>,
        }

        impl RelicPool {
            pub fn new(character: Character) -> Self {
                RelicPool {
                    common_relics: {
                        let mut relics = vec![$(Relic::$common_all,)*];
                        match character {
                            Character::IRONCLAD => {
                                relics.append(&mut vec![Relic::RedSkull]);
                            },
                            Character::SILENT => {
                                relics.append(&mut vec![Relic::SneckoSkull]);
                            },
                            Character::DEFECT => {
                                relics.append(&mut vec![Relic::DataDisk]);
                            },
                            Character::WATCHER => {
                                relics.append(&mut vec![Relic::Damaru]);
                            }
                        }
                        relics
                    },
                    uncommon_relics: {
                        let mut relics = vec![$(Relic::$uncommon_all,)*];
                        match character {
                            Character::IRONCLAD => {
                                relics.append(&mut vec![Relic::PaperPhrog, Relic::SelfFormingClay]);
                            },
                            Character::SILENT => {
                                relics.append(&mut vec![Relic::NinjaScroll, Relic::PaperKrane]);
                            },
                            Character::DEFECT => {
                                relics.append(&mut vec![Relic::GoldPlatedCables, Relic::SymbioticVirus]);
                            },
                            Character::WATCHER => {
                                relics.append(&mut vec![Relic::Duality, Relic::TeardropLocket]);
                            }
                        }
                        relics
                    },
                    rare_relics: {
                        let mut relics = vec![$(Relic::$rare_all,)*];
                        match character {
                            Character::IRONCLAD => {
                                relics.append(&mut vec![Relic::ChampionBelt, Relic::CharonsAshes, Relic::MagicFlower]);
                            },
                            Character::SILENT => {
                                relics.append(&mut vec![Relic::TheSpecimen, Relic::Tingsha, Relic::ToughBandages]);
                            },
                            Character::DEFECT => {
                                relics.append(&mut vec![Relic::EmotionChip]);
                            },
                            Character::WATCHER => {
                                relics.append(&mut vec![Relic::CloakClasp, Relic::GoldenEye]);
                            }
                        }
                        relics
                    },
                    boss_relics: {
                        let mut relics = vec![$(Relic::$boss_all,)*];
                        match character {
                            Character::IRONCLAD => {
                                relics.append(&mut vec![Relic::BlackBlood, Relic::MarkofPain, Relic::RunicCube]);
                            },
                            Character::SILENT => {
                                relics.append(&mut vec![Relic::RingoftheSerpent, Relic::WristBlade, Relic::HoveringKite]);
                            },
                            Character::DEFECT => {
                                relics.append(&mut vec![Relic::FrozenCore, Relic::Inserter, Relic::NuclearBattery]);
                            },
                            Character::WATCHER => {
                                relics.append(&mut vec![Relic::HolyWater, Relic::VioletLotus]);
                            }
                        }
                        relics
                    },
                    shop_relics: {
                        let mut relics = vec![$(Relic::$shop,)*];
                        match character {
                            Character::IRONCLAD => {
                                relics.append(&mut vec![Relic::Brimstone]);
                            },
                            Character::SILENT => {
                                relics.append(&mut vec![Relic::TwistedFunnel]);
                            },
                            Character::DEFECT => {
                                relics.append(&mut vec![Relic::RunicCapacitor]);
                            },
                            Character::WATCHER => {
                                relics.append(&mut vec![Relic::Melange]);
                            }
                        }
                        relics
                    },
                }
            }
        }
    }
}

relic_segments!( 
    //Common - All chars
    Akabeko,
    Anchor,
    AncientTeaSet,
    ArtofWar,
    BagofMarbles,
    BagofPreparation,
    BloodVial,
    BronzeScales,
    CentennialPuzzle,
    CeramicFish,
    DreamCatcher,
    HappyFlower,
    JuzuBracelet,
    Lantern,
    MawBank,
    MealTicket,
    Nunchaku,
    OddlySmoothStone,
    Omamori,
    Orichalum,
    PenNib,
    PotionBelt,
    PreservedInsect,
    RegalPillow,
    SmilingMask,
    Strawberry,
    TheBoot,
    TinyChest,
    ToyOrnithopter,
    Vajra,
    WarPaint,
    Whetstone;
    //Uncommon - All chars
    BlueCandle,
    BottledFlame,
    BottledLightning,
    BottledTornado,
    DarkstonePeriapt,
    EternalFeather,
    FrozenEgg,
    GremlinHorn,
    HornCleat,
    InkBottle,
    Kunai,
    LetterOpener,
    Matryoshka,
    MeatontheBone,
    MercuryHourglass,
    MoltenEgg,
    MummifiedHand,
    OrnamentalFan,
    Pantograph,
    Pear,
    QuestionCard,
    Shuriken,
    SingingBowl,
    StrikeDummy,
    Sundial,
    TheCourier,
    ToxicEgg,
    WhiteBeastStatue;
    //Rare - All chars
    BirdFacedUrn,
    Calipers,
    CaptainsWheel,
    DeadBranch,
    DuVuDoll,
    FossilizedHelix,
    GamblingChip,
    Ginger,
    Girya,
    IceCream,
    IncenseBurner,
    LizardTail,
    Mango,
    OldCoin,
    PeacePipe,
    Pocketwatch,
    PrayerWheel,
    Shovel,
    StoneCalendar,
    ThreadandNeedle,
    Torii,
    TungstenRod,
    Turnip,
    UnceasingTop,
    WingBoots;
    //Boss - All chars
    Astrolabe,
    BlackStar,
    BustedCrown,
    CallingBell,
    CoffeeDripper,
    CursedKey,
    Ectoplasm,
    EmptyCage,
    FusionHammer,
    PandorasBox,
    PhilosophersStone,
    RunicDome,
    RunicPyramid,
    SacredBark,
    SlaversCollar,
    SneckoEye,
    Sozu,
    TinyHouse,
    VelvetChoker;
    //Shop
    Cauldron,
    ChemicalX,
    ClockworkSouvenir,
    DollysMirror,
    FrozenEye,
    HandDrill,
    LeesWaffle,
    MedicalKit,
    MembershipCard,
    OrangePellets,
    Orrery,
    PrismaticShard,
    SlingofCourage,
    StrangeSpoon,
    TheAbacus,
    Toolbox;
    //Other - A mix of event relics and character specific relics
    BloodyIdol,
    CultistHeadpiece,
    Enrichidon,
    FaceofCleric,
    GoldenIdol,
    GremlinVisage,
    MarkoftheBloom,
    MutagenicStrength,
    NlothsGift,
    NlothsHungryFace,
    Necronomicon,
    NeowsLament,
    NilrysCodex,
    OddMushroom,
    RedMask,
    SpiritPoop,
    SsserpentHead,
    WarpedTongs,
    BurningBlood,
    RingoftheSnake,
    CrackedCore,
    PureWater,
    RedSkull,
    SneckoSkull,
    DataDisk,
    Damaru,
    PaperPhrog,
    SelfFormingClay,
    NinjaScroll,
    PaperKrane,
    GoldPlatedCables,
    SymbioticVirus,
    Duality,
    TeardropLocket,
    ChampionBelt,
    CharonsAshes,
    MagicFlower,
    TheSpecimen,
    Tingsha,
    ToughBandages,
    EmotionChip,
    CloakClasp,
    GoldenEye,
    Brimstone,
    TwistedFunnel,
    RunicCapacitor,
    Melange,
    BlackBlood,
    MarkofPain,
    RunicCube,
    RingoftheSerpent,
    WristBlade,
    HoveringKite,
    FrozenCore,
    Inserter,
    NuclearBattery,
    HolyWater,
    VioletLotus);

pub enum RelicRarity {
    Common,
    Uncommon,
    Rare,
    Shop,
    Boss,
}

fn random_or_circlet(rng: &mut Rng, relics: &mut Vec<Relic>) -> Relic {
    rng.try_sample(relics.len())
        .map_or(Relic::Circlet, |idx| relics.swap_remove(idx))
}
impl RelicPool {
    pub fn get_relic(&mut self, rng: &mut Rng, rarity: RelicRarity) -> Relic {
        match rarity {
            RelicRarity::Common => random_or_circlet(rng, &mut self.common_relics),
            RelicRarity::Uncommon => random_or_circlet(rng, &mut self.uncommon_relics),
            RelicRarity::Rare => random_or_circlet(rng, &mut self.rare_relics),
            RelicRarity::Shop => random_or_circlet(rng, &mut self.shop_relics),
            RelicRarity::Boss => random_or_circlet(rng, &mut self.boss_relics),
        }
    }
    pub fn get_random_tier_relic(&mut self, rng: &mut Rng) -> Relic {
        let sample = rng.sample_weighted(&[50, 33, 17]);
        let rarity = match sample {
            0 => RelicRarity::Common,
            1 => RelicRarity::Uncommon,
            2 => RelicRarity::Rare,
            _ => panic!("Invalid rarity returned by RNG"),
        };
        self.get_relic(rng, rarity)
    }
}
