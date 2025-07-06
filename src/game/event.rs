mod big_fish;
mod cleric;
mod dead_adventurer;
mod golden_idol;
mod living_wall;
mod shrooms;
mod scrap_ooze;
mod shining_light;
mod the_ssserpent;
mod world_of_goop;

use paste::paste;
use strum::VariantArray;

use crate::{
    game::{
        Choice, EventAction, Game,
        event::{
            big_fish::BigFish, cleric::Cleric, dead_adventurer::DeadAdventurer,
            golden_idol::GoldenIdol, shrooms::HypnotizingShrooms, living_wall::LivingWall,
            scrap_ooze::ScrapOoze, shining_light::ShiningLight, the_ssserpent::TheSsserpent,
            world_of_goop::WorldOfGoop
        },
    },
    rng::Rng,
};
/*
Event Generation works as follows:

There is first a weighted coin flip with 75% standard and 25% shrine.
If shrine if flipped and there are no more shrines left a standard event is generated.
Otherwise, a standard event is generated with a shrine as backup.
*/

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, VariantArray)]
pub enum EventNameOld {
    //Act 1 exclusive.
    //Neow,
    BigFish,
    Cleric,
    DeadAdventurer,
    GoldenIdol,
    HypnotizingShrooms,
    LivingWall,
    ScrapOoze,
    ShiningLight,
    TheSsserpent,
    WorldOfGoop,
    //WingStatue,
    //Shrines
    //BonfireSpirits,
    //GoldenShrine,
    //Lab,
    //MatchAndKeep,
    //OminousForge,
    //Purifier,
    //Transmogifier,
    //UpgradeShrine,
    //WeMeetAgain,
    //WheelOfChange,
    //Conditional Shrines
    //TheDivineFountain,
    //Duplicator,
    //Designer,
    //FaceTrader,
    //KnowingSkull,
    //Nloth,
    //Joust,
    //WomanInBlue,
    //I'm not including Secret Portal.
}

pub trait EventRoom {
    fn new(rng: &mut Rng) -> Self;
    fn get_actions(&self, game: &Game) -> Vec<EventAction>;
    fn take_action(self, game: &mut Game, action: EventAction) -> Choice;
    fn action_str(&self, game: &Game, action: EventAction) -> String;
    fn name(&self) -> &'static str;
}

macro_rules! event_array {
    ($($x:ident),*) => {
        paste!{
            #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
            pub enum EventName {
                $(
                    $x,
                )*
            }

            #[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
            pub enum Event {
                $(
                    $x($x),
                )*
            }

            impl EventName {
                fn new(&self, rng: &mut Rng) -> Event {
                    match &self {
                        $(
                            Self::$x => Event::$x($x::new(rng)),
                        )*
                    }
                }
            }

            impl Event {
                pub fn get_actions(&self, game: &Game) -> Vec<EventAction> {
                    match &self {
                        $(
                           Self::$x(event) => event.get_actions(game),
                        )*
                    }
                }

                pub fn take_action(self, game: &mut Game, action: EventAction) -> Choice {
                        match self {
                            $(
                                Self::$x(event) => event.take_action(game, action),
                            )*
                        }
                }

                pub fn action_str(&self, game: &Game, action: EventAction) -> String {
                        match &self {
                            $(
                                Self::$x(event) => event.action_str(game, action),
                            )*
                        }
                }

                pub fn name(&self) -> &'static str {
                        match &self {
                            $(
                                Self::$x(event) => event.name(),
                            )*
                        }
                }
            }
        }
    }
}

event_array!(
    BigFish,
    Cleric,
    DeadAdventurer,
    GoldenIdol,
    HypnotizingShrooms,
    LivingWall,
    ScrapOoze,
    ShiningLight,
    TheSsserpent,
    WorldOfGoop
);
