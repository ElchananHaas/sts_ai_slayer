use crate::{
    enemies::{StateEntry, uniform_inclusive, weighted_transition},
    fight::{Enemy, EnemyAction, EnemyBuffs, EnemyDebuffs, Fight},
    rng::Rng,
};

const ENEMY_NAME: &'static str = "Looter";
pub fn generate_looter(rng: &mut Rng) -> Enemy {
    let hp = uniform_inclusive(rng, 8, 12);
    fn ai(rng: &mut Rng, _: &Fight, _: &Enemy, state: u32) -> (u32, &'static [EnemyAction]) {
        // States are
        // 0) Playing Attack
        // 1) Debuff
        const ENEMY_TABLE: &'static [StateEntry] = &[
            StateEntry {
                actions: &[EnemyAction::Attack(10), EnemyAction::StealGold(15)],
                new_states: &[1],
                weights: &[1],
            },
            StateEntry {
                actions: &[EnemyAction::Attack(10), EnemyAction::StealGold(15)],
                new_states: &[2, 3],
                weights: &[1, 1],
            },
            StateEntry {
                actions: &[EnemyAction::Attack(12), EnemyAction::StealGold(15)],
                new_states: &[3],
                weights: &[1],
            },
            StateEntry {
                actions: &[EnemyAction::Block(6)],
                new_states: &[4],
                weights: &[1],
            },
            StateEntry {
                actions: &[EnemyAction::Escape],
                new_states: &[5],
                weights: &[1],
            },
        ];
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
        block: 0,
    }
}
