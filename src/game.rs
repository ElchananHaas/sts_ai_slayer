mod buff_debuff;
pub mod choice;
mod choice_handler;
pub mod encounter;
mod event;
mod goto_state;
mod perform_action;

use std::{cmp::min, mem, vec};

use derive_getters::Getters;

use crate::act::Act;
use crate::game::choice::{
    Choice, ChoiceState, ChooseEnemyAction, PlayCardAction, SelectCardAction, SelectionPile,
};
use crate::map::ActMap;
use crate::{
    card::{Buff, Card, CardBody, CardType, Debuff, SelectCardEffect},
    deck::Deck,
    enemies::{
        med_black_slime::generate_med_black_slime, med_green_slime::generate_med_green_slime,
    },
    fight::{Enemy, EnemyAction, EnemyIdx, Fight, PlayCardContext, PostCardItem},
    relic::{RelicPool, Relics},
    rng::Rng,
    util::insert_sorted,
};

pub const QUESTION_MONSTER_BASE_WEIGHT: i32 = 10;
pub const QUESTION_SHOP_BASE_WEIGHT: i32 = 3;
pub const QUESTION_TREASURE_BASE_WEIGHT: i32 = 2;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Getters)]
pub struct Game {
    player_hp: i32,
    player_max_hp: i32,
    max_potion_slots: i32,
    charachter: Charachter,
    fight: Fight,
    base_deck: Vec<Card>,
    relic_pool: RelicPool,
    relics: Relics,
    gold: i32,
    rng: Rng,
    floor: i32,
    map: ActMap,
    act: Act,
}

//Some cards, like Armaments, may require interrupting the execution of a
//cards effects to allow the player to select an option. In this case,
//the enum will signal that the loop must be broken. These cards
//will migrate to the SelectCardState which will resume exection once a selection
//is made.
#[must_use]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ActionControlFlow {
    Continue,
    SelectCards(Vec<SelectCardAction>, SelectCardEffect, SelectionPile),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct AttackResult {
    lethal: bool,
    damage_dealt: i32,
}

impl Game {
    pub fn get_floor(&self) -> i32 {
        self.floor
    }
    //This function starts a fight in the given game. Useful for testing.
    pub fn start_fight(&mut self) -> Choice {
        self.play_card_choice()
    }

    fn play_card_choice(&mut self) -> Choice {
        let fight = &self.fight;
        let mut actions = vec![PlayCardAction::EndTurn];
        for i in 0..fight.hand.len() {
            if fight.is_playable(i) {
                actions.push(PlayCardAction::PlayCard(i as u8));
            }
        }
        Choice::PlayCardState(actions)
    }

    fn choose_enemy_choice(&mut self, chosen_card_idx: usize) -> Choice {
        let fight = &self.fight;
        let mut actions = vec![];
        for i in fight.enemies.indicies() {
            actions.push(ChooseEnemyAction { enemy: i.0 });
        }
        Choice::ChooseEnemyState(actions, chosen_card_idx)
    }

