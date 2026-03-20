use serde::{Deserialize, Serialize};

use crate::card::Card;

pub fn insert_sorted(card: Card, vec: &mut Vec<Card>) {
    let pos = vec.binary_search(&card).unwrap_or_else(|e| e);
    vec.insert(pos, card);
}

//This represents a game state and the action taken in that state.
#[derive(Serialize, Deserialize)]
pub struct GameLog {
    seed: [u8; 32],
    actions: Vec<usize>,
}

impl GameLog {
    pub fn new(seed: [u8; 32]) -> Self {
        Self {
            seed,
            actions: Vec::new(),
        }
    }
    pub fn push(&mut self, action: usize) {
        self.actions.push(action);
    }
}
