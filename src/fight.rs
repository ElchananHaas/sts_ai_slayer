use std::{
    mem,
    ops::{Index, IndexMut},
};

use crate::{
    card::{Buff, Card},
    deck::Deck,
    rng::Rng,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Fight {
    pub enemies: Enemies,
    pub hand: Vec<Card>,
    pub discard_pile: Vec<Card>,
    pub deck: Deck,
    pub energy: i32,
    pub player_block: i32,
}

impl Fight {
    //The setup method must be called to allow for allocation reuse.
    pub fn new() -> Self {
        Self {
            enemies: Enemies {
                enemies: [const { None }; 5],
            },
            hand: vec![],
            deck: Deck::shuffled(vec![]),
            energy: 0,
            player_block: 0,
            discard_pile: vec![],
        }
    }
    pub fn draw(&mut self, rng: &mut Rng) {
        if self.hand.len() >= 10 {
            return;
        }
        if self.deck.len() == 0 {
            let mut old_discard = vec![];
            mem::swap(&mut old_discard, &mut self.discard_pile);
            self.deck = Deck::shuffled(old_discard);
        }
        if self.deck.len() > 0 {
            self.hand.push(self.deck.draw(rng));
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Enemies {
    //Fights have at most 5 enemies (Reptomancer + 4 Daggers).
    pub enemies: [Option<Enemy>; 5],
}

impl Enemies {
    pub fn indicies(&self) -> EnemiesIdxIter {
        let mut res: u8 = 0;
        for i in 0..self.enemies.len() {
            res <<= 1;
            if self.enemies[i].is_some() {
                res |= 1
            }
        }
        EnemiesIdxIter {
            filled: res,
            pos: 0,
        }
    }

    pub fn get_option(&mut self, idx: usize) -> &mut Option<Enemy> {
        &mut self.enemies[idx]
    }
}
impl Index<usize> for Enemies {
    type Output = Enemy;
    fn index(&self, index: usize) -> &Self::Output {
        &self.enemies[index].as_ref().expect("Enemy exists!")
    }
}

impl IndexMut<usize> for Enemies {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.enemies[index].as_mut().expect("Enemy exists!")
    }
}

pub struct EnemiesIdxIter {
    filled: u8,
    pos: u8,
}
impl Iterator for EnemiesIdxIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.filled == 0 {
            None
        } else {
            while (self.filled & 1) == 0 {
                self.filled >>= 1;
                self.pos += 1;
            }
            self.filled >>= 1;
            let res = self.pos;
            self.pos += 1;
            Some(res as usize)
        }
    }
}

impl Fight {
    //Returns if the i'th card in hand is playable.
    pub fn is_playable(&self, idx: usize) -> bool {
        if idx >= self.hand.len() {
            false
        } else {
            //TODO handle Blue Candle and Medical kit.
            //TODO handle can't play attack effects (Entangled, Awakened One dead)
            let Some(energy_cost) = &self.hand[idx].cost else {
                return false;
            };
            *energy_cost <= self.energy
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EnemyAction {
    Attack(i32),
    Block(i32),
    Buff(Buff),
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Enemy {
    //In order to allow full information to be passed to an AI model,
    //the enemy AI state is encoded as a state machine. This works for most
    //enemies. Some enemies like Avacado and some bosses will change intent
    //based on certain health breakpoints being hit.

    //The odd cases are - The Guardian. It has a intent change based on an HP threshold being met
    //which is raised on Mode shift.
    pub name: &'static str,
    pub ai_state: u32,
    //A function from the current state to the new ai state and the actions to take.
    pub behavior: fn(&mut Rng, &Fight, &Enemy, u32) -> (u32, &'static [EnemyAction]),
    pub hp: i32,
    pub max_hp: i32,
    //Being a minion is a buff.
    pub buffs: EnemyBuffs,
    pub debuffs: EnemyDebuffs,
    pub block: i32,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct EnemyBuffs {
    pub strength: i32,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct EnemyDebuffs {
    pub vulnerable: i32,
}

const JAW_WORM_NAME: &'static str = "Jaw Worm";
pub fn generate_jaw_worm(rng: &mut Rng) -> Enemy {
    let hp = 40 + rng.sample_i32(5);
    fn jaw_worm_ai(
        rng: &mut Rng,
        _: &Fight,
        _: &Enemy,
        state: u32,
    ) -> (u32, &'static [EnemyAction]) {
        // States are
        // 1) Playing Attack
        // 2) Playing Defend+Attack, different move first.
        // 3) Playing Defend+Attack, same move prior turn.
        // 4) Playing Buff.
        // Jaw Worm's actions are a bit weird. The code samples a boolean if the same
        // action is chosen too many times in a row. The devs then changed the AI but didn't
        // update the boolean values so the percentages are now strange, but the values are accurate.
        const JAW_WORM_TABLE: &'static [StateEntry] = &[
            StateEntry {
                actions: &[EnemyAction::Attack(11)],
                new_states: &[2, 4],
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
            },
        ];
        return weighted_transition(rng, state, JAW_WORM_TABLE);
    }
    Enemy {
        name: JAW_WORM_NAME,
        ai_state: 0,
        behavior: jaw_worm_ai,
        hp,
        max_hp: hp,
        buffs: EnemyBuffs::default(),
        debuffs: EnemyDebuffs::default(),
        block: 0,
    }
}

struct StateEntry {
    actions: &'static [EnemyAction],
    //The first entry is the new state. The second entry is the weight.
    new_states: &'static [u32],
    weights: &'static [u32],
}

fn weighted_transition(
    rng: &mut Rng,
    state: u32,
    entries: &'static [StateEntry],
) -> (u32, &'static [EnemyAction]) {
    let entry = &entries[state as usize];
    let new_idx = rng.sample_weighted(entry.weights);
    (entry.new_states[new_idx], entry.actions)
}
