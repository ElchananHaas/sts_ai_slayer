use crate::{fight::EnemyAction, rng::Rng};

pub mod blue_slaver;
pub mod cultist;
pub mod fungi_beast;
pub mod green_louse;
pub mod gremlin_fat;
pub mod gremlin_mad;
mod gremlin_nob;
pub mod gremlin_shield;
pub mod gremlin_sneaky;
pub mod gremlin_wizard;
pub mod jaw_worm;
pub mod large_black_slime;
pub mod large_green_slime;
pub mod looter;
pub mod med_black_slime;
pub mod med_green_slime;
pub mod red_louse;
pub mod red_slaver;
pub mod small_black_slime;
pub mod small_green_slime;

struct StateEntry {
    actions: &'static [EnemyAction],
    //The first entry is the new state. The second entry is the weight.
    new_states: &'static [u32],
    weights: &'static [u32],
}

fn weighted_transition(
    rng: &mut Rng,
    state: u32,
    entries: &'static [StateEntry],
) -> (u32, &'static [EnemyAction]) {
    let entry = &entries[state as usize];
    let new_idx = rng.sample_weighted(entry.weights);
    (entry.new_states[new_idx], entry.actions)
}

fn uniform_inclusive(rng: &mut Rng, min: i32, max: i32) -> i32 {
    min + rng.sample_i32(max - min + 1)
}
