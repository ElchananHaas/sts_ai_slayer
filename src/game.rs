use std::{fmt::Display, mem, vec};

use crate::{
    card::{
        Buff, Card, CardAssoc, CardBody, CardType, Debuff, IRONCLAD_ATTACK_CARDS, PlayEffect,
        SelectCardEffect,
    },
    deck::Deck,
    enemies::{
        cultist::generate_cultist, green_louse::generate_green_louse, jaw_worm::generate_jaw_worm,
        med_black_slime::generate_med_black_slime, med_green_slime::generate_med_green_slime,
        red_louse::generate_red_louse,
    },
    fight::{Enemy, EnemyAction, EnemyIdx, Fight, PlayCardContext, PostCardItem},
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
    SelectCardState(
        PlayCardContext,
        SelectCardEffect,
        Vec<SelectCardAction>,
        SelectionPile,
    ),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum SelectionPile {
    Hand,
    Discard,
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
    SelectCards(Vec<SelectCardAction>, SelectCardEffect, SelectionPile),
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
            Choice::SelectCardState(
                play_card_context,
                effect,
                select_card_actions,
                _selection_type,
            ) => {
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
                        format!("{:?}", self.game.fight.hand[card_idx as usize].body)
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
            Choice::SelectCardState(
                _play_card_context,
                _effect,
                select_card_actions,
                selection_type,
            ) => {
                let action = select_card_actions[action_idx];
                match selection_type {
                    SelectionPile::Hand => match action {
                        SelectCardAction::ChooseCard(choice) => {
                            format!("Select {:?}", self.game.fight.hand[choice as usize].body)
                        } //SelectCardAction::None => "No Selection".to_owned(),
                    },
                    SelectionPile::Discard => match action {
                        SelectCardAction::ChooseCard(choice) => {
                            format!(
                                "Select {:?}",
                                self.game.fight.discard_pile[choice as usize].body
                            )
                        }
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
        if let Some(choice) = self.resolve_actions(Some(context)) {
            return choice;
        }
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
    fn take_play_card_action(&mut self, action: PlayCardAction) -> Choice {
        match action {
            PlayCardAction::PlayCard(idx) => {
                let card = &self.fight.hand[idx as usize];
                if card.requires_target() {
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
                            self.fight.player_block = 0;
                            self.player_lose_hp(dealt, false);
                        } else {
                            self.fight.player_block -= damage;
                        }
                        if self.player_hp <= 0 {
                            self.player_hp = 0;
                            return Choice::Loss;
                        }
                        let player_spikiness = self.fight.player_buffs.temp_spikes;
                        if player_spikiness > 0 {
                            Self::damage_enemy(&mut self.fight.enemies[i], player_spikiness);
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
        }
        decrement(&mut self.fight.player_debuffs.vulnerable);
        decrement(&mut self.fight.player_debuffs.weak);
        decrement(&mut self.fight.player_debuffs.frail);
        self.fight.player_buffs.temp_spikes = 0;
        self.fight.player_buffs.rage = 0;
        for _ in 0..5 {
            self.fight.draw(&mut self.rng);
        }
        self.fight.player_block = 0;
        self.fight.energy = 3;
    }

    fn discard_hand_end_of_turn(&mut self) {
        let mut old_hand = Vec::new();
        mem::swap(&mut old_hand, &mut self.fight.hand);
        for mut card in old_hand {
            card.temp_cost = None;
            if card.ethereal() {
                self.exhaust(card);
            } else {
                insert_sorted(card, &mut self.fight.discard_pile);
            }
        }
        //TODO handle artifact.
        self.fight.player_buffs.strength -= self.fight.player_debuffs.strength_down;
        self.fight.player_debuffs.strength_down = 0;
        self.fight.player_debuffs.entangled = false;
        self.fight.player_debuffs.no_draw = false;
        for idx in self.fight.enemies.indicies() {
            self.fight.enemies[idx].block = 0;
        }
        if self.fight.player_buffs.metallicize > 0 {
            self.fight.player_block += self.fight.player_buffs.metallicize;
        }
        if self.fight.player_buffs.end_turn_lose_hp > 0 {
            self.player_lose_hp(self.fight.player_buffs.end_turn_lose_hp, true);
        }
        let damage_all_enemies = self.fight.player_buffs.end_turn_damage_all_enemies;
        if damage_all_enemies > 0 {
            for idx in self.fight.enemies.indicies() {
                let enemy = &mut self.fight.enemies[idx];
                Self::damage_enemy(enemy, damage_all_enemies);
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
        fight.energy -= cost;
        let context = PlayCardContext {
            card,
            target,
            exhausts: false,
            effect_index: 0,
        };
        self.trigger_play_card_effects(&context);
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
                    let next = self.handle_action(action, &mut card_context);
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
                    if card_context.card.body.card_type() == CardType::Power {
                        //Do nothing for powers, they just go away after playing.
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
                        PostCardItem::PlayCard(play_card_context) => {
                            self.trigger_play_card_effects(&play_card_context);
                            context = Some(play_card_context);
                        }
                        PostCardItem::Draw(amount) => {
                            for _ in 0..amount {
                                self.fight.draw(&mut self.rng);
                            }
                        }
                        PostCardItem::GainBlock(amount) => {
                            self.fight.player_block += amount;
                        }
                        PostCardItem::DamageAll(amount) => {
                            for idx in self.fight.enemies.indicies() {
                                Self::damage_enemy(&mut self.fight.enemies[idx], amount);
                            }
                        }
                    }
                } else {
                    return None;
                }
            }
        }
    }

    fn trigger_play_card_effects(&mut self, context: &PlayCardContext) {
        if context.card.body.card_type() == CardType::Attack && self.fight.player_buffs.rage > 0 {
            self.fight
                .post_card_queue
                .push_back(PostCardItem::GainBlock(self.fight.player_buffs.rage));
        }
    }

    fn exhaust(&mut self, card: Card) {
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
    }

    fn win_battle(&mut self) -> Choice {
        self.floor += 1;
        self.fight = Fight::default();
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
            }
            Debuff::StrengthDown(x) => {
                self.fight.player_debuffs.strength_down += x;
            }
            Debuff::NoDraw => {
                self.fight.player_debuffs.no_draw = true;
            }
        }
    }

    fn apply_buff_to_player(&mut self, buff: Buff) {
        match buff {
            //TODO handle if player has negative strength.
            Buff::Strength(x) => {
                self.fight.player_buffs.strength += x;
            }
            Buff::Ritual(_) => todo!(),
            Buff::RitualSkipFirst(_) => unimplemented!("Player gets normal ritual"),
            Buff::EndTurnLoseHP(x) => self.fight.player_buffs.end_turn_lose_hp += x,
            Buff::EndTurnDamageAllEnemies(x) => {
                self.fight.player_buffs.end_turn_damage_all_enemies += x
            }
            Buff::DarkEmbraceBuff => self.fight.player_buffs.dark_embrace += 1,
            Buff::EvolveBuff(x) => self.fight.player_buffs.evolve += x,
            Buff::FNPBuff(x) => self.fight.player_buffs.fnp += x,
            Buff::FireBreathingBuff(x) => self.fight.player_buffs.fire_breathing += x,
            Buff::TempSpikes(x) => self.fight.player_buffs.temp_spikes += x,
            Buff::Metallicize(x) => self.fight.player_buffs.metallicize += x,
            Buff::RageBuff(x) => self.fight.player_buffs.rage += x,
            Buff::RuptureBuff(x) => self.fight.player_buffs.rupture += x,
        }
    }

    fn enemy_buff(enemy: &mut Enemy, buff: Buff) {
        fn panic_not_apply_enemies(buff: Buff) -> ! {
            panic!("Buff {:?} doesn't apply to enemies", buff);
        }
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
            Buff::EndTurnDamageAllEnemies(_)
            | Buff::EndTurnLoseHP(_)
            | Buff::DarkEmbraceBuff
            | Buff::EvolveBuff(_)
            | Buff::FNPBuff(_)
            | Buff::FireBreathingBuff(_)
            | Buff::TempSpikes(_)
            | Buff::Metallicize(_)
            | Buff::RageBuff(_)
            | Buff::RuptureBuff(_) => {
                panic_not_apply_enemies(buff);
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

    fn handle_selected_action(
        &mut self,
        context: &mut PlayCardContext,
        effect: SelectCardEffect,
        action: SelectCardAction,
    ) {
        match effect {
            SelectCardEffect::UpgradeCardInHand => {
                let card = match action {
                    SelectCardAction::ChooseCard(idx) => &mut self.fight.hand[idx],
                };
                card.upgrade();
            }
            SelectCardEffect::DiscardToTop => {
                let card = match action {
                    SelectCardAction::ChooseCard(idx) => self.fight.discard_pile.remove(idx),
                };
                self.put_on_top(card);
            }
            SelectCardEffect::ExhaustChosen => {
                let card = match action {
                    SelectCardAction::ChooseCard(idx) => self.fight.hand.remove(idx),
                };
                self.exhaust(card);
            }
            SelectCardEffect::HandToTop => {
                let card = match action {
                    SelectCardAction::ChooseCard(idx) => self.fight.hand.remove(idx),
                };
                self.put_on_top(card);
            }
            SelectCardEffect::DuplicatePowerOrAttack(amount) => {
                let card = match action {
                    SelectCardAction::ChooseCard(idx) => &self.fight.hand[idx].clone(),
                };
                for _ in 0..amount {
                    //TODO - consider interactions with Genetic Algorithm.
                    self.add_card_to_hand(card.clone());
                }
            }
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

    //This function handles the effect of damage on enemy block. Returns the damage dealt.
    fn damage_enemy(enemy: &mut Enemy, mut damage: i32) -> i32 {
        if damage <= 0 {
            return 0;
        }
        if damage < enemy.block {
            enemy.block -= damage;
            damage = 0;
        } else {
            damage -= enemy.block;
            enemy.block = 0;
        }
        damage = std::cmp::min(damage, enemy.hp);
        enemy.hp -= damage as i32;
        damage
    }

    fn attack_enemy(&mut self, card: &Card, amount: i32, target: usize) {
        let strength = match card.body {
            CardBody::HeavyBlade => {
                self.fight.player_buffs.strength * (if card.is_upgraded() { 5 } else { 3 })
            }
            _ => 1,
        };
        let mut damage: f32 = (amount + strength) as f32;
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
        damage = Self::damage_enemy(enemy, damage);
        if damage > 0 {
            if enemy.buffs.curl_up > 0 {
                enemy.buffs.queued_block += enemy.buffs.curl_up;
                enemy.buffs.curl_up = 0;
            }
            enemy.buffs.strength += enemy.buffs.angry;
        }
        if enemy.hp <= 0 {
            if enemy.buffs.spore_cloud > 0 {
                self.fight.player_debuffs.vulnerable += 2;
            }
            self.fight.stolen_back_gold += enemy.buffs.stolen_gold;
            self.fight.enemies[target] = None;
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
    fn handle_action(
        &mut self,
        action: PlayEffect,
        context: &mut PlayCardContext,
    ) -> ActionControlFlow {
        let card = &mut context.card;
        let target = context.target;
        match action {
            PlayEffect::Draw(amount) => {
                for _ in 0..amount {
                    self.fight.draw(&mut self.rng);
                }
            }
            PlayEffect::Attack(attack) => {
                self.attack_enemy(
                    &context.card,
                    attack + self.bonus_attack(&context.card),
                    target,
                );
            }
            PlayEffect::AttackEqualBlock => {
                self.attack_enemy(&context.card, self.fight.player_block, target);
            }
            PlayEffect::AttackAll(amount) => {
                for enemy in self.fight.enemies.indicies() {
                    self.attack_enemy(&context.card, amount, enemy.0 as usize);
                }
            }
            PlayEffect::AttackRandomEnemy(amount) => {
                let target = self.choose_random_enemy();
                self.attack_enemy(&context.card, amount, target);
            }
            PlayEffect::DebuffEnemy(debuff) => {
                //This handles the case where the enemy dies during the card effect.
                let Some(enemy) = &mut self.fight.enemies[target] else {
                    return ActionControlFlow::Continue;
                };
                apply_debuff_to_enemy(enemy, debuff);
            }
            PlayEffect::DebuffAll(debuff) => {
                for idx in self.fight.enemies.indicies() {
                    let enemy = &mut self.fight.enemies[idx];
                    apply_debuff_to_enemy(enemy, debuff);
                }
            }
            PlayEffect::DebuffSelf(debuff) => {
                self.apply_debuff_to_player(debuff);
            }
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
            PlayEffect::ExhaustRandomInHand => {
                let idx = self.rng.sample(self.fight.hand.len());
                let card = self.fight.hand.remove(idx);
                self.exhaust(card);
            }
            PlayEffect::SelectCardEffect(select_effect) => match select_effect {
                SelectCardEffect::UpgradeCardInHand => {
                    let targets = choose_card_filter(&self.fight.hand, |card| card.can_upgrade());
                    if targets.len() > 0 {
                        return ActionControlFlow::SelectCards(
                            targets,
                            select_effect,
                            SelectionPile::Hand,
                        );
                    }
                }
                SelectCardEffect::DiscardToTop => {
                    let targets = choose_card_filter(&self.fight.discard_pile, |_| true);
                    if targets.len() > 0 {
                        return ActionControlFlow::SelectCards(
                            targets,
                            select_effect,
                            SelectionPile::Discard,
                        );
                    }
                }
                SelectCardEffect::ExhaustChosen => {
                    let targets = choose_card_filter(&self.fight.hand, |_| true);
                    if targets.len() > 0 {
                        return ActionControlFlow::SelectCards(
                            targets,
                            select_effect,
                            SelectionPile::Hand,
                        );
                    }
                }
                SelectCardEffect::HandToTop => {
                    let targets = choose_card_filter(&self.fight.hand, |_| true);
                    if targets.len() > 0 {
                        return ActionControlFlow::SelectCards(
                            targets,
                            select_effect,
                            SelectionPile::Hand,
                        );
                    }
                }
                SelectCardEffect::DuplicatePowerOrAttack(x) => {
                    let targets = choose_card_filter(&self.fight.hand, |card| {
                        let t = card.body.card_type();
                        t == CardType::Power || t == CardType::Attack
                    });
                    if targets.len() > 0 {
                        return ActionControlFlow::SelectCards(
                            targets,
                            select_effect,
                            SelectionPile::Hand,
                        );
                    }
                }
            },
            PlayEffect::UpgradeAllCardsInHand => {
                for card in &mut self.fight.hand {
                    card.upgrade();
                }
            }
            PlayEffect::PlayExhaustTop => {
                if let Some(card) = self.fight.remove_top_of_deck(&mut self.rng) {
                    let target = self.select_random_target(&card);
                    self.fight
                        .post_card_queue
                        .push_back(PostCardItem::PlayCard(PlayCardContext {
                            card,
                            target,
                            exhausts: true,
                            effect_index: 0,
                        }));
                }
            }
            PlayEffect::MarkExhaust => {
                context.exhausts = true;
            }
            PlayEffect::ShuffleInCard(body) => {
                self.fight.deck.shuffle_in(vec![body.to_card()]);
            }
            PlayEffect::LoseHP(x) => {
                self.player_lose_hp(x, true);
            }
            PlayEffect::GainEnergy(x) => {
                self.fight.energy += x;
            }
            PlayEffect::DropkickDraw => {
                if let Some(enemy) = &self.fight.enemies[target] {
                    if enemy.debuffs.vulnerable > 0 {
                        self.fight.energy += 1;
                        self.fight.draw(&mut self.rng);
                    }
                }
            }
            PlayEffect::DoubleBlock => {
                self.fight.player_block *= 2;
            }
            PlayEffect::GenerateAttackInfernal => {
                let idx = self.rng.sample(IRONCLAD_ATTACK_CARDS.len());
                self.gen_temp_card(IRONCLAD_ATTACK_CARDS[idx], true);
                todo!("Implement infernal blade!")
            }
            PlayEffect::AddCardToHand(body) => {
                self.gen_temp_card(body, false);
            }
            PlayEffect::IncreaseDamage(amount) => {
                card.assoc_data =
                    CardAssoc::BonusDamage(card.assoc_data.get_bonus_damage() + amount);
            }
        }
        ActionControlFlow::Continue
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
        }
        Debuff::StrengthDown(_) => {
            panic!("Strength down cannot be applied to enemies!");
        }
        Debuff::NoDraw => {
            panic!("No draw cannot be applied to enemies!");
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
                    CardBody::Strike.to_card(),
                    CardBody::Strike.to_card(),
                    CardBody::Strike.to_card(),
                    CardBody::Strike.to_card(),
                    CardBody::Strike.to_card(),
                    CardBody::Headbutt.to_card(),
                    CardBody::Defend.to_card(),
                    CardBody::Defend.to_card(),
                    CardBody::Defend.to_card(),
                    CardBody::Defend.to_card(),
                    CardBody::Bash.to_card(),
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
        self.fight = Default::default();
        self.fight.deck = Deck::shuffled(self.base_deck.clone());
        self.fight.energy = 3;
    }

    pub fn setup_jawworm_fight(&mut self) -> ChoiceState<'_> {
        self.setup_fight();
        self.fight.enemies[0] = Some(generate_jaw_worm(&mut self.rng));
        let choice = self.start_fight();
        ChoiceState {
            game: self,
            choice: choice,
        }
    }

    pub fn setup_cultist_fight(&mut self) -> ChoiceState<'_> {
        self.setup_fight();
        self.fight.enemies[0] = Some(generate_cultist(&mut self.rng));
        let choice = self.start_fight();
        ChoiceState {
            game: self,
            choice: choice,
        }
    }

    pub fn setup_redlouse_fight(&mut self) -> ChoiceState<'_> {
        self.setup_fight();
        self.fight.enemies[0] = Some(generate_red_louse(&mut self.rng));
        let choice = self.start_fight();
        ChoiceState {
            game: self,
            choice: choice,
        }
    }

    pub fn setup_greenlouse_fight(&mut self) -> ChoiceState<'_> {
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
            if let Some(cost) = game.fight.evaluate_cost(card) {
                write!(f, "{:?} [{}] | ", card.body, cost)?;
            } else {
                write!(f, "{:?} [x] | ", card.body)?;
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
