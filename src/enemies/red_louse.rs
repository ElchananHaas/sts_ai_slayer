use crate::{
    card::Buff,
    enemies::{uniform_inclusive, weighted_transition, StateEntry},
    fight::{Enemy, EnemyAction, EnemyBuffs, EnemyDebuffs, Fight},
    rng::Rng,
};

const ENEMY_NAME: &'static str = "Red Louse";
pub fn generate_red_louse(rng: &mut Rng) -> Enemy {
    let hp = uniform_inclusive(rng, 10, 15);
    fn ai(rng: &mut Rng, _: &Fight, _: &Enemy, state: u32) -> (u32, &'static [EnemyAction]) {
        // States are
        // 0) Playing Attack
        // 1) Playing Attack, same move prior turn.
        // 2) Playing Buff.
        // 3) Playing Buff, same move prior turn.
        const ENEMY_TABLE: &'static [StateEntry] = &[
            StateEntry {
                actions: &[EnemyAction::Attack(7)],
                new_states: &[1, 2],
                weights: &[3, 1],
            },
            StateEntry {
                actions: &[EnemyAction::Attack(7)],
                new_states: &[2],
                weights: &[1],
            },
            StateEntry {
                actions: &[EnemyAction::Buff(Buff::Strength(3))],
                new_states: &[0, 3],
                weights: &[3, 1],
            },
            StateEntry {
                actions: &[EnemyAction::Buff(Buff::Strength(3))],
                new_states: &[0],
                weights: &[1],
            },
        ];
        return weighted_transition(rng, state, ENEMY_TABLE);
    }
    let mut buffs = EnemyBuffs::default();
    buffs.curl_up = uniform_inclusive(rng, 3, 7);
    Enemy {
        name: ENEMY_NAME,
        ai_state: 0,
        behavior: ai,
        hp,
        max_hp: hp,
        buffs: buffs,
        debuffs: EnemyDebuffs::default(),
        block: 0,
    }
}
