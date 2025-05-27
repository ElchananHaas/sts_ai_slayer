use crate::{
    card::Debuff,
    enemies::{StateEntry, uniform_inclusive, weighted_transition},
    fight::{Enemy, EnemyAction, EnemyBuffs, EnemyDebuffs, Fight},
    rng::Rng,
};

const ENEMY_NAME: &'static str = "Red Slaver";
pub fn generate_red_slaver(rng: &mut Rng) -> Enemy {
    let hp = uniform_inclusive(rng, 46, 50);
    fn ai(rng: &mut Rng, _: &Fight, _: &Enemy, state: u32) -> (u32, &'static [EnemyAction]) {
        const ENEMY_TABLE: &'static [StateEntry] = &[
            StateEntry {
                actions: &[EnemyAction::Attack(13)],
                new_states: &[1, 3],
                weights: &[3, 1],
            },
            StateEntry {
                actions: &[EnemyAction::Attack(13)],
                new_states: &[2, 3],
                weights: &[3, 1],
            },
            StateEntry {
                actions: &[
                    EnemyAction::Attack(8),
                    EnemyAction::Debuff(Debuff::Vulnerable(1)),
                ],
                new_states: &[0, 3],
                weights: &[3, 1],
            },
            StateEntry {
                actions: &[EnemyAction::Debuff(Debuff::Entangled)],
                new_states: &[4, 6],
                weights: &[55, 45],
            },
            StateEntry {
                actions: &[
                    EnemyAction::Attack(8),
                    EnemyAction::Debuff(Debuff::Vulnerable(1)),
                ],
                new_states: &[5, 6],
                weights: &[55, 45],
            },
            StateEntry {
                actions: &[
                    EnemyAction::Attack(8),
                    EnemyAction::Debuff(Debuff::Vulnerable(1)),
                ],
                new_states: &[6],
                weights: &[1],
            },
            StateEntry {
                actions: &[EnemyAction::Attack(13)],
                new_states: &[4, 6],
                weights: &[55, 45],
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
        ..Enemy::default()
    }
}
