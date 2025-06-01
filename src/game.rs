use std::fmt::Display;

use crate::{
    card::{Buff, Card, CardEffect, Debuff, PlayEffect},
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
    //See if this can be improved for morre allocation reuse.
    PlayCardState(Vec<PlayCardAction>),
    ChooseEnemyState(Vec<ChooseEnemyAction>, usize),
    Win,
    Loss,
    RewardState(Vec<RewardStateAction>),
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
        match &self.choice {
            Choice::PlayCardState(play_card_actions) => {
                let action = play_card_actions[action_idx];
                self.take_play_card_action(action);
            }
            Choice::ChooseEnemyState(choose_enemy_actions, card_idx) => {
                let action = choose_enemy_actions[action_idx];
                self.take_choose_enemy_action(*card_idx, action);
            }
            Choice::Win => {
                panic!("The game is won, no actions can be taken");
            }
            Choice::Loss => {
                panic!("The game is lost, no actions can be taken");
            }
            Choice::RewardState(reward_state_actions) => {
                let action = reward_state_actions[action_idx];
                self.take_reward_state_action(action);
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
        }
    }

    //This function starts a fight in the given game. Useful for testing.
    pub fn start_fight(game: &'a mut Game) -> Self {
        let mut state = ChoiceState {
            game,
            choice: Choice::PlayCardState(vec![]),
        };
        state.game.draw_initial_hand();
        state.goto_play_card_state();
        state
    }

    fn goto_play_card_state(&mut self) {
        let fight = &self.game.fight;
        let mut actions = vec![PlayCardAction::EndTurn];
        for i in 0..fight.hand.len() {
            if fight.is_playable(i) {
                actions.push(PlayCardAction::PlayCard(i as u8));
            }
        }
        self.choice = Choice::PlayCardState(actions)
    }

    fn goto_choose_enemy_state(&mut self, chosen_card_idx: usize) {
        let fight = &self.game.fight;
        let mut actions = vec![];
        for i in fight.enemies.indicies() {
            actions.push(ChooseEnemyAction { enemy: i.0 });
        }
        self.choice = Choice::ChooseEnemyState(actions, chosen_card_idx)
    }
    fn take_play_card_action(&mut self, action: PlayCardAction) {
        match action {
            PlayCardAction::PlayCard(idx) => {
                let card = &self.game.fight.hand[idx as usize];
                if card.effect.requires_target() {
                    self.goto_choose_enemy_state(idx as usize);
                    return;
                }
                //If a card doesn't require targets supply 0 as a target since it won't matter.
                self.play_card_targets(idx as usize, 0)
            }
            PlayCardAction::EndTurn => self.enemy_phase(),
        }
    }

    fn take_choose_enemy_action(&mut self, card_idx: usize, action: ChooseEnemyAction) {
        self.play_card_targets(card_idx, action.enemy as usize);
    }

    fn take_reward_state_action(&mut self, action: RewardStateAction) {
        match &action {
            RewardStateAction::Proceed => {
                self.game.setup_jawworm_fight();
                self.goto_play_card_state();
            },
        }
    }

    fn enemy_phase(&mut self) {
        self.discard_hand_end_of_turn();
        for i in self.game.fight.enemies.indicies() {
            let enemy_actions;
            {
                let enemy = &self.game.fight.enemies[i];
                enemy_actions =
                    (enemy.behavior)(&mut self.game.rng, &self.game.fight, enemy, enemy.ai_state);
                self.game.fight.enemies[i].ai_state = enemy_actions.0;
            }

            for action in enemy_actions.1 {
                match action {
                    EnemyAction::Attack(damage) => {
                        let enemy = &self.game.fight.enemies[i];
                        let damage = *damage + enemy.buffs.strength + enemy.buffs.implicit_strength;
                        let mut damage = damage as f32;
                        //Weak and vulnerable calculations require using floats then rounding down afterwards.
                        if enemy.debuffs.weak > 0 {
                            damage *= 0.75;
                        }
                        if self.game.fight.player_debuffs.vulnerable > 0 {
                            damage *= 1.5;
                        }
                        let damage = damage as i32;
                        if damage > self.game.fight.player_block {
                            let dealt = damage - self.game.fight.player_block;
                            self.player_lose_life(dealt);
                        } else {
                            self.game.fight.player_block -= damage;
                        }
                        if self.game.player_hp <= 0 {
                            self.game.player_hp = 0;
                            self.choice = Choice::Loss;
                            return;
                        }
                    }
                    EnemyAction::Block(block) => {
                        self.game.fight.enemies[i].block += block;
                    }
                    EnemyAction::Buff(buff) => {
                        Self::enemy_buff(&mut self.game.fight.enemies[i], *buff);
                    }
                    EnemyAction::Debuff(debuff) => {
                        self.apply_debuff_to_player(*debuff);
                    }
                    EnemyAction::AddToDiscard(cards) => {
                        self.game
                            .fight
                            .discard_pile
                            .extend(cards.into_iter().cloned());
                        //Sort for greater MCTS efficiency. Technically, this is different from STS
                        //with regards to All For One, but I will accept this for now.
                        self.game.fight.discard_pile.sort();
                    }
                    EnemyAction::Split => {
                        split(&mut self.game, i);
                    }
                    EnemyAction::DefendAlly(amount) => {
                        defend_ally(&mut self.game, i, *amount);
                    }
                    EnemyAction::Escape => {
                        self.game.fight.enemies.enemies[i.0 as usize] = None;
                    }
                    EnemyAction::StealGold(amount) => {
                        let steal_amount = std::cmp::min(self.game.gold, *amount);
                        self.game.gold -= steal_amount;
                        self.game.fight.enemies[i].buffs.stolen_gold += steal_amount;
                    }
                }
            }
        }
        if self.game.fight.enemies.len() == 0 {
            self.win_battle();
            return;
        }
        self.reset_for_next_turn();
        self.goto_play_card_state();
    }

    fn reset_for_next_turn(&mut self) {
        //TODO implement relics that affect energy.
        //TODO implement cards that affect energy.
        for enemy_idx in self.game.fight.enemies.indicies() {
            let enemy: &mut Enemy = &mut self.game.fight.enemies[enemy_idx];
            enemy.buffs.strength += enemy.buffs.ritual;
            //Cultists skip the ritual buff the turn they play it.
            enemy.buffs.ritual += enemy.buffs.ritual_skip_first;
            enemy.buffs.ritual_skip_first = 0;
            decrement(&mut enemy.debuffs.vulnerable);
            decrement(&mut enemy.debuffs.weak);
            decrement(&mut self.game.fight.player_debuffs.vulnerable);
            decrement(&mut self.game.fight.player_debuffs.weak);
        }
        for _ in 0..5 {
            self.game.fight.draw(&mut self.game.rng);
        }
        self.game.fight.player_block = 0;
        self.game.fight.energy = 3;
    }

    fn discard_hand_end_of_turn(&mut self) {
        //TODO handle retained cards.
        //TODO handle statuses+curses with effects at the end of turn.
        self.game
            .fight
            .discard_pile
            .append(&mut self.game.fight.hand);
        self.game.fight.player_debuffs.entangled = false;
        for idx in self.game.fight.enemies.indicies() {
            self.game.fight.enemies[idx].block = 0;
        }
    }
    //TODO handle various effects of HP loss.
    fn player_lose_life(&mut self, amount: i32) {
        self.game.player_hp -= amount;
    }

    fn play_card_targets(&mut self, card_idx: usize, target: usize) {
        let fight = &mut self.game.fight;
        //Cards are small and cheap to clone. They aren't copy because they are mutable.
        let card = fight.hand[card_idx].clone();
        if !fight.is_playable(card_idx) {
            panic!("Attempted to play an unplayable card.");
        }
        fight.energy -= card.cost.expect("Card has a cost");
        for action in card.effect.actions() {
            handle_action(self.game, card.clone(), action, target);
            if self.game.player_hp <= 0 {
                self.game.player_hp = 0;
                self.choice = Choice::Loss;
                return;
            }
        }
        post_card_play(self.game);
        let card = self.game.fight.hand.remove(card_idx);
        insert_sorted(card, &mut self.game.fight.discard_pile);
        if self.game.fight.enemies.len() == 0 {
            self.win_battle();
            return;
        }
        self.goto_play_card_state();
        return;
    }

    fn win_battle(&mut self) {
        self.game.floor += 1;
        self.choice = Choice::RewardState(vec![RewardStateAction::Proceed]);
    }

    fn apply_debuff_to_player(&mut self, debuff: Debuff) {
        match debuff {
            Debuff::Vulnerable(amount) => {
                debuff_player_turn_wind_down(
                    &mut self.game.fight.player_debuffs.vulnerable,
                    amount,
                );
            }
            Debuff::Weak(amount) => {
                debuff_player_turn_wind_down(&mut self.game.fight.player_debuffs.weak, amount);
            }
            Debuff::Frail(amount) => {
                debuff_player_turn_wind_down(&mut self.game.fight.player_debuffs.frail, amount);
            }
            Debuff::Entangled => {
                self.game.fight.player_debuffs.entangled = true;
            }
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
}

fn post_card_play<'a>(game: &'a mut Game) {
    for enemy_idx in game.fight.enemies.indicies() {
        let enemy = &mut game.fight.enemies[enemy_idx];
        if enemy.buffs.queued_block > 0 {
            enemy.block += enemy.buffs.queued_block;
            enemy.buffs.queued_block = 0;
        }
    }
}

fn handle_action<'a>(game: &'a mut Game, card: Card, action: &PlayEffect, target: usize) {
    match action {
        PlayEffect::Attack(attack) => {
            //TODO handle player buffs and debuffs.
            let mut damage: f32 = *attack as f32;
            let Some(enemy) = &mut game.fight.enemies[target] else {
                return;
            };
            if enemy.debuffs.vulnerable > 0 {
                damage *= 1.5;
            }
            if game.fight.player_debuffs.weak > 0 {
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
                    game.fight.player_debuffs.vulnerable += 2;
                }
                game.fight.stolen_back_gold += enemy.buffs.stolen_gold;
                game.fight.enemies[target] = None;
                return;
            }
        }
        PlayEffect::DebuffEnemy(debuff) => {
            //This handles the case where the enemy dies during the card effect.
            let Some(enemy) = &mut game.fight.enemies[target] else {
                return;
            };
            apply_debuff_to_enemy(enemy, *debuff);
        }
        PlayEffect::Block(block) => {
            //TODO handle player buffs and debuffs.
            let mut block = *block as f32;
            if game.fight.player_debuffs.frail > 0 {
                block *= 0.75;
            }
            let block = block as i32;
            game.fight.player_block += block;
        }
        PlayEffect::AddCopyToDiscard => {
            insert_sorted(card, &mut game.fight.discard_pile);
        }
        PlayEffect::UpgradeCardInHand => {
            todo!("Implement upgrading cards in hand.");
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

fn split(game: &mut Game, i: EnemyIdx) {
    let hp = game.fight.enemies[i].hp;
    let name = game.fight.enemies[i].name;
    if name == crate::enemies::large_black_slime::ENEMY_NAME {
        let mut med_slime_1 = generate_med_black_slime(&mut game.rng);
        med_slime_1.max_hp = hp;
        med_slime_1.hp = hp;
        let mut med_slime_2 = generate_med_black_slime(&mut game.rng);
        med_slime_2.max_hp = hp;
        med_slime_2.hp = hp;
        game.fight.enemies[(i.0) as usize] = Some(med_slime_1);
        game.fight.enemies[(i.0 + 1) as usize] = Some(med_slime_2);
    }
    if name == crate::enemies::large_green_slime::ENEMY_NAME {
        let mut med_slime_1 = generate_med_green_slime(&mut game.rng);
        med_slime_1.max_hp = hp;
        med_slime_1.hp = hp;
        let mut med_slime_2 = generate_med_green_slime(&mut game.rng);
        med_slime_2.max_hp = hp;
        med_slime_2.hp = hp;
        game.fight.enemies[(i.0) as usize] = Some(med_slime_1);
        game.fight.enemies[(i.0 + 1) as usize] = Some(med_slime_2);
    }
    panic!("Splitting not implemented for {}", name);
}

//Used for Shield Gremlin.
fn defend_ally(game: &mut Game, i: EnemyIdx, amount: i32) {
    let num_enemies = game.fight.enemies.len();
    //If there are no other enemies to shield it will protect itself.
    if num_enemies == 1 {
        game.fight.enemies[i].block += amount;
    } else {
        let mut chosen_enemy = game.rng.sample(num_enemies - 1);
        for j in 0..game.fight.enemies.enemies.len() {
            if let Some(enemy) = &mut game.fight.enemies.enemies[j] {
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
        ChoiceState::start_fight(self)
    }

    pub fn setup_cultist_fight(&mut self) -> ChoiceState {
        self.setup_fight();
        self.fight.enemies[0] = Some(generate_cultist(&mut self.rng));
        ChoiceState::start_fight(self)
    }

    pub fn setup_redlouse_fight(&mut self) -> ChoiceState {
        self.setup_fight();
        self.fight.enemies[0] = Some(generate_red_louse(&mut self.rng));
        ChoiceState::start_fight(self)
    }

    pub fn setup_greenlouse_fight(&mut self) -> ChoiceState {
        self.setup_fight();
        self.fight.enemies[0] = Some(generate_green_louse(&mut self.rng));
        ChoiceState::start_fight(self)
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
