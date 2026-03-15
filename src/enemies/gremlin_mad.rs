use crate::{
    enemies::uniform_inclusive,
    fight::{Enemy, EnemyAction, EnemyBuffs, EnemyDebuffs, EnemyName, Fight},
    rng::Rng,
};

pub fn ai(_: &mut Rng, _: &Fight, _: &Enemy, _: u32) -> (u32, &'static [EnemyAction]) {
    return (0, &[EnemyAction::Attack(4)]);
}
pub fn generate_mad_gremlin(rng: &mut Rng) -> Enemy {
    let hp = uniform_inclusive(rng, 20, 24);
    let mut buffs = EnemyBuffs::default();
    buffs.angry = 1;
    Enemy {
        name: EnemyName::GremlinMad,
        ai_state: 0,
        hp,
        max_hp: hp,
        buffs,
        debuffs: EnemyDebuffs::default(),
        block: 0,
    }
}
