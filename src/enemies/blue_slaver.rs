use crate::{
    card::Debuff,
    enemies::{StateEntry, uniform_inclusive, weighted_transition},
    fight::{Enemy, EnemyAction, EnemyBuffs, EnemyDebuffs, Fight},
    rng::Rng,
};

macro_rules! make_blue_slaver_table {
    ($attack_big: expr, $attack: expr, $weak_amount: expr) => {
        &[
            StateEntry {
                actions: &[
                    EnemyAction::Attack($attack),
                    EnemyAction::Debuff(Debuff::Weak($weak_amount)),
                ],
                new_states: &[1, 2],
                weights: &[4, 6],
            },
            StateEntry {
                actions: &[
                    EnemyAction::Attack($attack),
                    EnemyAction::Debuff(Debuff::Weak($weak_amount)),
                ],
                new_states: &[2],
                weights: &[1],
            },
            StateEntry {
                actions: &[EnemyAction::Attack($attack_big)],
                new_states: &[0, 3],
                weights: &[4, 6],
            },
            StateEntry {
                actions: &[EnemyAction::Attack($attack_big)],
                new_states: &[0],
                weights: &[1],
            },
        ]
    };
}

const ENEMY_NAME: &'static str = "Blue Slaver";
pub fn generate_blue_slaver(rng: &mut Rng) -> Enemy {
    let hp = uniform_inclusive(rng, 46, 50);
    fn ai(rng: &mut Rng, _: &Fight, _: &Enemy, state: u32) -> (u32, &'static [EnemyAction]) {
        // States are
        // 0) Playing Attack
        // 1) Debuff
        const ENEMY_TABLE: &'static [StateEntry] = make_blue_slaver_table!(12, 7, 1);
        return weighted_transition(rng, state, ENEMY_TABLE);
    }
    Enemy {
        name: ENEMY_NAME,
        ai_state: rng.sample_weighted(&[4, 0, 6, 0]) as u32,
        behavior: ai,
        hp,
        max_hp: hp,
        buffs: EnemyBuffs::default(),
        debuffs: EnemyDebuffs::default(),
        block: 0,
    }
}
