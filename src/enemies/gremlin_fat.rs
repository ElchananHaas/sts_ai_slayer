use crate::{
    card::Debuff,
    enemies::uniform_inclusive,
    fight::{Enemy, EnemyAction, EnemyBuffs, EnemyDebuffs, Fight},
    rng::Rng,
};

const ENEMY_NAME: &'static str = "Fat Gremlin";
pub fn generate_fat_gremlin(rng: &mut Rng) -> Enemy {
    let hp = uniform_inclusive(rng, 13, 17);
    fn ai(_: &mut Rng, _: &Fight, _: &Enemy, _: u32) -> (u32, &'static [EnemyAction]) {
        return (
            0,
            &[EnemyAction::Attack(4), EnemyAction::Debuff(Debuff::Weak(1))],
        );
    }
    let mut buffs = EnemyBuffs::default();
    buffs.angry = 1;
    Enemy {
        name: ENEMY_NAME,
        ai_state: 0,
        behavior: ai,
        hp,
        max_hp: hp,
        buffs,
        debuffs: EnemyDebuffs::default(),
        ..Enemy::default()
    }
}