    fn damage_player(&mut self, damage: i32, from_card: bool) -> Option<Choice> {
        if damage > self.fight.player_block {
            let dealt = damage - self.fight.player_block;
            self.fight.player_block = 0;
            self.player_lose_hp(dealt, from_card);
        } else {
            self.fight.player_block -= damage;
        }
        if self.player_hp <= 0 {
            self.player_hp = 0;
            return Some(Choice::Loss);
        }
        return None;
    }
    fn enemy_phase(&mut self) -> Choice {
        self.discard_hand_end_of_turn();
        for i in self.fight.enemies.indicies() {
            let enemy_actions;
            {
                let enemy = &self.fight.enemies[i];
                enemy_actions = (enemy.behavior)(&mut self.rng, &self.fight, enemy, enemy.ai_state);
                self.fight.enemies[i].ai_state = enemy_actions.0;
            }

            for action in enemy_actions.1 {
                match action {
                    EnemyAction::Attack(damage) => {
                        let enemy = &self.fight.enemies[i];
                        let damage = *damage + enemy.buffs.strength + enemy.buffs.implicit_strength;
                        let mut damage = damage as f32;
                        //Weak and vulnerable calculations require using floats then rounding down afterwards.
                        if enemy.debuffs.weak > 0 {
                            damage *= 0.75;
                        }
                        if self.fight.player_debuffs.vulnerable > 0 {
                            damage *= 1.5;
                        }
                        let damage = damage as i32;
                        if let Some(choice) = self.damage_player(damage, false) {
                            return choice;
                        }
                        let player_spikiness = self.fight.player_buffs.temp_spikes;
                        if player_spikiness > 0 {
                            self.damage_enemy(player_spikiness, i.0 as usize, false);
                        }
                        let enemy = &self.fight.enemies[i];
                        //An enemy dying from spikes can interrupt a multi-attack.
                        if enemy.hp <= 0 {
                            break;
                        };
                    }
                    EnemyAction::Block(block) => {
                        self.fight.enemies[i].block += block;
                    }
                    EnemyAction::Buff(buff) => {
                        Self::buff_enemy(&mut self.fight.enemies[i], *buff);
                    }
                    EnemyAction::Debuff(debuff) => {
                        self.apply_debuff_to_player(*debuff);
                    }
                    EnemyAction::AddToDiscard(cards) => {
                        self.fight
                            .discard_pile
                            .extend(cards.into_iter().map(|c| c.to_card()));
                        //Sort for greater MCTS efficiency. Technically, this is different from STS
                        //with regards to All For One, but I will accept this for now.
                        self.fight.discard_pile.sort();
                    }
                    EnemyAction::Split => {
                        self.split(i);
                    }
                    EnemyAction::DefendAlly(amount) => {
                        self.defend_ally(i, *amount);
                    }
                    EnemyAction::Escape => {
                        self.fight.enemies.enemies[i.0 as usize] = None;
                    }
                    EnemyAction::StealGold(amount) => {
                        let steal_amount = std::cmp::min(self.gold, *amount);
                        self.gold -= steal_amount;
                        self.fight.enemies[i].buffs.stolen_gold += steal_amount;
                    }
                }
            }
        }
        if self.fight.enemies.len() == 0 {
            return self.win_battle();
        }
        self.reset_for_next_turn();
        self.play_card_choice()
    }

    fn reset_for_next_turn(&mut self) {
        //TODO implement relics that affect energy.
        //TODO implement cards that affect energy.
        for enemy_idx in self.fight.enemies.indicies() {
            let enemy: &mut Enemy = &mut self.fight.enemies[enemy_idx];
            enemy.buffs.strength += enemy.buffs.ritual;
            //Cultists skip the ritual buff the turn they play it.
            enemy.buffs.ritual += enemy.buffs.ritual_skip_first;
            enemy.buffs.ritual_skip_first = 0;
            decrement(&mut enemy.debuffs.vulnerable);
            decrement(&mut enemy.debuffs.weak);
            if enemy.buffs.metallicize > 0 {
                enemy.block += enemy.buffs.metallicize;
            }
        }
        decrement(&mut self.fight.player_debuffs.vulnerable);
        decrement(&mut self.fight.player_debuffs.weak);
        decrement(&mut self.fight.player_debuffs.frail);
        self.fight.player_buffs.temp_spikes = 0;
        self.fight.player_buffs.rage = 0;
        let draw_amount = 5 + self.fight.player_buffs.brutality;
        for _ in 0..draw_amount {
            self.fight.draw(&mut self.rng);
        }
        if self.fight.player_buffs.brutality > 0 {
            self.player_lose_hp(self.fight.player_buffs.brutality, false);
        }
        if !self.fight.player_buffs.barricade {
            self.fight.player_block = 0;
        }
        self.fight.energy = 3 + self.fight.player_buffs.energy_every_turn;
    }

