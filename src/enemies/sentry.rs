use crate::{
    card::CardBody,
    enemies::{StateEntry, uniform_inclusive, weighted_transition},
    fight::{Enemy, EnemyAction, EnemyBuffs, EnemyDebuffs, Fight},
    rng::Rng,
};

const ENEMY_NAME: &'static str = "Sentry";
pub fn generate_sentry(rng: &mut Rng, start_state: u32) -> Enemy {
    let hp = uniform_inclusive(rng, 38, 42);
    fn ai(rng: &mut Rng, _: &Fight, _: &Enemy, state: u32) -> (u32, &'static [EnemyAction]) {
        const ENEMY_TABLE: &'static [StateEntry] = &[
            StateEntry {
                actions: &[EnemyAction::Attack(9)],
                new_states: &[1],
                weights: &[1],
            },
            StateEntry {
                actions: &[EnemyAction::AddToDiscard(&[
                    CardBody::Dazed,
                    CardBody::Dazed,
                ])],
                new_states: &[0],
                weights: &[1],
            },
        ];
        return weighted_transition(rng, state, ENEMY_TABLE);
    }
    Enemy {
        name: ENEMY_NAME,
        ai_state: start_state,
        behavior: ai,
        hp,
        max_hp: hp,
        buffs: EnemyBuffs::default(),
        debuffs: EnemyDebuffs::default(),
        block: 0,
    }
}
