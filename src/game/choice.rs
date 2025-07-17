use crate::{card::SelectCardEffect, fight::PlayCardContext, game::{event::Event, Game}};


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum SelectionPile {
    Hand,
    Discard,
    Exhaust,
}


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum SelectCardAction {
    //Choose the i'th card
    ChooseCard(usize),
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ChoiceState<'a> {
    pub(super) game: &'a mut Game,
    pub(super) choice: Choice,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum PlayCardAction {
    //Play the i'th card in hand
    PlayCard(u8),
    //End the turn
    EndTurn,
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ChooseEnemyAction {
    //Target the i'th enemy
    pub enemy: u8,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum MapStateAction {
    Jump(i32),
    Left,
    Forwards,
    Right,
}

//Choose the i'th choice in the event. The interpretation
//of this is event dependent.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct EventAction(pub usize);

//Remove the i'th card in the deck.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RemoveCardAction(pub usize);

//Transform the i'th card in the deck.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct TransformCardAction(pub usize);

//Upgrade the i'th card in the deck.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct UpgradeCardAction(pub usize);


#[must_use]
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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
    RemoveCardState(Vec<RemoveCardAction>),
    TransformCardState(Vec<TransformCardAction>),
    UpgradeCardState(Vec<UpgradeCardAction>),
}

impl<'a> ChoiceState<'a> {
    pub fn is_over(&self) -> bool {
        match self.choice {
            Choice::Win | Choice::Loss => true,
            _ => false,
        }
    }

    //This function clones the choice state to another Game. It
    //will still behave differently due to the Rng returning different
    //results. This can be used to simulate different outcomes
    pub fn clone_to<'b>(&self, game: &'b mut Game) -> ChoiceState<'b> {
        *game = self.game.clone();
        return ChoiceState {
            game: game,
            choice: self.choice.clone(),
        };
    }

    //This function handles an action being taken.
    pub fn take_action(&mut self, action_idx: usize) {
        let game = &mut *self.game;
        //The choice is set on the next line in the match statement.
        //The issue is that Rust won't let the program consume choice by value
        //even though it is overwritten. This is solved by swapping in a temporary value.
        let mut choice = Choice::Loss;
        std::mem::swap(&mut choice, &mut self.choice);
        self.choice = match choice {
            Choice::PlayCardState(play_card_actions) => {
                let action = play_card_actions[action_idx];
                game.take_play_card_action(action)
            }
            Choice::ChooseEnemyState(choose_enemy_actions, card_idx) => {
                let action = choose_enemy_actions[action_idx];
                let card_idx = card_idx;
                game.take_choose_enemy_action(card_idx, action)
            }
            Choice::Win => {
                panic!("The game is won, no actions can be taken");
            }
            Choice::Loss => {
                panic!("The game is lost, no actions can be taken");
            }
            Choice::MapState(map_state_actions) => {
                let action = map_state_actions[action_idx];
                game.take_map_state_action(action)
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
            Choice::Event(mut event, actions) => {
                event.take_action(&mut self.game, actions[action_idx])
            }
            Choice::RemoveCardState(actions) => game.handle_remove_card_action(actions[action_idx]),
            Choice::TransformCardState(actions) => {
                game.handle_transform_card_action(actions[action_idx])
            }
            Choice::UpgradeCardState(actions) => {
                game.handle_upgrade_card_action(actions[action_idx])
            }
        }
    }

    //Outside the game only immutable access should be granted.
    pub fn get_choice(&self) -> &Choice {
        &self.choice
    }

    //Outside the game only immutable access should be granted.
    pub fn get_game(&self) -> &Game {
        &self.game
    }

    pub fn action_str(&self, action_idx: usize) -> String {
        match &self.choice {
            Choice::PlayCardState(play_card_actions) => {
                let action = play_card_actions[action_idx];
                match action {
                    PlayCardAction::PlayCard(card_idx) => {
                        format!("{:?}", self.game.fight.hand[card_idx as usize].body)
                    }
                    PlayCardAction::EndTurn => "End Turn".to_owned(),
                }
            }
            Choice::ChooseEnemyState(choose_enemy_actions, _) => {
                format!(
                    "Target {:?}",
                    self.game.fight.enemies[choose_enemy_actions[action_idx].enemy as usize]
                        .as_ref()
                        .map_or("", |enemy| enemy.name)
                )
            }
            Choice::Win => {
                panic!("Win state has no actions.")
            }
            Choice::Loss => {
                panic!("Loss state has no actions.")
            }
            Choice::MapState(map_state_actions) => {
                format!("Proceed by {:?}", map_state_actions[action_idx])
            }
            Choice::SelectCardState(
                _play_card_context,
                _effect,
                select_card_actions,
                selection_type,
            ) => {
                let action = select_card_actions[action_idx];
                match selection_type {
                    SelectionPile::Hand => match action {
                        SelectCardAction::ChooseCard(choice) => {
                            format!("Select {:?}", self.game.fight.hand[choice as usize].body)
                        } //SelectCardAction::None => "No Selection".to_owned(),
                    },
                    SelectionPile::Discard => match action {
                        SelectCardAction::ChooseCard(choice) => {
                            format!(
                                "Select {:?}",
                                self.game.fight.discard_pile[choice as usize].body
                            )
                        }
                    },
                    SelectionPile::Exhaust => match action {
                        SelectCardAction::ChooseCard(choice) => {
                            format!("Select {:?}", self.game.fight.exhaust[choice as usize].body)
                        }
                    },
                }
            }
            Choice::Event(event, event_actions) => {
                event.action_str(&self.game, event_actions[action_idx])
            }
            Choice::RemoveCardState(remove_card_actions) => {
                let card = &self.game.base_deck[remove_card_actions[action_idx].0];
                format!("Remove {:?}", card.body)
            }
            Choice::TransformCardState(transform_card_actions) => {
                let card = &self.game.base_deck[transform_card_actions[action_idx].0];
                format!("Transform {:?}", card.body)
            }
            Choice::UpgradeCardState(transform_card_actions) => {
                let card = &self.game.base_deck[transform_card_actions[action_idx].0];
                format!("Upgrade {:?}", card.body)
            }
        }
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
            Choice::RemoveCardState(actions) => actions.len(),
            Choice::TransformCardState(actions) => actions.len(),
            Choice::UpgradeCardState(actions) => actions.len(),
        }
    }
}