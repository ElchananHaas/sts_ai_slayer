use crate::{
    card::{Card, CardEffect, Debuff},
    enemies::{StateEntry, uniform_inclusive, weighted_transition},
    fight::{Enemy, EnemyAction, EnemyBuffs, EnemyDebuffs, Fight},
    rng::Rng,
};
macro_rules! make_black_slime_table {
    ($attack: expr, $frail_amount: expr, $slimeds: expr) => {
        &[
            StateEntry {
                actions: &[
                    EnemyAction::Attack($attack),
                    EnemyAction::AddToDiscard($slimeds),
                ],
                new_states: &[1, 2],
                weights: &[3, 7],
            },
            StateEntry {
                actions: &[
                    EnemyAction::Attack($attack),
                    EnemyAction::AddToDiscard($slimeds),
                ],
                new_states: &[2],
                weights: &[1],
            },
            StateEntry {
                actions: &[EnemyAction::Debuff(Debuff::Frail($frail_amount))],
                new_states: &[0, 3],
                weights: &[3, 7],
            },
            StateEntry {
                actions: &[EnemyAction::Debuff(Debuff::Frail($frail_amount))],
                new_states: &[0],
                weights: &[1],
            },
        ]
    };
}
pub(crate) use make_black_slime_table;

const ENEMY_NAME: &'static str = "Black Slime [M]";
pub fn generate_med_black_slime(rng: &mut Rng) -> Enemy {
    let hp = uniform_inclusive(rng, 28, 32);
    fn ai(rng: &mut Rng, _: &Fight, _: &Enemy, state: u32) -> (u32, &'static [EnemyAction]) {
        const SLIMEDS: &'static [Card] = &[CardEffect::Slimed.to_card()];
        const ENEMY_TABLE: &'static [StateEntry] = make_black_slime_table!(8, 1, SLIMEDS);
        return weighted_transition(rng, state, ENEMY_TABLE);
    }
    let starting_state = rng.sample_weighted(&[3, 0, 7, 0]);
    Enemy {
        name: ENEMY_NAME,
        ai_state: starting_state as u32,
        behavior: ai,
        hp,
        max_hp: hp,
        buffs: EnemyBuffs::default(),
        debuffs: EnemyDebuffs::default(),
        block: 0,
    }
}
