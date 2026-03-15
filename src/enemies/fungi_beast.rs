use crate::{
    card::Buff,
    enemies::{StateEntry, uniform_inclusive, weighted_transition},
    fight::{Enemy, EnemyAction, EnemyBuffs, EnemyDebuffs, EnemyName, Fight},
    rng::Rng,
};

pub fn ai(rng: &mut Rng, _: &Fight, _: &Enemy, state: u32) -> (u32, &'static [EnemyAction]) {
    const ENEMY_TABLE: &'static [StateEntry] = &[
        StateEntry {
            actions: &[EnemyAction::Attack(6)],
            new_states: &[1, 2],
            weights: &[6, 4],
        },
        StateEntry {
            actions: &[EnemyAction::Attack(6)],
            new_states: &[2],
            weights: &[1],
        },
        StateEntry {
            actions: &[EnemyAction::Buff(Buff::Strength(3))],
            new_states: &[0],
            weights: &[1],
        },
    ];
    return weighted_transition(rng, state, ENEMY_TABLE);
}
pub fn generate_fungi_beast(rng: &mut Rng) -> Enemy {
    let hp = uniform_inclusive(rng, 22, 28);
    let mut buffs = EnemyBuffs::default();
    buffs.spore_cloud = 2;
    Enemy {
        name: EnemyName::FungiBeast,
        ai_state: rng.sample_weighted(&[6, 0, 4]) as u32,
        hp,
        max_hp: hp,
        buffs,
        debuffs: EnemyDebuffs::default(),
        block: 0,
    }
}
