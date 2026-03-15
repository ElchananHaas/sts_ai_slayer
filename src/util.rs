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
#[derive(Serialize, Deserialize)]
pub struct GameLog {
    chosen_state: Vec<ChosenState>,
    terminal_state: ChoiceState,
}
