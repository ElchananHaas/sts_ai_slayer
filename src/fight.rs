use std::{
    cmp::{max, min},
    collections::VecDeque,
    mem,
    ops::{Index, IndexMut},
};

use derive_getters::Getters;

use crate::{
    card::{Buff, Card, CardBody, CardType, Cost, Debuff}, deck::Deck, game::Game, relic::Relic, rng::Rng, util::insert_sorted
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default, Getters)]
pub struct Fight {
    pub enemies: Enemies,
    pub hand: Vec<Card>,
    pub discard_pile: Vec<Card>,
    pub exhaust: Vec<Card>,
    pub deck: Deck,
    pub energy: i32,
    pub player_block: i32,
    pub player_debuffs: PlayerDebuffs,
    pub player_buffs: PlayerBuffs,
    pub stolen_back_gold: i32,
    //This is used for cards which play other cards, such as Havoc and some powers.
    pub post_card_queue: VecDeque<PostCardItem>,
    pub rewards: FightRewards,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct FightRewards {
    pub gold_min: i32,
    pub gold_max: i32,
    pub relic_count: i32,
    pub fixed_relic: Option<Relic>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PlayerDebuffs {
    pub weak: i32,
    pub vulnerable: i32,
    pub frail: i32,
    pub entangled: bool,
    //These are used for temporary dex/str for the turn effects.
    pub strength_down: i32,
    pub dexterity_down: i32,
    pub no_draw: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PlayerBuffs {
    pub strength: i32,
    pub num_times_lost_hp: i32,
    pub end_turn_lose_hp: i32,
    pub end_turn_damage_all_enemies: i32,
    pub dark_embrace: i32,
    pub evolve: i32,
    pub fnp: i32,
    pub fire_breathing: i32,
    pub temp_spikes: i32,
    pub metallicize: i32,
    pub rage: i32,
    pub rupture: i32,
    pub barricade: bool,
    pub energy_every_turn: i32,
    pub brutality: i32,
    pub corruption: bool,
    pub ritual: i32,
    pub double_tap: i32,
    pub juggernaut: i32,
    pub dexterity: i32,
}

//This holds effects that happen after a card finishes resolving.
//This includes some powers, relics, and cards that play other cards.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PostCardItem {
    PlayCard(PlayCardContext),
    Draw(i32),
    GainBlock(i32),
    DamageAll(i32),
    GainEnergy(i32),
    DamageRandomEnemy(i32),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PlayCardContext {
    pub card: Card,
    pub target: usize,
    pub exhausts: bool,
    pub real_card: bool,
    pub effect_index: usize,
    pub x: i32,
}

impl Fight {
    pub fn new() -> Self {
        Self::default()
    }
    //This removes the top of the deck, reshuffling if needed.
    //It can fail if the deck and discard are both empty.
    pub fn remove_top_of_deck(&mut self, rng: &mut Rng) -> Option<Card> {
        if self.deck.len() == 0 {
            let mut old_discard = vec![];
            mem::swap(&mut old_discard, &mut self.discard_pile);
            self.deck = Deck::shuffled(old_discard);
        }
        if self.deck.len() > 0 {
            //Since there are at most 10 cards in hand, just insert into a vec
            //rather than use a fancier data structure.
            //Be careful to always maintain the hand as sorted when doing other operations
            //on the hand, such as creating cards.
            //Maintaining the sorted order helps with MCTS and idenitfying identical states.
            return Some(self.deck.draw(rng));
        }
        None
    }
    pub fn draw(&mut self, rng: &mut Rng) {
        if self.hand.len() >= 10 || self.player_debuffs.no_draw {
            return;
        }
        self.remove_top_of_deck(rng).map(|card| {
            if card.body.card_type() == CardType::Status && self.player_buffs.evolve > 0 {
                self.post_card_queue
                    .push_back(PostCardItem::Draw(self.player_buffs.evolve));
            }
            if (card.body.card_type() == CardType::Status
                || card.body.card_type() == CardType::Curse)
                && self.player_buffs.fire_breathing > 0
            {
                self.post_card_queue
                    .push_back(PostCardItem::DamageAll(self.player_buffs.fire_breathing));
            }
            insert_sorted(card, &mut self.hand);
        });
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct Enemies {
    //Fights have at most 5 enemies (Reptomancer + 4 Daggers).
    pub enemies: [Option<Enemy>; Game::MAX_ENEMIES],
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
        self.enemies[index.0 as usize]
            .as_ref()
            .expect("Enemy exists!")
    }
}

impl IndexMut<EnemyIdx> for Enemies {
    fn index_mut(&mut self, index: EnemyIdx) -> &mut Self::Output {
        self.enemies[index.0 as usize]
            .as_mut()
            .expect("Enemy exists!")
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
            let card = self.hand[idx].clone();
            if self.player_debuffs.entangled && card.body.card_type() == CardType::Attack {
                return false;
            }
            if card.body == CardBody::Clash {
                for card in &self.hand {
                    if card.body.card_type() != CardType::Attack {
                        return false;
                    }
                }
            }
            let Some(energy_cost) = self.evaluate_cost(&card) else {
                return false;
            };
            energy_cost <= self.energy
        }
    }

    pub fn evaluate_cost(&self, card: &Card) -> Option<i32> {
        let base = match card.cost {
            Cost::Unplayable => None,
            Cost::Fixed(x) => Some(x),
            Cost::X => Some(self.energy),
            Cost::NumMinusHpLoss(x) => Some(max(0, x - self.player_buffs.num_times_lost_hp)),
        };
        if self.player_buffs.corruption
            && base.is_some()
            && card.body.card_type() == CardType::Skill
        {
            return Some(0);
        };
        if let Some(temp) = card.temp_cost {
            if let Some(base) = base
                && !matches!(card.cost, Cost::X)
            {
                Some(min(base, temp))
            } else {
                None
            }
        } else {
            base
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EnemyAction {
    Attack(i32),
    Block(i32),
    Buff(Buff),
    Debuff(Debuff),
    AddToDiscard(&'static [CardBody]),
    Split,
    DefendAlly(i32),
    Escape,
    StealGold(i32),
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
    pub curl_up: i32,
    pub queued_block: i32, //Queued block for after the current card finishes resolving.
    //This is used for Malleable and Curl Up
    pub implicit_strength: i32, //This is used for the louses which
    //start with a strength between 5 and 7
    pub angry: i32,
    pub spore_cloud: i32,
    pub stolen_gold: i32,
    pub enrage: i32,
    pub metallicize: i32,
    pub asleep: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct EnemyDebuffs {
    pub vulnerable: i32,
    pub weak: i32,
}
