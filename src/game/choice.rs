use std::{fmt::Display, mem};

use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

use crate::{
    card::{CardBody, SelectCardEffect},
    fight::{Enemy, PlayCardContext},
    game::{Game, event::Event},
    relic::Relic,
    rng::Rng,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Getters, Serialize, Deserialize)]
pub struct ChoiceState {
    pub(super) game: Box<Game>,
    pub(super) choice: Choice,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SelectionPile {
    Hand,
    Discard,
    Exhaust,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
//Choose the i'th card

pub struct SelectCardAction(pub usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PlayCardAction {
    //Play the i'th card in hand
    PlayCard(u8),
    //End the turn
    EndTurn,
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChooseEnemyAction {
    //Target the i'th enemy
    pub enemy: u8,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MapStateAction {
    Jump(i32),
    Left,
    Forwards,
    Right,
}

//Choose the i'th choice in the event. The interpretation
//of this is event dependent.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EventAction(pub usize);

//Rest Site Actions
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RestSiteAction {
    Heal,
    Upgrade,
}

//The rewards
#[derive(Clone, Debug, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct Rewards {
    pub gold: i32,
    pub relics: SmallVec<[Relic; 1]>,
    pub card_choices: SmallVec<[SmallVec<[CardBody; 4]>; 1]>,
}

//Rest Site Actions
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RewardAction {
    TakeGold,
    TakeRelic(usize),
    TakeCard(usize, usize),
    Proceed,
}

//Rest Site Actions
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SelectDeckCardReason {
    Remove,
    Transform,
    Upgrade,
}

#[must_use]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub enum Choice {
    //See if this can be improved for more allocation reuse.
    PlayCardState(Vec<PlayCardAction>),
    ChooseEnemyState(Vec<ChooseEnemyAction>, usize),
    Win,
    Loss,
    MapState(Vec<MapStateAction>),
    SelectCardState(
        PlayCardContext,
        SelectCardEffect,
        Vec<SelectCardAction>,
        SelectionPile,
    ),
    Event(Event, Vec<EventAction>),
    SelectDeckCardState(SelectDeckCardReason, Vec<SelectCardAction>),
    RestSite(Vec<RestSiteAction>),
    Rewards(Rewards, Vec<RewardAction>),
}

impl ChoiceState {
    pub fn is_over(&self) -> bool {
        match self.choice {
            Choice::Win | Choice::Loss => true,
            _ => false,
        }
    }

    //This function clones the choice state to another Game. It
    //will still behave differently due to the Rng returning different
    //results. This can be used to simulate different outcomes
    pub fn clone_to_reseeded(&self, other: &mut ChoiceState) {
        *other.game = (*self.game).clone();
        other.game.rng = Rng::new();
        other.choice = self.choice.clone();
    }

    //This function handles an action being taken.
    pub fn take_action(&mut self, action_idx: usize) {
        let game = &mut *self.game;
        game.state_counter += 1;
        //The choice is set on the next line in the match statement.
        //The issue is that Rust won't let the program consume choice by value
        //even though it is overwritten. This is solved by swapping in a temporary value.
        self.choice = match mem::replace(&mut self.choice, Choice::Loss) {
            Choice::PlayCardState(play_card_actions) => {
                game.handle_play_card_action(play_card_actions[action_idx])
            }
            Choice::ChooseEnemyState(choose_enemy_actions, card_idx) => {
                game.handle_choose_enemy_action(card_idx, choose_enemy_actions[action_idx])
            }
            Choice::Win => {
                panic!("The game is won, no actions can be taken");
            }
            Choice::Loss => {
                panic!("The game is lost, no actions can be taken");
            }
            Choice::MapState(map_state_actions) => {
                game.handle_map_state_action(map_state_actions[action_idx])
            }
            Choice::SelectCardState(
                play_card_context,
                effect,
                select_card_actions,
                _selection_type,
            ) => {
                let action = select_card_actions[action_idx];
                game.handle_select_card_action(play_card_context, effect, action)
            }
            Choice::Event(event, actions) => {
                event.handle_action(&mut self.game, actions[action_idx])
            }
            Choice::SelectDeckCardState(reason, actions) => {
                let action = actions[action_idx];
                match reason {
                    SelectDeckCardReason::Remove => game.handle_remove_card_action(action.0),
                    SelectDeckCardReason::Transform => game.handle_transform_card_action(action.0),
                    SelectDeckCardReason::Upgrade => game.handle_upgrade_card_action(action.0),
                }
            }
            Choice::RestSite(rest_site_actions) => {
                game.handle_rest_site_action(rest_site_actions[action_idx])
            }
            Choice::Rewards(rewards, actions) => {
                let action = actions[action_idx];
                game.handle_reward_action(rewards, action)
            }
        };
    }

    pub fn num_actions(&self) -> usize {
        match &self.choice {
            Choice::PlayCardState(play_card_actions) => play_card_actions.len(),
            Choice::ChooseEnemyState(choose_enemy_actions, _) => choose_enemy_actions.len(),
            Choice::Win => 0,
            Choice::Loss => 0,
            Choice::MapState(map_state_actions) => map_state_actions.len(),
            Choice::SelectCardState(
                _play_card_context,
                _effect,
                select_card_actions,
                _selection_type,
            ) => select_card_actions.len(),
            Choice::Event(_event, event_actions) => event_actions.len(),
            Choice::SelectDeckCardState(_reason, actions) => actions.len(),
            Choice::RestSite(actions) => actions.len(),
            Choice::Rewards(_rewards, actions) => actions.len(),
        }
    }
}
