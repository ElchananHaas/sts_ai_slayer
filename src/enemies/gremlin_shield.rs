use crate::{
    enemies::uniform_inclusive,
    fight::{Enemy, EnemyAction, EnemyBuffs, EnemyDebuffs, EnemyName, Fight},
    rng::Rng,
};

pub fn ai(_: &mut Rng, fight: &Fight, _: &Enemy, _: u32) -> (u32, &'static [EnemyAction]) {
    if fight.enemies.len() > 1 {
        return (0, &[EnemyAction::DefendAlly(6)]);
    } else {
        return (0, &[EnemyAction::Attack(6)]);
    }
}
pub fn generate_shield_gremlin(rng: &mut Rng) -> Enemy {
    let hp = uniform_inclusive(rng, 12, 15);
    Enemy {
        name: EnemyName::GremlinShield,
        ai_state: 0,
        hp,
        max_hp: hp,
        buffs: EnemyBuffs::default(),
        debuffs: EnemyDebuffs::default(),
        block: 0,
    }
}
