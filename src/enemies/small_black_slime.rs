use crate::{
    enemies::{StateEntry, uniform_inclusive, weighted_transition},
    fight::{Enemy, EnemyAction, EnemyBuffs, EnemyDebuffs, Fight},
    rng::Rng,
};

const ENEMY_NAME: &'static str = "Black Slime [S]";
pub fn generate_small_black_slime(rng: &mut Rng) -> Enemy {
    let hp = uniform_inclusive(rng, 10, 14);
    fn ai(rng: &mut Rng, _: &Fight, _: &Enemy, state: u32) -> (u32, &'static [EnemyAction]) {
        // States are
        // 0) Playing Attack
        const ENEMY_TABLE: &'static [StateEntry] = &[StateEntry {
            actions: &[EnemyAction::Attack(5)],
            new_states: &[0],
            weights: &[1],
        }];
        return weighted_transition(rng, state, ENEMY_TABLE);
    }
    Enemy {
        name: ENEMY_NAME,
        ai_state: 0,
        behavior: ai,
        hp,
        max_hp: hp,
        buffs: EnemyBuffs::default(),
        debuffs: EnemyDebuffs::default(),
        ..Enemy::default()
    }
}
