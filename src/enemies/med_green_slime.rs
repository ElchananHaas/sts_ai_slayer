use crate::{
    card::{Card, CardEffect, Debuff},
    enemies::{StateEntry, uniform_inclusive, weighted_transition},
    fight::{Enemy, EnemyAction, EnemyBuffs, EnemyDebuffs, Fight},
    rng::Rng,
};

macro_rules! make_green_slime_table {
    ($attack: expr, $attack_big: expr, $weak_amount: expr, $slimeds: expr) => {
        &[
            StateEntry {
                actions: &[
                    EnemyAction::Attack($attack),
                    EnemyAction::AddToDiscard(SLIMEDS),
                ],
                new_states: &[1, 2, 3],
                weights: &[3, 4, 3],
            },
            StateEntry {
                actions: &[
                    EnemyAction::Attack($attack),
                    EnemyAction::AddToDiscard(SLIMEDS),
                ],
                new_states: &[2, 3],
                weights: &[55, 45],
            },
            StateEntry {
                actions: &[EnemyAction::Attack($attack_big)],
                new_states: &[0, 3],
                weights: &[46, 54],
            },
            StateEntry {
                actions: &[EnemyAction::Debuff(Debuff::Weak($weak_amount))],
                new_states: &[0, 2, 4],
                weights: &[3, 4, 3],
            },
            StateEntry {
                actions: &[EnemyAction::Debuff(Debuff::Weak($weak_amount))],
                new_states: &[0, 2],
                weights: &[42, 58],
            },
        ]
    };
}
pub(crate) use make_green_slime_table;

const ENEMY_NAME: &'static str = "Green Slime [M]";
pub fn generate_med_green_slime(rng: &mut Rng) -> Enemy {
    let hp = uniform_inclusive(rng, 28, 32);
    fn ai(rng: &mut Rng, _: &Fight, _: &Enemy, state: u32) -> (u32, &'static [EnemyAction]) {
        // States are
        // 0) Attack + Slimed inserted
        // 1) Attack + Slimed inserted (second)
        // 2) Attack
        // 3) Debuff
        // 4) Debuff (second)
        const SLIMEDS: &'static [Card] = &[CardEffect::Slimed.to_card()];
        const ENEMY_TABLE: &'static [StateEntry] = make_green_slime_table!(7, 10, 1, SLIMEDS);
        return weighted_transition(rng, state, ENEMY_TABLE);
    }
    let starting_state = rng.sample_weighted(&[3, 0, 4, 3, 0]);
    Enemy {
        name: ENEMY_NAME,
        ai_state: starting_state as u32,
        behavior: ai,
        hp,
        max_hp: hp,
        buffs: EnemyBuffs::default(),
        debuffs: EnemyDebuffs::default(),
        ..Enemy::default()
    }
}
