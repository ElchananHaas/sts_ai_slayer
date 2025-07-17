use std::mem;

use crate::{
    card::{Buff, CardAssoc, CardType, IRONCLAD_ATTACK_CARDS, PlayEffect, SelectCardEffect},
    fight::{PlayCardContext, PostCardItem},
    game::{
        ActionControlFlow, Game, apply_debuff_to_enemy,
        choice::{SelectCardAction, SelectionPile},
        choose_card_filter,
    },
    util::insert_sorted,
};

impl Game {
    pub(super) fn handle_selected_action(
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
            SelectCardEffect::ExhaustToHand => {
                let card = match action {
                    SelectCardAction::ChooseCard(idx) => self.fight.exhaust.remove(idx),
                };
                self.add_card_to_hand(card);
            }
        }
    }

    pub(super) fn handle_action(
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
                self.player_gain_block(block, true);
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
                SelectCardEffect::ExhaustToHand => {
                    let targets = choose_card_filter(&self.fight.exhaust, |_| true);
                    if targets.len() > 0 {
                        return ActionControlFlow::SelectCards(
                            targets,
                            select_effect,
                            SelectionPile::Exhaust,
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
                            real_card: true,
                            exhausts: true,
                            effect_index: 0,
                            x: self.fight.energy,
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
                self.player_gain_block(self.fight.player_block, true);
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
            PlayEffect::ExhaustNonAttackForBlock(amount) => {
                let mut temp_hand = Vec::new();
                mem::swap(&mut temp_hand, &mut self.fight.hand);
                let mut count = 0;
                for card in temp_hand {
                    if card.body.card_type() == CardType::Attack {
                        self.fight.hand.push(card);
                    } else {
                        self.exhaust(card);
                        count += 1;
                    }
                }
                for _ in 0..count {
                    self.player_gain_block(amount, false);
                }
            }
            PlayEffect::ExhaustNonAttack => {
                let mut temp_hand = Vec::new();
                mem::swap(&mut temp_hand, &mut self.fight.hand);
                for card in temp_hand {
                    if card.body.card_type() == CardType::Attack {
                        self.fight.hand.push(card);
                    } else {
                        self.exhaust(card);
                    }
                }
            }
            PlayEffect::SpotWeakness(amount) => {
                if self.intends_to_attack(target) {
                    self.apply_buff_to_player(Buff::Strength(amount));
                }
            }
            PlayEffect::AttackAllX(attack) => {
                for _ in 0..context.x {
                    for enemy in self.fight.enemies.indicies() {
                        self.attack_enemy(&context.card, attack, enemy.0 as usize);
                    }
                }
            }
            PlayEffect::AttackLethalEffect(attack, lethal_effect) => {
                let res = self.attack_enemy(&context.card, attack, target);
                if res.lethal {
                    match lethal_effect {
                        crate::card::LethalEffect::Gain3MaxHP => {
                            self.gain_max_hp(3);
                        }
                        crate::card::LethalEffect::Gain4MaxHP => {
                            self.gain_max_hp(4);
                        }
                    }
                }
            }
            PlayEffect::AttackFiendFire(amount) => {
                let mut temp_hand = Vec::new();
                mem::swap(&mut temp_hand, &mut self.fight.hand);
                let count = temp_hand.len();
                for card in temp_hand {
                    self.exhaust(card);
                }
                for _ in 0..count {
                    self.attack_enemy(&context.card, amount, target);
                }
            }
            PlayEffect::AddCardToDiscard(card_body) => {
                insert_sorted(card_body.to_card(), &mut self.fight.discard_pile);
            }
            PlayEffect::DoubleStrength => {
                self.fight.player_buffs.strength *= 2;
            }
            PlayEffect::AttackAllForHP(amount) => {
                let mut total = 0;
                for enemy in self.fight.enemies.indicies() {
                    total += self
                        .attack_enemy(&context.card, amount, enemy.0 as usize)
                        .damage_dealt;
                }
                if total > 0 {
                    self.heal(total);
                }
            }
        }
        ActionControlFlow::Continue
    }
}
