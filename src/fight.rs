use crate::{
    card::{Buff, Debuff},
    rng::Rng,
};

struct Fight {
    //Fights have at most 5 enemies (Reptomancer + 4 Daggers).
    enemies: [Option<Enemy>; 5],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum EnemyAction {
    Attack(i32),
    Block(i32),
    Buff(Buff),
}
struct Enemy {
    //In order to allow full information to be passed to an AI model,
    //the enemy AI state is encoded as a state machine. This works for most
    //enemies. Some enemies like Avacado and some bosses will change intent
    //based on certain health breakpoints being hit.

    //The odd cases are - The Guardian. It has a intent change based on an HP threshold being met
    //which is raised on Mode shift.
    name: &'static str,
    ai_state: u32,
    //A function from the current state to the new ai state and the actions to take.
    behavior: fn(&mut Rng, &Fight, &Enemy, u32) -> (u32, &'static [EnemyAction]),
    hp: i32,
    max_hp: i32,
    //Being a minion is a buff.
    buffs: Vec<Buff>,
    debuffs: Vec<Debuff>,
}

const JAW_WORM_NAME: &'static str = "Jaw Worm";
fn generate_jaw_worm(rng: &mut Rng) -> Enemy {
    let hp = 40 + rng.sample_i32(5);
    fn jaw_worm_ai(rng: &mut Rng, _: &Fight, _: &Enemy, state: u32) -> (u32, &'static [EnemyAction]) {
        // States are 
        // 1) Playing Attack
        // 2) Playing Defend+Attack, different move first. 
        // 3) Playing Defend+Attack, same move prior turn.
        // 4) Playing Buff. 
        // Jaw Worm's actions are a bit weird. The code samples a boolean if the same
        // action is chosen too many times in a row. The devs then changed the AI but didn't
        // update the boolean values so the percentages are now strange.
        const JAW_WORM_TABLE: &'static [StateEntry]= &[
            StateEntry {
                actions: &[EnemyAction::Attack(11)],
                new_states: &[2, 4],
                //Yes, these are the actual odds.
                weights: &[131, 189],
            },
            StateEntry {
                actions: &[EnemyAction::Attack(7), EnemyAction::Block(5)],
                new_states: &[1, 3, 4],
                weights: &[25, 30, 45],
            },
            StateEntry {
                actions: &[EnemyAction::Attack(7), EnemyAction::Block(5)],
                new_states: &[1, 4],
                weights: &[3571, 6429],
            },
            StateEntry {
                actions: &[EnemyAction::Buff(Buff::Strength(3)), EnemyAction::Block(6)],
                new_states: &[1, 2],
                weights: &[1093, 1407],
            }
        ];
        return weighted_transition(rng, state, JAW_WORM_TABLE)
    }
    Enemy {
        name: JAW_WORM_NAME,
        ai_state: 0,
        behavior: jaw_worm_ai,
        hp: hp,
        max_hp: hp,
        buffs: Vec::new(),
        debuffs: Vec::new(),
    }
}

struct StateEntry {
    actions: &'static [EnemyAction],
    //The first entry is the new state. The second entry is the weight.
    new_states: &'static [u32],
    weights: &'static [u32]
}

fn weighted_transition(rng: &mut Rng, state: u32, entries: &'static [StateEntry]) -> (u32, &'static [EnemyAction]) {
    let entry = &entries[state as usize];
    let new_idx = rng.sample_weighted(entry.weights);
    (entry.new_states[new_idx], entry.actions)
}