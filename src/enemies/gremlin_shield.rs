use crate::{
    enemies::uniform_inclusive,
    fight::{Enemy, EnemyAction, EnemyBuffs, EnemyDebuffs, Fight},
    rng::Rng,
};

const ENEMY_NAME: &'static str = "Shield Gremlin";
pub fn generate_shield_gremlin(rng: &mut Rng) -> Enemy {
    let hp = uniform_inclusive(rng, 12, 15);
    fn ai(_: &mut Rng, fight: &Fight, _: &Enemy, _: u32) -> (u32, &'static [EnemyAction]) {
        if fight.enemies.len() > 1 {
            return (0, &[EnemyAction::DefendAlly(6)]);
        } else {
            return (0, &[EnemyAction::Attack(6)]);
        }
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
