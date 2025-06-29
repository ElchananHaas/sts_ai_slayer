mod big_fish;
mod cleric;

use strum::VariantArray;

use crate::game::{Choice, EventAction, Game, event::big_fish::BigFish};
/*
Event Generation works as follows:

There is first a weighted coin flip with 75% standard and 25% shrine.
If shrine if flipped and there are no more shrines left a standard event is generated.
Otherwise, a standard event is generated with a shrine as backup.
*/

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, VariantArray)]
pub enum Event {
    //Act 1 exclusive.
    //Neow,
    BigFish,
    //Cleric,
    //DeadAdventurer,
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
    //I'm not inluding Secret Portal.
}

pub trait EventData {
    fn get_actions(&self, game: &Game) -> Vec<EventAction>;
    fn take_action(&self, game: &mut Game, action: EventAction) -> Choice;
    fn action_str(&self, game: &Game, action: EventAction) -> String;
    fn name(&self) -> &'static str;
}

impl Event {
    pub fn data(&self) -> impl EventData {
        match self {
            Event::BigFish => BigFish,
        }
    }
}
