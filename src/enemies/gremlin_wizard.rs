use crate::{
    enemies::{StateEntry, uniform_inclusive, weighted_transition},
    fight::{Enemy, EnemyAction, EnemyBuffs, EnemyDebuffs, EnemyName, Fight},
    rng::Rng,
};

pub fn ai(rng: &mut Rng, _: &Fight, _: &Enemy, state: u32) -> (u32, &'static [EnemyAction]) {
    // States are
    // 0) Charge Up Attack
    // 0) Charge Up Attack (Starting state)
    // 0) Charge Up Attack
    // 0) Attack
    const ENEMY_TABLE: &'static [StateEntry] = &[
        StateEntry {
            actions: &[],
            new_states: &[1],
            weights: &[1],
        },
        StateEntry {
            actions: &[],
            new_states: &[2],
            weights: &[1],
        },
        StateEntry {
            actions: &[],
            new_states: &[3],
            weights: &[1],
        },
        StateEntry {
            actions: &[EnemyAction::Attack(25)],
            new_states: &[0],
            weights: &[1],
        },
    ];
    return weighted_transition(rng, state, ENEMY_TABLE);
}
pub fn generate_wizard_gremlin(rng: &mut Rng) -> Enemy {
    let hp = uniform_inclusive(rng, 23, 25);
    Enemy {
        name: EnemyName::GremlinWizard,
        ai_state: 1,
        hp,
        max_hp: hp,
        buffs: EnemyBuffs::default(),
        debuffs: EnemyDebuffs::default(),
        block: 0,
    }
}
