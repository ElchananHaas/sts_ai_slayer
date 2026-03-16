use std::mem;

use serde::{Deserialize, Serialize};

use crate::{card::Card, game::choice::ChoiceState};

pub fn insert_sorted(card: Card, vec: &mut Vec<Card>) {
    let pos = vec.binary_search(&card).unwrap_or_else(|e| e);
    vec.insert(pos, card);
}

#[derive(Serialize, Deserialize)]
struct ChosenState {
    state: ChoiceState,
    choice: usize,
}
//This represents a game state and the action taken in that state.
#[derive(Serialize, Deserialize)]
pub struct GameLog {
    chosen_states: Vec<ChosenState>,
    terminal_state: ChoiceState,
}

impl GameLog {
    pub fn new(state: ChoiceState) -> Self {
        Self {
            chosen_states: Vec::new(),
            terminal_state: state,
        }
    }
    pub fn push(&mut self, action: usize, new_state: ChoiceState) {
        let current_state = mem::replace(&mut self.terminal_state, new_state);
        self.chosen_states.push(ChosenState {
            state: current_state,
            choice: action,
        });
    }
}
