use crate::{
    enemies::uniform_inclusive,
    fight::{Enemy, EnemyAction, EnemyBuffs, EnemyDebuffs, EnemyName, Fight},
    rng::Rng,
};
pub fn ai(_: &mut Rng, _: &Fight, _: &Enemy, _: u32) -> (u32, &'static [EnemyAction]) {
    return (0, &[EnemyAction::Attack(9)]);
}

pub fn generate_sneaky_gremlin(rng: &mut Rng) -> Enemy {
    let hp = uniform_inclusive(rng, 10, 14);
    Enemy {
        name: EnemyName::GremlinSneaky,
        ai_state: 0,
        hp,
        max_hp: hp,
        buffs: EnemyBuffs::default(),
        debuffs: EnemyDebuffs::default(),
        block: 0,
    }
}