    fn discard_hand_end_of_turn(&mut self) {
        let hand_size = self.fight.hand.len();
        let mut old_hand = Vec::new();
        mem::swap(&mut old_hand, &mut self.fight.hand);
        for mut card in old_hand {
            card.temp_cost = None;
            if card.body == CardBody::Burn {
                self.damage_player(2, true);
            }
            if card.body == CardBody::Regret {
                self.player_lose_hp(hand_size as i32, true);
            }
            if card.body == CardBody::Doubt {
                self.apply_debuff_to_player(Debuff::Weak(1));
            }
            if card.ethereal() {
                self.exhaust(card);
            } else {
                insert_sorted(card, &mut self.fight.discard_pile);
            }
        }
        //TODO handle artifact.
        self.fight.player_buffs.strength -= self.fight.player_debuffs.strength_down;
        self.fight.player_debuffs.strength_down = 0;
        self.fight.player_buffs.dexterity -= self.fight.player_debuffs.dexterity_down;
        self.fight.player_debuffs.dexterity_down = 0;
        self.fight.player_buffs.strength += self.fight.player_buffs.ritual;
        self.fight.player_debuffs.entangled = false;
        self.fight.player_debuffs.no_draw = false;
        self.fight.player_buffs.double_tap = 0;
        for i in self.fight.enemies.indicies() {
            self.fight.enemies[i].block = 0;
        }
        if self.fight.player_buffs.metallicize > 0 {
            self.player_gain_block(self.fight.player_buffs.metallicize, false);
        }
        if self.fight.player_buffs.end_turn_lose_hp > 0 {
            self.player_lose_hp(self.fight.player_buffs.end_turn_lose_hp, true);
        }
        let damage_all_enemies = self.fight.player_buffs.end_turn_damage_all_enemies;
        if damage_all_enemies > 0 {
            for idx in self.fight.enemies.indicies() {
                self.damage_enemy(damage_all_enemies, idx.0 as usize, false);
            }
        }
    }

    //TODO handle various effects of HP loss.
    fn player_lose_hp(&mut self, amount: i32, from_card: bool) {
        if amount <= 0 {
            return;
        }
        self.fight.player_buffs.num_times_lost_hp += 1;
        self.player_hp -= amount;
        if from_card && self.fight.player_buffs.rupture > 0 {
            self.apply_buff_to_player(Buff::Strength(self.fight.player_buffs.rupture));
        }
    }

    fn player_lose_max_hp(&mut self, amount: i32) {
        self.player_max_hp -= amount;
        if self.player_max_hp < 1 {
            self.player_max_hp = 1;
        }
        self.player_hp = min(self.player_hp, self.player_max_hp);
    }

