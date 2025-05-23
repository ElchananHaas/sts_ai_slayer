use std::fmt::Display;

use crate::{
    card::{Buff, Card, CardEffect, Debuff, PlayEffect},
    deck::Deck,
    enemies::{cultist::generate_cultist, jaw_worm::generate_jaw_worm},
    fight::{Enemies, Enemy, EnemyAction, Fight},
    rng::Rng,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Game {
    player_hp: i32,
    player_max_hp: i32,
    max_potion_slots: i32,
    charachter: Charachter,
    fight: Fight,
    base_deck: Vec<Card>,
    rng: Rng,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ChoiceState<'a> {
    PlayCardState(PlayCardState<'a>),
    ChooseEnemyState(ChooseEnemyState<'a>),
    WinState(&'a mut Game),
    LossState(&'a mut Game),
    MapState(&'a mut Game),
}

impl<'a> ChoiceState<'a> {
    pub fn is_over(&self) -> bool {
        match self {
            ChoiceState::WinState(_) => true,
            ChoiceState::LossState(_) => true,
            _ => false,
        }
    }
}
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct PlayCardState<'a> {
    game: &'a mut Game,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ChooseEnemyState<'a> {
    game: &'a mut Game,
    chosen_card: usize,
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

fn play_card_targets<'a>(game: &'a mut Game, card_idx: usize, target: usize) -> ChoiceState<'a> {
    let fight = &mut game.fight;
    let card = &fight.hand[card_idx];
    if !fight.is_playable(card_idx) {
        panic!("Attempted to play an unplayable card.");
    }
    fight.energy -= card.cost.expect("Card has a cost");
    for action in card.effect.actions() {
        handle_action(game, action, target);
        if game.player_hp <= 0 {
            return ChoiceState::LossState(game);
        }
    }
    let card = game.fight.hand.remove(card_idx);
    game.fight.discard_pile.push(card);
    if game.fight.enemies.len() == 0 {
        return ChoiceState::MapState(game);
    }
    ChoiceState::PlayCardState(PlayCardState { game })
}

//Returns if the action is interrupted due to an enemy dying.
fn handle_action<'a>(game: &'a mut Game, action: &PlayEffect, target: usize) {
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
            let mut damage = damage as i32;
            if damage < enemy.block {
                enemy.block -= damage;
                damage = 0;
            } else {
                damage -= enemy.block;
                enemy.block = 0;
            }
            damage = std::cmp::min(damage, enemy.hp);
            enemy.hp -= damage as i32;
            if enemy.hp <= 0 {
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
            game.fight.player_block += block;
        }
    }
}

fn apply_debuff_to_enemy(enemy: &mut Enemy, debuff: Debuff) {
    match debuff {
        Debuff::Vulnerable(amount) => {
            enemy.debuffs.vulnerable += amount;
        }
    }
}

impl<'a> PlayCardState<'a> {
    pub fn available_actions(&self) -> Vec<PlayCardAction> {
        let fight = &self.game.fight;
        let mut res = vec![PlayCardAction::EndTurn];
        for i in 0..fight.hand.len() {
            if fight.is_playable(i) {
                res.push(PlayCardAction::PlayCard(i as u8));
            }
        }
        res
    }

    pub fn take_action(self, action: PlayCardAction) -> ChoiceState<'a> {
        match action {
            PlayCardAction::PlayCard(idx) => {
                let card = &self.game.fight.hand[idx as usize];
                if card.effect.requires_target() {
                    return ChoiceState::ChooseEnemyState(ChooseEnemyState {
                        game: self.game,
                        chosen_card: idx as usize,
                    });
                }
                //If a card doesn't require targets supply 0 as a target since it won't matter.
                play_card_targets(self.game, idx as usize, 0)
            }
            PlayCardAction::EndTurn => self.enemy_phase(),
        }
    }

    fn enemy_phase(mut self) -> ChoiceState<'a> {
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
                        let damage = *damage + enemy.buffs.strength;
                        let damage = damage as f32;
                        //Weak and vulnerable calculations require using floats then rounding down afterwards.
                        let damage = damage as i32;
                        if damage > self.game.fight.player_block {
                            let dealt = damage - self.game.fight.player_block;
                            self.player_lose_life(dealt);
                        } else {
                            self.game.fight.player_block -= damage;
                        }
                        if self.game.player_hp <= 0 {
                            self.game.player_hp = 0;
                            return ChoiceState::LossState(self.game);
                        }
                    }
                    EnemyAction::Block(block) => {
                        self.game.fight.enemies[i].block += block;
                    }
                    EnemyAction::Buff(buff) => {
                        Self::enemy_buff(&mut self.game.fight.enemies[i], *buff);
                    }
                }
            }
        }
        self.reset_for_next_turn();
        ChoiceState::PlayCardState(PlayCardState { game: self.game })
    }

    fn discard_hand_end_of_turn(&mut self) {
        //TODO handle retained cards.
        //TODO handle statuses+curses with effects at the end of turn.
        self.game
            .fight
            .discard_pile
            .append(&mut self.game.fight.hand);
        for idx in self.game.fight.enemies.indicies() {
            self.game.fight.enemies[idx].block = 0;
        }
    }
    fn reset_for_next_turn(&mut self) {
        //TODO implement relics that affect energy.
        //TODO implement cards that affect energy.
        for enemy_idx in self.game.fight.enemies.indicies() {
            self.game.fight.enemies[enemy_idx].buffs.strength +=
                self.game.fight.enemies[enemy_idx].buffs.ritual;
            //Cultists skip the ritual buff the turn they play it.
            self.game.fight.enemies[enemy_idx].buffs.ritual +=
                self.game.fight.enemies[enemy_idx].buffs.ritual_skip_first;
            self.game.fight.enemies[enemy_idx].buffs.ritual_skip_first = 0;
        }
        for _ in 0..5 {
            self.game.fight.draw(&mut self.game.rng);
        }
        self.game.fight.player_block = 0;
        self.game.fight.energy = 3;
    }
    //TODO handle various effects of HP loss.
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
    //TODO handle various effects of HP loss.
    fn player_lose_life(&mut self, amount: i32) {
        self.game.player_hp -= amount;
    }
}

