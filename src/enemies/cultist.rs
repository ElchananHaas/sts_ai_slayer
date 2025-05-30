use crate::{
    card::Buff,
    enemies::{StateEntry, uniform_inclusive, weighted_transition},
    fight::{Enemy, EnemyAction, EnemyBuffs, EnemyDebuffs, Fight},
    rng::Rng,
};

const ENEMY_NAME: &'static str = "Cultist";
pub fn generate_cultist(rng: &mut Rng) -> Enemy {
    let hp = uniform_inclusive(rng, 48, 54);
    fn ai(rng: &mut Rng, _: &Fight, _: &Enemy, state: u32) -> (u32, &'static [EnemyAction]) {
        // States are
        // 0) Buff
        // 1) Attack for 6
        const ENEMY_TABLE: &'static [StateEntry] = &[
            StateEntry {
                actions: &[EnemyAction::Buff(Buff::RitualSkipFirst(3))],
                new_states: &[1],
                weights: &[1],
            },
            StateEntry {
                actions: &[EnemyAction::Attack(6)],
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