    fn play_card_targets(&mut self, card_idx: usize, target: usize) -> Choice {
        let fight = &mut self.fight;
        if !fight.is_playable(card_idx) {
            panic!("Attempted to play an unplayable card.");
        }
        //Cards are small and cheap to clone. They aren't copy because they are mutable.
        //Remove the card before playing any actions so it can't upgrade itself.
        let card = fight.hand.remove(card_idx);
        let cost = fight.evaluate_cost(&card).expect("Card is playable");
        assert!(fight.energy >= cost);
        //Record the cost of an X spell before it is spent.
        let x = fight.energy;
        fight.energy -= cost;
        let mut context = PlayCardContext {
            card,
            target,
            real_card: true,
            exhausts: false,
            effect_index: 0,
            x,
        };
        self.trigger_play_card_effects(&mut context);
        if let Some(choice) = self.resolve_actions(Some(context)) {
            return choice;
        }
        self.play_card_choice()
    }
    //There are 3 cases for this function:
    //1) A card play is in process and the card has more actions. In this case the function
    //will continue to resolve actions.
    //2) A card play is in process and the card has no more actions. The card will go
    // to discard or exhaust as appropriate.
    //3) There isn't a card play in progress. In this case actions that were queued for after
    // the card was played will resolve until that queue is empty.
    //This function returns a choice if a battle was won, lost, interrupted, or over
    fn resolve_actions(&mut self, mut context: Option<PlayCardContext>) -> Option<Choice> {
        //This uses a loop so it can be interruped in the middle
        //to get player input for a card like Armaments.
        loop {
            if self.player_hp <= 0 {
                self.player_hp = 0;
                return Some(Choice::Loss);
            }
            //If there are no more enemies alive end the battle.
            if self.fight.enemies.len() == 0 {
                return Some(self.win_battle());
            }
            if let Some(mut card_context) = context {
                if card_context.effect_index < card_context.card.actions().len() {
                    let action = card_context.card.actions()[card_context.effect_index];
                    card_context.effect_index += 1;
                    let next = self.perform_play_effect(action, &mut card_context);
                    //If the player needs to make a selection, break out of the loop. It will be
                    //resumed by calling resolve_actions again once the player makes their choice
                    //and the in-progress action is handled.
                    if let ActionControlFlow::SelectCards(select, select_action, t) = next {
                        return Some(Choice::SelectCardState(
                            card_context,
                            select_action,
                            select,
                            t,
                        ));
                    }
                    context = Some(card_context);
                } else {
                    //The card's actions are over.
                    for enemy_idx in self.fight.enemies.indicies() {
                        let enemy = &mut self.fight.enemies[enemy_idx];
                        if enemy.buffs.queued_block > 0 {
                            enemy.block += enemy.buffs.queued_block;
                            enemy.buffs.queued_block = 0;
                        }
                    }
                    card_context.card.temp_cost = None;
                    if card_context.card.body.card_type() == CardType::Power
                        || !card_context.real_card
                    {
                        //Do nothing for powers or duplicated cards, they just go away after playing.
                    } else if card_context.exhausts {
                        self.exhaust(card_context.card);
                    } else {
                        insert_sorted(card_context.card, &mut self.fight.discard_pile);
                    }
                    context = None
                }
            } else {
                //Cards like Havoc, Omniscience can queue up other cards to be played. If
                //this happens pop them off and play them until there are none left.
                if let Some(front) = self.fight.post_card_queue.pop_front() {
                    match front {
                        PostCardItem::PlayCard(mut play_card_context) => {
                            self.trigger_play_card_effects(&mut play_card_context);
                            context = Some(play_card_context);
                        }
                        PostCardItem::Draw(amount) => {
                            for _ in 0..amount {
                                self.fight.draw(&mut self.rng);
                            }
                        }
                        PostCardItem::GainBlock(amount) => {
                            self.player_gain_block(amount, false);
                        }
                        PostCardItem::DamageAll(amount) => {
                            for idx in self.fight.enemies.indicies() {
                                self.damage_enemy(amount, idx.0 as usize, false);
                            }
                        }
                        PostCardItem::GainEnergy(amount) => {
                            self.fight.energy += amount;
                        }
                        PostCardItem::DamageRandomEnemy(amount) => {
                            let enemy = self.choose_random_enemy();
                            self.damage_enemy(amount, enemy, false);
                        }
                    }
                } else {
                    return None;
                }
            }
        }
    }

    fn trigger_play_card_effects(&mut self, context: &mut PlayCardContext) {
        if self.fight.player_buffs.corruption && context.card.body.card_type() == CardType::Skill {
            context.exhausts = true;
        }
        if context.card.body.card_type() == CardType::Attack && self.fight.player_buffs.rage > 0 {
            self.fight
                .post_card_queue
                .push_back(PostCardItem::GainBlock(self.fight.player_buffs.rage));
        }
        if self.fight.player_buffs.double_tap > 0
            && context.card.body.card_type() == CardType::Attack
            && context.real_card
        {
            self.fight.player_buffs.double_tap -= 1;
            let mut new_context = context.clone();
            new_context.real_card = false;
            self.fight
                .post_card_queue
                .push_back(PostCardItem::PlayCard(new_context));
        }
        if context.card.body.card_type() == CardType::Skill {
            for idx in self.fight.enemies.indicies() {
                let enraged = self.fight.enemies[idx].buffs.enrage;
                if enraged > 0 {
                    Self::buff_enemy(&mut self.fight.enemies[idx], Buff::Strength(enraged));
                }
            }
        }
    }

    fn exhaust(&mut self, card: Card) {
        let body = card.body;
        let upgraded = card.is_upgraded();
        insert_sorted(card, &mut self.fight.exhaust);
        if self.fight.player_buffs.dark_embrace > 0 {
            self.fight
                .post_card_queue
                .push_back(PostCardItem::Draw(self.fight.player_buffs.dark_embrace));
        }
        if self.fight.player_buffs.fnp > 0 {
            self.fight
                .post_card_queue
                .push_back(PostCardItem::GainBlock(self.fight.player_buffs.fnp));
        }
        if body == CardBody::Sentinel {
            self.fight
                .post_card_queue
                .push_back(PostCardItem::GainEnergy(if upgraded { 3 } else { 2 }));
        }
    }

