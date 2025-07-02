mod big_fish;
mod cleric;
mod dead_adventurer;

use paste::paste;
use strum::VariantArray;

use crate::game::{
    Choice, EventAction, Game,
    event::{big_fish::BigFish, cleric::Cleric, dead_adventurer::DeadAdventurer},
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
    //GoldenIdol,
    //HypnotizingShrooms,
    //LivingWall,
    //ScrapOoze,
    //ShiningLight,
    //TheSsserpent,
    //WorldOfGoop,
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

pub trait EventData {
    fn get_actions(&self, game: &Game) -> Vec<EventAction>;
    fn take_action(&self, game: &mut Game, action: EventAction) -> Choice;
    fn action_str(&self, game: &Game, action: EventAction) -> String;
    fn name(&self) -> &'static str;
}

macro_rules! event_array {
    ($($x:expr),*) => {
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

            impl EventData for EventName {
                fn get_actions(&self, game: &Game) -> Vec<EventAction> {
                    match self {
                        $(
                            Self::$x => $x.get_actions(game),
                        )*
                    }
                }

                fn take_action(&self, game: &mut Game, action: EventAction) -> Choice {
                        match self {
                            $(
                                Self::$x => $x.take_action(game, action),
                            )*
                        }
                }

                fn action_str(&self, game: &Game, action: EventAction) -> String {
                        match self {
                            $(
                                Self::$x => $x.action_str(game, action),
                            )*
                        }
                }

                fn name(&self) -> &'static str {
                        match self {
                            $(
                                Self::$x => $x.name(),
                            )*
                        }
                }
            }
        }
    }
}

event_array!(BigFish, Cleric, DeadAdventurer);
