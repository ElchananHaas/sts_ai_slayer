use crate::{
    fight::{Enemy, EnemyAction, EnemyName, Fight},
    rng::Rng,
};

pub mod blue_slaver;
pub mod cultist;
pub mod fungi_beast;
pub mod green_louse;
pub mod gremlin_fat;
pub mod gremlin_mad;
pub mod gremlin_nob;
pub mod gremlin_shield;
pub mod gremlin_sneaky;
pub mod gremlin_wizard;
pub mod jaw_worm;
pub mod lagavulin;
pub mod large_black_slime;
pub mod large_green_slime;
pub mod looter;
pub mod med_black_slime;
pub mod med_green_slime;
pub mod red_louse;
pub mod red_slaver;
pub mod sentry;
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

pub fn behavior(
    rng: &mut Rng,
    fight: &Fight,
    enemy: &Enemy,
    state: u32,
) -> (u32, &'static [EnemyAction]) {
    match enemy.name {
        EnemyName::BlueSlaver => blue_slaver::ai(rng, fight, enemy, state),
        EnemyName::Cultist => cultist::ai(rng, fight, enemy, state),
        EnemyName::FungiBeast => fungi_beast::ai(rng, fight, enemy, state),
        EnemyName::GreenLouse => green_louse::ai(rng, fight, enemy, state),
        EnemyName::GremlinFat => gremlin_fat::ai(rng, fight, enemy, state),
        EnemyName::GremlinMad => gremlin_mad::ai(rng, fight, enemy, state),
        EnemyName::GremlinNob => gremlin_nob::ai(rng, fight, enemy, state),
        EnemyName::GremlinShield => gremlin_shield::ai(rng, fight, enemy, state),
        EnemyName::GremlinSneaky => gremlin_sneaky::ai(rng, fight, enemy, state),
        EnemyName::GremlinWizard => gremlin_wizard::ai(rng, fight, enemy, state),
        EnemyName::JawWorm => jaw_worm::ai(rng, fight, enemy, state),
        EnemyName::Lagavulin => lagavulin::ai(rng, fight, enemy, state),
        EnemyName::LargeBlackSlime => large_black_slime::ai(rng, fight, enemy, state),
        EnemyName::LargeGreenSlime => large_green_slime::ai(rng, fight, enemy, state),
        EnemyName::Looter => looter::ai(rng, fight, enemy, state),
        EnemyName::MedBlackSlime => med_black_slime::ai(rng, fight, enemy, state),
        EnemyName::MedGreenSlime => med_green_slime::ai(rng, fight, enemy, state),
        EnemyName::RedLouse => red_louse::ai(rng, fight, enemy, state),
        EnemyName::RedSlaver => red_slaver::ai(rng, fight, enemy, state),
        EnemyName::Sentry => sentry::ai(rng, fight, enemy, state),
        EnemyName::SmallBlackSlime => small_black_slime::ai(rng, fight, enemy, state),
        EnemyName::SmallGreenSlime => small_green_slime::ai(rng, fight, enemy, state),
    }
}