    fn win_battle(&mut self) -> Choice {
        self.fight = Fight::default();
        self.goto_map()
    }

    //Used for Shield Gremlin.
    fn defend_ally(&mut self, i: EnemyIdx, amount: i32) {
        let num_enemies = self.fight.enemies.len();
        //If there are no other enemies to shield it will protect itself.
        if num_enemies == 1 {
            self.fight.enemies[i].block += amount;
        } else {
            let mut chosen_enemy = self.rng.sample(num_enemies - 1);
            for j in 0..self.fight.enemies.enemies.len() {
                if let Some(enemy) = &mut self.fight.enemies.enemies[j] {
                    if j == i.0 as usize {
                        continue;
                    }
                    if chosen_enemy == 0 {
                        enemy.block += amount;
                    } else {
                        chosen_enemy -= 1;
                    }
                }
            }
        }
    }

    fn split(&mut self, i: EnemyIdx) {
        let hp = self.fight.enemies[i].hp;
        let name = self.fight.enemies[i].name;
        if name == crate::enemies::large_black_slime::ENEMY_NAME {
            let mut med_slime_1 = generate_med_black_slime(&mut self.rng);
            med_slime_1.max_hp = hp;
            med_slime_1.hp = hp;
            let mut med_slime_2 = generate_med_black_slime(&mut self.rng);
            med_slime_2.max_hp = hp;
            med_slime_2.hp = hp;
            self.fight.enemies[(i.0) as usize] = Some(med_slime_1);
            self.fight.enemies[(i.0 + 1) as usize] = Some(med_slime_2);
        } else if name == crate::enemies::large_green_slime::ENEMY_NAME {
            let mut med_slime_1 = generate_med_green_slime(&mut self.rng);
            med_slime_1.max_hp = hp;
            med_slime_1.hp = hp;
            let mut med_slime_2 = generate_med_green_slime(&mut self.rng);
            med_slime_2.max_hp = hp;
            med_slime_2.hp = hp;
            self.fight.enemies[(i.0) as usize] = Some(med_slime_1);
            self.fight.enemies[(i.0 + 1) as usize] = Some(med_slime_2);
        } else {
            panic!("Splitting not implemented for {}", name);
        }
    }

    fn add_card_to_hand(&mut self, card: Card) {
        if self.fight.hand.len() < 10 {
            insert_sorted(card, &mut self.fight.hand);
        } else {
            insert_sorted(card, &mut self.fight.discard_pile);
        }
    }

    fn put_on_top(&mut self, card: Card) {
        self.fight.deck.put_on_top(vec![card]);
    }

    fn attack_enemy(&mut self, card: &Card, amount: i32, target: usize) -> AttackResult {
        let strength = self.fight.player_buffs.strength
            * (match card.body {
                CardBody::HeavyBlade => {
                    if card.is_upgraded() {
                        5
                    } else {
                        3
                    }
                }
                _ => 1,
            });
        let mut damage: f32 = (amount + strength) as f32;
        let Some(enemy) = &mut self.fight.enemies[target] else {
            return AttackResult::default();
        };
        if enemy.debuffs.vulnerable > 0 {
            damage *= 1.5;
        }
        if self.fight.player_debuffs.weak > 0 {
            damage *= 0.75;
        }
        let damage = damage as i32;
        self.damage_enemy(damage, target, true)
    }

