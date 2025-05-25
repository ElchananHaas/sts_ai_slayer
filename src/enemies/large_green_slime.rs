use crate::{
    card::{Card, CardEffect, Debuff},
    enemies::{med_green_slime::make_green_slime_table, uniform_inclusive, weighted_transition, StateEntry},
    fight::{Enemy, EnemyAction, EnemyBuffs, EnemyDebuffs, Fight},
    rng::Rng,
};

pub const ENEMY_NAME: &'static str = "Green Slime [L]";
pub fn generate_med_green_slime(rng: &mut Rng) -> Enemy {
    let hp = uniform_inclusive(rng, 28, 32);
    fn ai(rng: &mut Rng, _: &Fight, enemy: &Enemy, state: u32) -> (u32, &'static [EnemyAction]) {
        // States are
        // 0) Attack + Slimed inserted
        // 1) Attack + Slimed inserted (second)
        // 2) Attack
        // 3) Debuff
        // 4) Debuff (second)
        const SLIMEDS: &'static [Card] = &[CardEffect::Slimed.to_card(), CardEffect::Slimed.to_card()];
        const ENEMY_TABLE: &'static [StateEntry] = make_green_slime_table!(11, 16, 2, SLIMEDS);
        if enemy.hp * 2 <= enemy.max_hp {
            return (0, &[EnemyAction::Split]);
        }
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
        block: 0,
    }
}
