use crate::{
    card::{Card, CardBody, Debuff},
    enemies::{
        StateEntry, med_black_slime::make_black_slime_table, uniform_inclusive, weighted_transition,
    },
    fight::{Enemy, EnemyAction, EnemyBuffs, EnemyDebuffs, Fight},
    rng::Rng,
};

pub const ENEMY_NAME: &'static str = "Black Slime [L]";
pub fn generate_large_black_slime(rng: &mut Rng) -> Enemy {
    let hp = uniform_inclusive(rng, 64, 70);
    fn ai(rng: &mut Rng, _: &Fight, enemy: &Enemy, state: u32) -> (u32, &'static [EnemyAction]) {
        const SLIMEDS: &'static [CardBody] = &[CardBody::Slimed, CardBody::Slimed];
        const ENEMY_TABLE: &'static [StateEntry] = make_black_slime_table!(16, 2, SLIMEDS);
        if enemy.hp * 2 <= enemy.max_hp {
            return (0, &[EnemyAction::Split]);
        }
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
