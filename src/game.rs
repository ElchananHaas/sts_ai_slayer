use std::{collections::VecDeque, fmt::Display};

use crate::{
    card::{Buff, Card, CardEffect, Debuff, PlayEffect, SelectCardEffect},
    deck::Deck,
    enemies::{
        cultist::generate_cultist, green_louse::generate_green_louse, jaw_worm::generate_jaw_worm,
        med_black_slime::generate_med_black_slime, med_green_slime::generate_med_green_slime,
        red_louse::generate_red_louse,
    },
    fight::{Enemies, Enemy, EnemyAction, EnemyIdx, Fight},
    rng::Rng,
    util::insert_sorted,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Game {
    player_hp: i32,
    player_max_hp: i32,
    max_potion_slots: i32,
    charachter: Charachter,
    fight: Fight,
    base_deck: Vec<Card>,
    gold: i32,
    rng: Rng,
    //This queue is ordered with standard VecDequeue ordering. 
    //By default PlayEffects are pushed onto the back and popped from the front.
    action_queue: VecDeque<PlayEffect>,
    pub floor: i32,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ChoiceState<'a> {
    game: &'a mut Game,
    choice: Choice,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum PlayCardAction {
    //Play the i'th card in hand
    PlayCard(u8),
    //End the turn
    EndTurn,
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ChooseEnemyAction {
    //Target the i'th enemy
    pub enemy: u8,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum RewardStateAction {
    //Proceed to choosing the next node.
    Proceed,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Choice {
    //See if this can be improved for more allocation reuse.
    PlayCardState(Vec<PlayCardAction>),
    ChooseEnemyState(Vec<ChooseEnemyAction>, usize),
    Win,
    Loss,
    RewardState(Vec<RewardStateAction>),
    SelectCardState(PlayCardContext, SelectCardEffect, Vec<SelectCardAction>, SelectionType),
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PlayCardContext {
    card: Card,
    target: usize,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum SelectionType {
    //Proceed to choosing the next node.
    Hand,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum SelectCardAction {
    //Choose the i'th card
    ChooseCard(usize),
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
    SelectCards(Vec<SelectCardAction>, SelectCardEffect, SelectionType),
}
impl<'a> ChoiceState<'a> {
    pub fn is_over(&self) -> bool {
        match self.choice {
            Choice::Win | Choice::Loss => true,
            _ => false,
        }
    }

    //This function clones the choice state to another Game. It
    //will still behave differently due to the Rng returning different
    //results. This can be used to simulate different outcomes
    pub fn clone_to<'b>(&self, game: &'b mut Game) -> ChoiceState<'b> {
        *game = self.game.clone();
        return ChoiceState {
            game: game,
            choice: self.choice.clone(),
        };
    }

    //This function handles an action being taken.
    pub fn take_action(&mut self, action_idx: usize) {
        let game = &mut *self.game;
        //The choice is set on the next line in the match statement.
        //The issue is that Rust won't let the program consume choice by value
        //even though it is overwritten. This is solved by swapping in a temporary value.
        let mut choice = Choice::Loss;
        std::mem::swap(&mut choice, &mut self.choice);
        self.choice = match choice {
            Choice::PlayCardState(play_card_actions) => {
                let action = play_card_actions[action_idx];
                game.take_play_card_action(action)
            }
            Choice::ChooseEnemyState(choose_enemy_actions, card_idx) => {
                let action = choose_enemy_actions[action_idx];
                let card_idx = card_idx;
                game.take_choose_enemy_action(card_idx, action)
            }
            Choice::Win => {
                panic!("The game is won, no actions can be taken");
            }
            Choice::Loss => {
                panic!("The game is lost, no actions can be taken");
            }
            Choice::RewardState(reward_state_actions) => {
                let action = reward_state_actions[action_idx];
                game.take_reward_state_action(action)
            }
            Choice::SelectCardState(play_card_context, effect, select_card_actions, _selection_type) => {
                let action = select_card_actions[action_idx];
                game.handle_select_card_action(play_card_context, effect, action)
            }
        }
    }

    //Outside the game only immutable access should be granted.
    pub fn get_choice(&self) -> &Choice {
        &self.choice
    }

    //Outside the game only immutable access should be granted.
    pub fn get_game(&self) -> &Game {
        &self.game
    }

    pub fn action_str(&self, action_idx: usize) -> String {
        match &self.choice {
            Choice::PlayCardState(play_card_actions) => {
                let action = play_card_actions[action_idx];
                match action {
                    PlayCardAction::PlayCard(card_idx) => {
                        format!("{:?}", self.game.fight.hand[card_idx as usize].effect)
                    }
                    PlayCardAction::EndTurn => "End Turn".to_owned(),
                }
            }
            Choice::ChooseEnemyState(choose_enemy_actions, _) => {
                format!(
                    "Target {:?}",
                    self.game.fight.enemies[choose_enemy_actions[action_idx].enemy as usize]
                        .as_ref()
                        .map_or("", |enemy| enemy.name)
                )
            }
            Choice::Win => {
                panic!("Win state has no actions.")
            }
            Choice::Loss => {
                panic!("Loss state has no actions.")
            }
            Choice::RewardState(reward_state_actions) => match reward_state_actions[action_idx] {
                RewardStateAction::Proceed => "Proceed".to_owned(),
            },
            Choice::SelectCardState(_play_card_context, _effect, select_card_actions, selection_type) => {
                let action = select_card_actions[action_idx];
                match selection_type {
                    SelectionType::Hand => match action {
                        SelectCardAction::ChooseCard(choice) => {
                            format!("{:?}", self.game.fight.hand[choice as usize].effect)
                        } //SelectCardAction::None => "No Selection".to_owned(),
                    },
                }
            }
        }
    }

    pub fn num_actions(&self) -> usize {
        match &self.choice {
            crate::game::Choice::PlayCardState(play_card_actions) => play_card_actions.len(),
            crate::game::Choice::ChooseEnemyState(choose_enemy_actions, _) => {
                choose_enemy_actions.len()
            }
            crate::game::Choice::Win => 0,
            crate::game::Choice::Loss => 0,
            crate::game::Choice::RewardState(reward_state_actions) => reward_state_actions.len(),
            crate::game::Choice::SelectCardState(
                _play_card_context,
                _effect,
                select_card_actions,
                _selection_type,
            ) => select_card_actions.len(),
        }
    }
}

impl Game {
    //This function starts a fight in the given game. Useful for testing.
    pub fn start_fight(&mut self) -> Choice {
        self.draw_initial_hand();
        self.play_card_choice()
    }

    fn handle_select_card_action(
        &mut self,
        mut context: PlayCardContext,
        effect: SelectCardEffect,
        action: SelectCardAction,
    ) -> Choice {
        self.handle_selected_action(&mut context, effect, action);
        self.resolve_actions(context)
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
    fn take_play_card_action(&mut self, action: PlayCardAction) -> Choice {
        match action {
            PlayCardAction::PlayCard(idx) => {
                let card = &self.fight.hand[idx as usize];
                if card.effect.requires_target() {
                    return self.choose_enemy_choice(idx as usize);
                }
                //If a card doesn't require targets supply 0 as a target since it won't matter.
                return self.play_card_targets(idx as usize, 0);
            }
            PlayCardAction::EndTurn => self.enemy_phase(),
        }
    }

    fn take_choose_enemy_action(&mut self, card_idx: usize, action: ChooseEnemyAction) -> Choice {
        self.play_card_targets(card_idx, action.enemy as usize)
    }

    fn take_reward_state_action(&mut self, action: RewardStateAction) -> Choice {
        match &action {
            RewardStateAction::Proceed => {
                self.setup_jawworm_fight();
                self.play_card_choice()
            }
        }
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
                        if damage > self.fight.player_block {
                            let dealt = damage - self.fight.player_block;
                            self.player_lose_life(dealt);
                        } else {
                            self.fight.player_block -= damage;
                        }
                        if self.player_hp <= 0 {
                            self.player_hp = 0;
                            return Choice::Loss;
                        }
                    }
                    EnemyAction::Block(block) => {
                        self.fight.enemies[i].block += block;
                    }
                    EnemyAction::Buff(buff) => {
                        Self::enemy_buff(&mut self.fight.enemies[i], *buff);
                    }
                    EnemyAction::Debuff(debuff) => {
                        self.apply_debuff_to_player(*debuff);
                    }
                    EnemyAction::AddToDiscard(cards) => {
                        self.fight.discard_pile.extend(cards.into_iter().cloned());
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
            decrement(&mut self.fight.player_debuffs.vulnerable);
            decrement(&mut self.fight.player_debuffs.weak);
        }
        for _ in 0..5 {
            self.fight.draw(&mut self.rng);
        }
        self.fight.player_block = 0;
        self.fight.energy = 3;
    }

    fn discard_hand_end_of_turn(&mut self) {
        self.fight.discard_pile.append(&mut self.fight.hand);
        //TODO handle artifact.
        self.fight.player_buffs.strength -= self.fight.player_debuffs.strength_down;
        self.fight.player_debuffs.strength_down = 0;
        self.fight.player_debuffs.entangled = false;
        for idx in self.fight.enemies.indicies() {
            self.fight.enemies[idx].block = 0;
        }
    }
    //TODO handle various effects of HP loss.
    fn player_lose_life(&mut self, amount: i32) {
        self.player_hp -= amount;
    }

    fn play_card_targets(&mut self, card_idx: usize, target: usize) -> Choice {
        let fight = &mut self.fight;
        if !fight.is_playable(card_idx) {
            panic!("Attempted to play an unplayable card.");
        }
        //Cards are small and cheap to clone. They aren't copy because they are mutable.
        //Remove the card before playing any actions so it can't upgrade itself.
        let card = fight.hand.remove(card_idx);
        fight.energy -= card.cost.expect("Card has a cost");
        self.action_queue.extend(card.effect.actions());
        let context = PlayCardContext {
            card,
            target,
        };
        self.resolve_actions(context)
    }

    fn resolve_actions(&mut self, mut context: PlayCardContext) -> Choice {
        //This uses a while loop so it can be interruped in the middle
        //to get player input for a card like Armaments.
        while let Some(action) = self.action_queue.pop_front() {
            let next = self.handle_action(action, &mut context);
            //If the player needs to make a selection, break out of the loop. It will be
            //resumed by calling resolve_actions again once the player makes their choice
            //and the in-progress action is handled.
            if let ActionControlFlow::SelectCards(select, select_action, t) = next {
                return Choice::SelectCardState(context, select_action, select, t);
            }
            if self.player_hp <= 0 {
                self.player_hp = 0;
                return Choice::Loss;
            }
        }
        self.post_card_play();
        insert_sorted(context.card, &mut self.fight.discard_pile);
        if self.fight.enemies.len() == 0 {
            return self.win_battle();
        }
        return self.play_card_choice();
    }
    fn win_battle(&mut self) -> Choice {
        self.floor += 1;
        Choice::RewardState(vec![RewardStateAction::Proceed])
    }

    fn apply_debuff_to_player(&mut self, debuff: Debuff) {
        match debuff {
            Debuff::Vulnerable(amount) => {
                debuff_player_turn_wind_down(&mut self.fight.player_debuffs.vulnerable, amount);
            }
            Debuff::Weak(amount) => {
                debuff_player_turn_wind_down(&mut self.fight.player_debuffs.weak, amount);
            }
            Debuff::Frail(amount) => {
                debuff_player_turn_wind_down(&mut self.fight.player_debuffs.frail, amount);
            }
            Debuff::Entangled => {
                self.fight.player_debuffs.entangled = true;
            },
            Debuff::StrengthDown(x) => {
                self.fight.player_debuffs.strength_down += x;
            },
        }
    }

    fn apply_buff_to_player(&mut self, buff: Buff) {
        match buff {
            //TODO handle if player has negative strength.
            Buff::Strength(x) => {
                self.fight.player_buffs.strength += x;
            },
            Buff::Ritual(_) => todo!(),
            Buff::RitualSkipFirst(_) => unimplemented!("Player gets normal ritual"),
        }
    }

    fn enemy_buff(enemy: &mut Enemy, buff: Buff) {
        match buff {
            Buff::Strength(x) => {
                enemy.buffs.strength += x;
            }
            Buff::Ritual(x) => {
                enemy.buffs.ritual += x;
            }
            Buff::RitualSkipFirst(x) => {
                enemy.buffs.ritual_skip_first += x;
            }
        }
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
        }
        if name == crate::enemies::large_green_slime::ENEMY_NAME {
            let mut med_slime_1 = generate_med_green_slime(&mut self.rng);
            med_slime_1.max_hp = hp;
            med_slime_1.hp = hp;
            let mut med_slime_2 = generate_med_green_slime(&mut self.rng);
            med_slime_2.max_hp = hp;
            med_slime_2.hp = hp;
            self.fight.enemies[(i.0) as usize] = Some(med_slime_1);
            self.fight.enemies[(i.0 + 1) as usize] = Some(med_slime_2);
        }
        panic!("Splitting not implemented for {}", name);
    }

    fn post_card_play<'a>(&mut self) {
        for enemy_idx in self.fight.enemies.indicies() {
            let enemy = &mut self.fight.enemies[enemy_idx];
            if enemy.buffs.queued_block > 0 {
                enemy.block += enemy.buffs.queued_block;
                enemy.buffs.queued_block = 0;
            }
        }
    }

    fn handle_selected_action(&mut self, context: &mut PlayCardContext, effect: SelectCardEffect,action: SelectCardAction) {
        match effect {
            SelectCardEffect::UpgradeCardInHand => {
                let card = match action {
                    SelectCardAction::ChooseCard(idx) => &mut self.fight.hand[idx],
                };
                upgrade(card);
            }
        }
    }

    fn attack_enemy(&mut self, amount: i32, target: usize) {
        let mut damage: f32 = (amount + self.fight.player_buffs.strength) as f32;
        let Some(enemy) = &mut self.fight.enemies[target] else {
            return;
        };
        if enemy.debuffs.vulnerable > 0 {
            damage *= 1.5;
        }
        if self.fight.player_debuffs.weak > 0 {
            damage *= 0.75;
        }
        let mut damage = damage as i32;
        if damage < enemy.block {
            enemy.block -= damage;
            damage = 0;
        } else {
            damage -= enemy.block;
            enemy.block = 0;
        }
        damage = std::cmp::min(damage, enemy.hp);
        if damage > 0 {
            if enemy.buffs.curl_up > 0 {
                enemy.buffs.queued_block += enemy.buffs.curl_up;
                enemy.buffs.curl_up = 0;
            }
            enemy.buffs.strength += enemy.buffs.angry;
        }
        enemy.hp -= damage as i32;
        if enemy.hp <= 0 {
            if enemy.buffs.spore_cloud > 0 {
                self.fight.player_debuffs.vulnerable += 2;
            }
            self.fight.stolen_back_gold += enemy.buffs.stolen_gold;
            self.fight.enemies[target] = None;
        }
    }
    fn handle_action(&mut self, action: PlayEffect, context: &mut PlayCardContext) -> ActionControlFlow {
        let card = &mut context.card;
        let target = context.target;
        match action {
            PlayEffect::Attack(attack) => {
                self.attack_enemy(attack, target);
            }
            PlayEffect::AttackEqualBlock => {
                self.attack_enemy(self.fight.player_block, target);
            }
            PlayEffect::AttackAll(amount) => {
                for enemy in self.fight.enemies.indicies() {
                    self.attack_enemy(amount, enemy.0 as usize);
                }
            }
            PlayEffect::DebuffEnemy(debuff) => {
                //This handles the case where the enemy dies during the card effect.
                let Some(enemy) = &mut self.fight.enemies[target] else {
                    return ActionControlFlow::Continue;
                };
                apply_debuff_to_enemy(enemy, debuff);
            },
            PlayEffect::DebuffSelf(debuff) => {
                self.apply_debuff_to_player(debuff);
            },
            PlayEffect::Buff(buff) => {
                self.apply_buff_to_player(buff);
            }
            PlayEffect::Block(block) => {
                //TODO handle player buffs and debuffs.
                let mut block = block as f32;
                if self.fight.player_debuffs.frail > 0 {
                    block *= 0.75;
                }
                let block = block as i32;
                self.fight.player_block += block;
            }
            PlayEffect::AddCopyToDiscard => {
                insert_sorted(card.clone(), &mut self.fight.discard_pile);
            }
            PlayEffect::SelectCardEffect(select_effect) => match select_effect {
                SelectCardEffect::UpgradeCardInHand => {
                    let mut upgrade_targets: Vec<SelectCardAction> = Vec::new();
                    for i in 0..self.fight.hand.len() {
                        if self.fight.hand[i].effect.upgraded().is_some() {
                            upgrade_targets.push(SelectCardAction::ChooseCard(i));
                        }
                    }
                    if upgrade_targets.len() > 0 {
                        return ActionControlFlow::SelectCards(
                            upgrade_targets,
                            select_effect,
                            SelectionType::Hand,
                        );
                    }
                }
            },
            PlayEffect::UpgradeAllCardsInHand => {
                for card in &mut self.fight.hand {
                    upgrade(card);
                }
            },
            PlayEffect::PlayExhaustTop => {
                todo!("Implement Havoc's effect");
            }
        }
        ActionControlFlow::Continue
    }
}

fn upgrade(card: &mut Card) {
    if let Some(upgraded) = card.effect.upgraded() {
        let current_cost = card.cost;
        let base_cost = card.effect.default_cost();
        card.effect = upgraded;
        if current_cost == base_cost {
            card.cost = upgraded.default_cost();
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
        Debuff::Frail(_) => {
            panic!("Frail cannot be applied to enemies!");
        }
        Debuff::Entangled => {
            panic!("Entangled cannot be applied to enemies!");
        },
        Debuff::StrengthDown(_) => {
            panic!("Strength down cannot be applied to enemies!");
        },
    }
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
        match charachter {
            Charachter::IRONCLAD => Game {
                player_hp: 80,
                player_max_hp: 80,
                max_potion_slots: 3,
                charachter,
                fight: Fight::new(),
                gold: 99,
                floor: 0,
                action_queue: VecDeque::new(),
                base_deck: vec![
                    CardEffect::Strike.to_card(),
                    CardEffect::Strike.to_card(),
                    CardEffect::Strike.to_card(),
                    CardEffect::Strike.to_card(),
                    CardEffect::Strike.to_card(),
                    CardEffect::Defend.to_card(),
                    CardEffect::Defend.to_card(),
                    CardEffect::Defend.to_card(),
                    CardEffect::Defend.to_card(),
                    CardEffect::Bash.to_card(),
                ],
                rng: Rng::new(),
            },
            Charachter::SILENT => todo!(),
            Charachter::DEFECT => todo!(),
            Charachter::WATCHER => todo!(),
        }
    }

    pub fn draw_initial_hand(&mut self) {
        //TODO handle relics that affect initial hand size.
        //TODO handle innate cards.
        for _ in 0..5 {
            self.fight.draw(&mut self.rng);
        }
    }
    fn setup_fight(&mut self) {
        self.fight.enemies = Enemies {
            enemies: [const { None }; 5],
        };
        self.fight.hand.clear();
        self.fight.energy = 0;
        self.fight.player_block = 0;
        self.fight.deck = Deck::shuffled(self.base_deck.clone());
        self.fight.energy = 3;
    }

    pub fn setup_jawworm_fight(&mut self) -> ChoiceState {
        self.setup_fight();
        self.fight.enemies[0] = Some(generate_jaw_worm(&mut self.rng));
        let choice = self.start_fight();
        ChoiceState {
            game: self,
            choice: choice,
        }
    }

    pub fn setup_cultist_fight(&mut self) -> ChoiceState {
        self.setup_fight();
        self.fight.enemies[0] = Some(generate_cultist(&mut self.rng));
        let choice = self.start_fight();
        ChoiceState {
            game: self,
            choice: choice,
        }
    }

    pub fn setup_redlouse_fight(&mut self) -> ChoiceState {
        self.setup_fight();
        self.fight.enemies[0] = Some(generate_red_louse(&mut self.rng));
        let choice = self.start_fight();
        ChoiceState {
            game: self,
            choice: choice,
        }
    }

    pub fn setup_greenlouse_fight(&mut self) -> ChoiceState {
        self.setup_fight();
        self.fight.enemies[0] = Some(generate_green_louse(&mut self.rng));
        let choice = self.start_fight();
        ChoiceState {
            game: self,
            choice: choice,
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

impl<'a> Display for ChoiceState<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn dash_line(f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:-<80}\n", "")?;
            Ok(())
        }
        fn fmt_enemy(enemy: &Enemy, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "| ")?;
            write!(f, "{} | ", enemy.name)?;
            write!(f, "AI {} | ", enemy.ai_state)?;
            write!(f, "{}/{} hp | ", enemy.hp, enemy.max_hp)?;
            if enemy.block > 0 {
                write!(f, "{} block | ", enemy.block)?;
            }
            if enemy.buffs.strength > 0 {
                write!(f, "{} str | ", enemy.buffs.strength)?;
            }
            if enemy.buffs.ritual > 0 || enemy.buffs.ritual_skip_first > 0 {
                write!(
                    f,
                    "{} ritual | ",
                    enemy.buffs.ritual + enemy.buffs.ritual_skip_first
                )?;
            }
            if enemy.buffs.curl_up > 0 {
                write!(f, "{} curl up | ", enemy.buffs.curl_up)?;
            }
            if enemy.debuffs.vulnerable > 0 {
                write!(f, "{} vuln | ", enemy.debuffs.vulnerable)?;
            }
            write!(f, "\n")?;
            dash_line(f)?;
            Ok(())
        }
        let game = &*self.game;
        let state_name = match &self.choice {
            Choice::PlayCardState(_) => "PlayCard",
            Choice::ChooseEnemyState(_, _) => "ChooseEnemy",
            Choice::Win => "Win",
            Choice::Loss => "Loss",
            Choice::RewardState(_) => "Rewards",
            Choice::SelectCardState(_ctx, __effect, actions, _type) => "SelectCard",
        };
        dash_line(f)?;
        write!(f, "| ")?;
        write!(f, "{} | ", state_name)?;
        write!(f, "{} | ", game.charachter.name())?;
        write!(f, "{}/{} hp | ", game.player_hp, game.player_max_hp)?;
        write!(f, "{}⚡︎ | ", game.fight.energy)?;
        write!(f, "{} block | ", game.fight.player_block)?;
        write!(f, "floor {} | ", game.floor)?;
        write!(f, "\n")?;
        write!(f, "{:.<80}\n", "")?;
        write!(f, "| ")?;
        for card in &game.fight.hand {
            if let Some(cost) = card.cost {
                write!(f, "{:?} [{}] | ", card.effect, cost)?;
            } else {
                write!(f, "{:?} [x] | ", card.effect)?;
            }
        }
        write!(f, "\n")?;
        dash_line(f)?;
        for enemy_idx in game.fight.enemies.indicies() {
            let enemy = &game.fight.enemies[enemy_idx];
            fmt_enemy(enemy, f)?;
        }
        Ok(())
    }
}
