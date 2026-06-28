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
            pub struct Relics {
                $(
                    [<$x:lower>]: bool,
                )*
            }

            impl Relics {
                pub fn new() -> Self {
                    Relics {
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
            }
        }
    }
}

macro_rules! relic_segments {
    ($($common_all:ident),* ;
     $($uncommon_all:ident),* ;
     $($rare_all:ident),* ;
     $($boss_all:ident),* ;
     $($shop:ident),* ;
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
        }

        impl RelicPool {
            //TODO - add character specific relics.
            pub fn new(character: Character) -> Self {
                RelicPool {
                    common_relics: {vec![$(Relic::$common_all,)*]},
                    uncommon_relics: {vec![$(Relic::$uncommon_all,)*]},
                    rare_relics: {vec![$(Relic::$rare_all,)*]},
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
    Event,
}
impl RelicPool {
    pub fn get_relic(&mut self, rarity: RelicRarity) -> Relic {
        return Relic::Circlet;
    }
    pub fn get_random_tier_relic(&mut self, rng: &mut Rng) -> Relic {
        let sample = rng.sample_weighted(&[50, 33, 17]);
        let rarity = match sample {
            0 => RelicRarity::Common,
            1 => RelicRarity::Uncommon,
            2 => RelicRarity::Rare,
            _ => panic!("Invalid rarity returned by RNG"),
        };
        self.get_relic(rarity)
    }
}
