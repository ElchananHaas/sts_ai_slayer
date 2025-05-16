use crate::{
    card::{Debuff, PlayEffect},
    fight::{Enemy, Fight},
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GameCommon {
    player_hp: i32,
    player_max_hp: i32,
    max_potion_slots: i32,
    charachter: Charachter,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PlayCardState {
    common: Box<GameCommon>,
    fight: Box<Fight>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ChooseEnemyState {
    common: Box<GameCommon>,
    fight: Box<Fight>,
    chosen_card: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GameOverState {
    common: Box<GameCommon>,
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Game {
    PlayCard(PlayCardState),
    ChooseEnemy(ChooseEnemyState),
    Loss(GameOverState),
    Win(GameOverState),
}

pub enum PlayCardAction {
    //Play the i'th card in hand
    PlayCard(u8),
    //End the turn
    EndTurn,
}

pub struct ChooseEnemyAction {
    //Target the i'th enemy
    pub enemy: u8,
}

fn play_card_targets(
    mut common: Box<GameCommon>,
    mut fight: Box<Fight>,
    card_idx: usize,
    target: usize,
) -> Game {
    let card = &fight.hand[card_idx];
    if !fight.is_playable(card_idx) {
        panic!("Attempted to play an unplayable card.");
    }
    fight.energy -= card.cost.expect("Card has a cost");
    for action in card.effect.actions() {
        handle_action(&mut common, &mut fight, action, target);
        if common.player_hp <= 0 {
            return Game::Loss(GameOverState { common });
        }
    }
    Game::PlayCard(PlayCardState { common, fight })
}

fn handle_action(
    common: &mut Box<GameCommon>,
    fight: &mut Box<Fight>,
    action: &PlayEffect,
    target: usize,
) {
    match action {
        PlayEffect::Attack(attack) => {
            //TODO handle player buffs and debuffs.
            let mut damage: f32 = *attack as f32;
            let enemy = fight.enemies[target].as_mut().expect("Valid enemy chosen");
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
            enemy.hp -= damage as i32;
        }
        PlayEffect::DebuffEnemy(debuff) => {
            apply_debuff_to_enemy(
                fight.enemies[target].as_mut().expect("Valid enemy chosen"),
                *debuff,
            );
        }
        PlayEffect::Block(block) => {
            //TODO handle player buffs and debuffs.
            fight.player_block += block;
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

impl PlayCardState {
    fn available_actions(&self) -> Vec<PlayCardAction> {
        let fight = &self.fight;
        let mut res = vec![PlayCardAction::EndTurn];
        for i in 0..fight.hand.len() {
            if fight.is_playable(i) {
                res.push(PlayCardAction::PlayCard(i as u8));
            }
        }
        res
    }

    fn take_action(self, action: PlayCardAction) -> Game {
        match action {
            PlayCardAction::PlayCard(idx) => {
                let card = &self.fight.hand[idx as usize];
                if card.effect.requires_target() {
                    return Game::ChooseEnemy(ChooseEnemyState {
                        common: self.common,
                        fight: self.fight,
                        chosen_card: idx as usize,
                    });
                }
                //If a card doesn't require targets supply 0 as a target since it won't matter.
                play_card_targets(self.common, self.fight, idx as usize, 0)
            }
            PlayCardAction::EndTurn => self.enemy_turn(),
        }
    }

    fn enemy_turn(self) -> Game {}
}

impl ChooseEnemyState {
    fn available_actions(&self) -> Vec<ChooseEnemyAction> {
        let fight = &self.fight;
        let mut res = vec![];
        for i in 0..fight.enemies.len() {
            if fight.enemies[i].is_some() {
                res.push(ChooseEnemyAction { enemy: i as u8 });
            }
        }
        res
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Charachter {
    IRONCLAD,
    SILENT,
    DEFECT,
    WATCHER,
}
