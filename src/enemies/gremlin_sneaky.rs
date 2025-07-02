use crate::{
    enemies::uniform_inclusive,
    fight::{Enemy, EnemyAction, EnemyBuffs, EnemyDebuffs, Fight},
    rng::Rng,
};

const ENEMY_NAME: &'static str = "Sneaky Gremlin";
pub fn generate_sneaky_gremlin(rng: &mut Rng) -> Enemy {
    let hp = uniform_inclusive(rng, 10, 14);
    fn ai(_: &mut Rng, _: &Fight, _: &Enemy, _: u32) -> (u32, &'static [EnemyAction]) {
        return (0, &[EnemyAction::Attack(9)]);
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
