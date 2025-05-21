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
        for i in (0..self.enemies.len()).rev() {
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
    pub fn len(&self) -> usize {
        self.enemies.iter().filter(|x| x.is_some()).count()
    }
}
impl Index<usize> for Enemies {
    type Output = Option<Enemy>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.enemies[index]
    }
}

impl IndexMut<usize> for Enemies {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.enemies[index]
    }
}

#[derive(Clone, Copy, Debug)]
pub struct EnemyIdx(pub u8);
impl Index<EnemyIdx> for Enemies {
    type Output = Enemy;
    fn index(&self, index: EnemyIdx) -> &Self::Output {
        self.enemies[index.0 as usize].as_ref().expect("Enemy exists!")
    }
}

impl IndexMut<EnemyIdx> for Enemies {
    fn index_mut(&mut self, index: EnemyIdx) -> &mut Self::Output {
        self.enemies[index.0 as usize].as_mut().expect("Enemy exists!")
    }
}

pub struct EnemiesIdxIter {
    filled: u8,
    pos: u8,
}
impl Iterator for EnemiesIdxIter {
    type Item = EnemyIdx;

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
            Some(EnemyIdx(res))
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
    pub ritual: i32,
    pub ritual_skip_first: i32,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct EnemyDebuffs {
    pub vulnerable: i32,
}