    fn damage_enemy(&mut self, mut damage: i32, target: usize, from_card: bool) -> AttackResult {
        let Some(enemy) = &mut self.fight.enemies[target] else {
            return AttackResult::default();
        };
        if damage <= 0 {
            return AttackResult::default();
        }
        if damage < enemy.block {
            enemy.block -= damage;
            damage = 0;
        } else {
            damage -= enemy.block;
            enemy.block = 0;
        }
        damage = min(damage, enemy.hp);
        enemy.hp -= damage as i32;
        if damage > 0 && enemy.buffs.asleep {
            enemy.buffs.asleep = false;
            enemy.buffs.metallicize = 0;
            //This is the AI state for Lagabulin when it wakes up. No other
            //enemies sleep so this is
            enemy.ai_state = 2;
        }
        if damage > 0 && from_card {
            if enemy.buffs.curl_up > 0 {
                enemy.buffs.queued_block += enemy.buffs.curl_up;
                enemy.buffs.curl_up = 0;
            }
            enemy.buffs.strength += enemy.buffs.angry;
        }
        let lethal = if enemy.hp <= 0 {
            if enemy.buffs.spore_cloud > 0 {
                self.fight.player_debuffs.vulnerable += 2;
            }
            self.fight.stolen_back_gold += enemy.buffs.stolen_gold;
            self.fight.enemies[target] = None;
            true
        } else {
            false
        };
        AttackResult {
            lethal,
            damage_dealt: damage,
        }
    }

    fn num_strikes(&self) -> i32 {
        let mut count = self.fight.deck.count(|card| card.body.is_strike());
        count += self
            .fight
            .hand
            .iter()
            .filter(|card| card.body.is_strike())
            .count();
        count += self
            .fight
            .discard_pile
            .iter()
            .filter(|card| card.body.is_strike())
            .count();
        count as i32
    }

    fn bonus_attack(&self, card: &Card) -> i32 {
        match card.body {
            CardBody::SearingBlow => {
                let upgrades = card.assoc_data.get_unlimited_upgrade();
                (upgrades * (upgrades + 7)) / 2
            }
            CardBody::PerfectedStrike => {
                self.num_strikes() * (if card.is_upgraded() { 3 } else { 2 })
            }
            CardBody::Rampage => card.assoc_data.get_bonus_damage(),
            _ => 0,
        }
    }

    fn gain_max_hp(&mut self, amount: i32) {
        self.player_max_hp += amount;
        self.player_hp += amount;
    }

    fn heal(&mut self, amount: i32) {
        self.player_hp = min(self.player_max_hp, self.player_hp + amount);
    }

    fn intends_to_attack(&mut self, target: usize) -> bool {
        if let Some(enemy) = &self.fight.enemies[target] {
            let behavior = (enemy.behavior)(&mut self.rng, &self.fight, &enemy, enemy.ai_state);
            for behave in behavior.1 {
                if let EnemyAction::Attack(_) = *behave {
                    return true;
                }
            }
        }
        false
    }
    fn gen_temp_card(&mut self, body: CardBody, costs_0_this_turn: bool) {
        let mut card = body.to_card();
        if self.fight.hand.len() < 10 {
            if costs_0_this_turn {
                card.temp_cost = Some(0);
            }
            insert_sorted(card, &mut self.fight.hand);
        } else {
            insert_sorted(card, &mut self.fight.discard_pile);
        }
    }
    fn choose_random_enemy(&mut self) -> usize {
        let num_targets = self.fight.enemies.len();
        let mut sample = self.rng.sample(num_targets);
        for idx in self.fight.enemies.indicies() {
            if sample == 0 {
                return idx.0 as usize;
            } else {
                sample -= 1;
            }
        }
        panic!("Something went wrong when selecting a target");
    }

    fn select_random_target(&mut self, card: &Card) -> usize {
        if card.requires_target() {
            self.choose_random_enemy()
        } else {
            0
        }
    }

    fn player_gain_block(&mut self, block: i32, from_card: bool) {
        let block = if from_card {
            let block = block + self.fight.player_buffs.dexterity;
            let mut block = block as f32;
            if self.fight.player_debuffs.frail > 0 {
                block *= 0.75;
            }
            block as i32
        } else {
            block
        };
        if block > 0 {
            if self.fight.player_buffs.juggernaut > 0 {
                self.fight
                    .post_card_queue
                    .push_back(PostCardItem::DamageRandomEnemy(
                        self.fight.player_buffs.juggernaut,
                    ));
            }
            self.fight.player_block += block;
        }
    }

