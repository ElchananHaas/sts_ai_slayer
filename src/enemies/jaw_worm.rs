use crate::{
    card::Buff,
    enemies::{uniform_inclusive, weighted_transition, StateEntry},
    fight::{Enemy, EnemyAction, EnemyBuffs, EnemyDebuffs, Fight},
    rng::Rng,
};

const ENEMY_NAME: &'static str = "Jaw Worm";
pub fn generate_jaw_worm(rng: &mut Rng) -> Enemy {
    let hp = uniform_inclusive(rng, 40, 44);
    fn ai(rng: &mut Rng, _: &Fight, _: &Enemy, state: u32) -> (u32, &'static [EnemyAction]) {
        // States are
        // 0) Playing Attack
        // 1) Playing Defend+Attack, different move first.
        // 2) Playing Defend+Attack, same move prior turn.
        // 3) Playing Buff.
        // Jaw Worm's actions are a bit weird. The code samples a boolean if the same
        // action is chosen too many times in a row. The devs then changed the AI but didn't
        // update the boolean values so the percentages are now strange, but the values are accurate.
        const ENEMY_TABLE: &'static [StateEntry] = &[
            StateEntry {
                actions: &[EnemyAction::Attack(11)],
                new_states: &[1, 3],
                weights: &[131, 189],
            },
            StateEntry {
                actions: &[EnemyAction::Attack(7), EnemyAction::Block(5)],
                new_states: &[0, 2, 3],
                weights: &[25, 30, 45],
            },
            StateEntry {
                actions: &[EnemyAction::Attack(7), EnemyAction::Block(5)],
                new_states: &[0, 3],
                weights: &[3571, 6429],
            },
            StateEntry {
                actions: &[EnemyAction::Buff(Buff::Strength(3)), EnemyAction::Block(6)],
                new_states: &[0, 1],
                weights: &[1093, 1407],
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
