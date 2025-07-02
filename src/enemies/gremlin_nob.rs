use crate::{
    card::{Buff, Debuff},
    enemies::{StateEntry, uniform_inclusive, weighted_transition},
    fight::{Enemy, EnemyAction, EnemyBuffs, EnemyDebuffs, Fight},
    rng::Rng,
};

const ENEMY_NAME: &'static str = "Gremlin Nob";
pub fn generate_gremlin_nob(rng: &mut Rng) -> Enemy {
    let hp = uniform_inclusive(rng, 82, 86);
    fn ai(rng: &mut Rng, _: &Fight, _: &Enemy, state: u32) -> (u32, &'static [EnemyAction]) {
        const ENEMY_TABLE: &'static [StateEntry] = &[
            StateEntry {
                actions: &[EnemyAction::Buff(Buff::Enrage(2))],
                new_states: &[1, 2],
                weights: &[1, 2],
            },
            StateEntry {
                actions: &[
                    EnemyAction::Attack(6),
                    EnemyAction::Debuff(Debuff::Vulnerable(2)),
                ],
                new_states: &[1, 2],
                weights: &[1, 2],
            },
            StateEntry {
                actions: &[EnemyAction::Attack(14)],
                new_states: &[1, 3],
                weights: &[1, 2],
            },
            StateEntry {
                actions: &[EnemyAction::Attack(14)],
                new_states: &[1],
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