    fn add_card_to_deck(&mut self, card: CardBody) {
        insert_sorted(card.to_card(), &mut self.base_deck);
    }

    fn lose_gold(&mut self, amount: i32) {
        assert!(amount > 0);
        self.gold -= amount;
        //It's a bug to have an option to pay more gold than the charachter has.
        assert!(self.gold > 0);
    }

    fn gain_gold(&mut self, amount: i32) {
        self.gold += amount;
    }

    fn setup_fight(&mut self) {
        self.fight = Default::default();
        let mut deck_cards = Vec::new();
        for card in self.base_deck.clone() {
            if card.innate() {
                self.add_card_to_hand(card);
            } else {
                deck_cards.push(card);
            }
        }
        self.fight.deck = Deck::shuffled(deck_cards);
        self.fight.energy = 3;
        //TODO handle relics that affect initial hand size.
        for _ in 0..(5_usize.saturating_sub(self.fight.hand.len())) {
            self.fight.draw(&mut self.rng);
        }
    }
}

fn apply_debuff_to_enemy(enemy: &mut Enemy, debuff: Debuff) {
    match debuff {
        Debuff::Vulnerable(amount) => {
            enemy.debuffs.vulnerable += amount;
        }
        Debuff::Weak(amount) => {
            enemy.debuffs.weak += amount;
        }
        Debuff::Frail(_)
        | Debuff::Entangled
        | Debuff::StrengthDown(_)
        | Debuff::NoDraw
        | Debuff::DexterityDown(_)
        | Debuff::MinusStrength(_)
        | Debuff::MinusDexterity(_) => {
            panic!("{:?} cannot be applied to enemies!", debuff);
        }
    }
}

fn choose_card_filter(cards: &Vec<Card>, filter: impl Fn(&Card) -> bool) -> Vec<SelectCardAction> {
    cards
        .iter()
        .enumerate()
        .filter(|x| filter(x.1))
        .map(|(i, _)| SelectCardAction::ChooseCard(i))
        .collect()
}

fn decrement(x: &mut i32) {
    if *x > 0 {
        *x -= 1;
    }
}

//This applies player debuffs that wind down at the end of turn.
//In the original STS there is a flag for if the debuff was applied this turn,
//but I'll just add 1 extra when applied. It has the same gameplay behavior.
//This could be replaced with something better later.
fn debuff_player_turn_wind_down(x: &mut i32, amount: i32) {
    if *x == 0 {
        *x = amount + 1;
    } else {
        *x += amount;
    }
}

impl Game {
    pub fn new(charachter: Charachter) -> Self {
        let mut rng = Rng::new();
        let map = ActMap::standard(&mut rng);
        match charachter {
            Charachter::IRONCLAD => Game {
                player_hp: 80,
                player_max_hp: 80,
                max_potion_slots: 3,
                charachter,
                fight: Fight::new(),
                gold: 99,
                floor: 0,
                base_deck: vec![
                    CardBody::Bash.to_card(),
                    CardBody::Defend.to_card(),
                    CardBody::Defend.to_card(),
                    CardBody::Defend.to_card(),
                    CardBody::Defend.to_card(),
                    CardBody::Strike.to_card(),
                    CardBody::Strike.to_card(),
                    CardBody::Strike.to_card(),
                    CardBody::Strike.to_card(),
                    CardBody::Strike.to_card(),
                ],
                relics: Relics::new(),
                relic_pool: RelicPool::new(),
                rng,
                map,
                act: Act::new(),
            },
            Charachter::SILENT => todo!(),
            Charachter::DEFECT => todo!(),
            Charachter::WATCHER => todo!(),
        }
    }

    pub fn start(self) -> ChoiceState {
        let choice = self.goto_map();
        ChoiceState {
            game: Box::new(self),
            choice,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Charachter {
    IRONCLAD,
    SILENT,
    DEFECT,
    WATCHER,
}

impl Charachter {
    fn name(&self) -> &str {
        match self {
            Charachter::IRONCLAD => "Ironclad",
            Charachter::SILENT => "Silent",
            Charachter::DEFECT => "Defect",
            Charachter::WATCHER => "Watcher",
        }
    }
}