impl<'a> ChooseEnemyState<'a> {
    pub fn available_actions(&self) -> Vec<ChooseEnemyAction> {
        let fight = &self.game.fight;
        let mut res = vec![];
        for i in fight.enemies.indicies() {
            res.push(ChooseEnemyAction { enemy: i.0 });
        }
        res
    }

    pub fn take_action(self, action: ChooseEnemyAction) -> ChoiceState<'a> {
        play_card_targets(self.game, self.chosen_card, action.enemy as usize)
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
        self.draw_initial_hand();
        ChoiceState::PlayCardState(PlayCardState { game: self })
    }

    pub fn setup_cultist_fight(&mut self) -> ChoiceState {
        self.setup_fight();
        self.fight.enemies[0] = Some(generate_cultist(&mut self.rng));
        self.draw_initial_hand();
        ChoiceState::PlayCardState(PlayCardState { game: self })
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

impl<'a> PlayCardState<'a> {
    pub fn action_str(&self, action: PlayCardAction) -> String {
        match action {
            PlayCardAction::PlayCard(idx) => {
                format!("{:?}", self.game.fight.hand[idx as usize].effect)
            }
            PlayCardAction::EndTurn => "EndTurn".to_owned(),
        }
    }
}

impl<'a> ChooseEnemyState<'a> {
    pub fn action_str(&self, action: ChooseEnemyAction) -> String {
        match action {
            ChooseEnemyAction { enemy } => format!(
                "Target {:?}",
                self.game.fight.enemies[enemy as usize]
                    .as_ref()
                    .map_or("", |enemy| enemy.name)
            ),
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
                write!(
                    f,
                    "{} curl up | ",
                    enemy.buffs.curl_up
                )?;
            }
            if enemy.debuffs.vulnerable > 0 {
                write!(f, "{} vuln | ", enemy.debuffs.vulnerable)?;
            }
            write!(f, "\n")?;
            dash_line(f)?;
            Ok(())
        }
        let (state_name, game) = match self {
            ChoiceState::PlayCardState(play_card_state) => ("PlayCard", &*play_card_state.game),
            ChoiceState::ChooseEnemyState(choose_enemy_state) => {
                ("ChooseEnemy", &*choose_enemy_state.game)
            }
            ChoiceState::WinState(game) => ("Win", &**game),
            ChoiceState::LossState(game) => ("Loss", &**game),
            ChoiceState::MapState(game) => ("Map Choice", &**game),
        };
        dash_line(f)?;
        write!(f, "| ")?;
        write!(f, "{} | ", state_name)?;
        write!(f, "{} | ", game.charachter.name())?;
        write!(f, "{}/{} hp | ", game.player_hp, game.player_max_hp)?;
        write!(f, "{}⚡︎ | ", game.fight.energy)?;
        write!(f, "{} block | ", game.fight.player_block)?;
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
