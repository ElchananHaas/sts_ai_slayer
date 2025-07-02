use crate::{
    card::{Buff, Debuff},
    enemies::{StateEntry, uniform_inclusive, weighted_transition},
    fight::{Enemy, EnemyAction, EnemyBuffs, EnemyDebuffs, Fight},
    rng::Rng,
};

const ENEMY_NAME: &'static str = "Lagavulin";
pub fn generate_lagavulin(rng: &mut Rng) -> Enemy {
    let hp = uniform_inclusive(rng, 109, 111);
    fn ai(rng: &mut Rng, _: &Fight, _: &Enemy, state: u32) -> (u32, &'static [EnemyAction]) {
        const ENEMY_TABLE: &'static [StateEntry] = &[
            StateEntry {
                actions: &[],
                new_states: &[1],
                weights: &[1],
            },
            StateEntry {
                actions: &[],
                new_states: &[2],
                weights: &[1],
            },
            StateEntry {
                actions: &[],
                new_states: &[3],
                weights: &[1],
            },
            StateEntry {
                actions: &[EnemyAction::Attack(18)],
                new_states: &[4],
                weights: &[1],
            },
            StateEntry {
                actions: &[EnemyAction::Attack(18)],
                new_states: &[5],
                weights: &[1],
            },
            StateEntry {
                actions: &[
                    EnemyAction::Debuff(Debuff::MinusDexterity(1)),
                    EnemyAction::Debuff(Debuff::MinusStrength(1)),
                ],
                new_states: &[3],
                weights: &[1],
            },
        ];
        return weighted_transition(rng, state, ENEMY_TABLE);
    }
    let mut buffs = EnemyBuffs::default();
    buffs.metallicize = 8;
    buffs.asleep = true;
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
