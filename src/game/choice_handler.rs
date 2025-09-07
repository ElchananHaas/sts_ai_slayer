use smallvec::SmallVec;

use crate::{
    card::{
        COLORLESS_CARDS, CURSE_CARDS, CardCharachter, CardType, IRONCLAD_CARDS, SelectCardEffect,
        sample_card,
    },
    fight::PlayCardContext,
    game::{
        Game, QUESTION_MONSTER_BASE_WEIGHT, QUESTION_SHOP_BASE_WEIGHT,
        QUESTION_TREASURE_BASE_WEIGHT,
        choice::{
            Choice, ChooseEnemyAction, MapStateAction, PlayCardAction, RemoveCardAction,
            RestSiteAction, SelectCardAction, TransformCardAction, UpgradeCardAction,
        },
        encounter::{Encounter},
    },
    map::RoomType,
};

impl Game {
    pub(super) fn handle_remove_card_action(&mut self, removal: RemoveCardAction) -> Choice {
        self.base_deck.remove(removal.0);
        self.goto_map()
    }

    pub(super) fn handle_rest_site_action(&mut self, action: RestSiteAction) -> Choice {
        match action {
            RestSiteAction::Heal => {
                self.heal((self.player_max_hp * 10) / 3);
                self.goto_map()
            }
            RestSiteAction::Upgrade => self.goto_upgrade_card(),
        }
    }

    pub(super) fn handle_transform_card_action(
        &mut self,
        transform: TransformCardAction,
    ) -> Choice {
        let card = self.base_deck.remove(transform.0);
        let transformed = if card.body.card_type() == CardType::Curse {
            sample_card(CURSE_CARDS, &mut self.rng)
        } else {
            match card.charachter() {
                CardCharachter::IRONCLAD => sample_card(IRONCLAD_CARDS, &mut self.rng),
                CardCharachter::SILENT => todo!(),
                CardCharachter::DEFECT => todo!(),
                CardCharachter::WATCHER => todo!(),
                CardCharachter::COLORLESS => sample_card(COLORLESS_CARDS, &mut self.rng),
            }
        };
        self.add_card_to_deck(transformed);
        self.goto_map()
    }

    pub(super) fn handle_upgrade_card_action(&mut self, upgrade: UpgradeCardAction) -> Choice {
        self.base_deck[upgrade.0].upgrade();
        self.goto_map()
    }

    pub(super) fn handle_select_card_action(
        &mut self,
        mut context: PlayCardContext,
        effect: SelectCardEffect,
        action: SelectCardAction,
    ) -> Choice {
        self.perform_selected_action(&mut context, effect, action);
        if let Some(choice) = self.resolve_actions(Some(context)) {
            return choice;
        }
        self.play_card_choice()
    }

    pub(super) fn handle_play_card_action(&mut self, action: PlayCardAction) -> Choice {
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

    pub(super) fn handle_choose_enemy_action(
        &mut self,
        card_idx: usize,
        action: ChooseEnemyAction,
    ) -> Choice {
        self.play_card_targets(card_idx, action.enemy as usize)
    }

    pub(super) fn handle_map_state_action(&mut self, action: MapStateAction) -> Choice {
        let prior_floor_shop = self.act.prior_floor_shop;
        self.act.prior_floor_shop = false;
        self.act.map_y += 1;
        self.floor += 1;
        match &action {
            MapStateAction::Forwards => {
                //Nothing
            }
            MapStateAction::Jump(x) => {
                self.act.map_x = *x;
            }
            MapStateAction::Left => {
                self.act.map_x -= 1;
            }
            MapStateAction::Right => {
                self.act.map_x += 1;
            }
        };
        match self.map.rooms[self.act.map_y as usize][self.act.map_x as usize].room_type {
            RoomType::QuestionMark => {
                let roll = self.rng.sample(100) as i32;
                let monster_weight =
                    self.act.question_monster_weight + self.act.question_mark_visits * 2;
                let shop_weight = if prior_floor_shop {
                    0
                } else {
                    self.act.question_shop_weight
                };
                if roll < monster_weight {
                    self.act.question_monster_weight = QUESTION_MONSTER_BASE_WEIGHT;
                    self.act.question_shop_weight += QUESTION_SHOP_BASE_WEIGHT;
                    self.act.question_treasure_weight += QUESTION_TREASURE_BASE_WEIGHT;
                    self.goto_fight()
                } else if roll < monster_weight + shop_weight {
                    self.act.question_monster_weight += QUESTION_MONSTER_BASE_WEIGHT;
                    self.act.question_shop_weight = QUESTION_SHOP_BASE_WEIGHT;
                    self.act.question_treasure_weight += QUESTION_TREASURE_BASE_WEIGHT;
                    self.goto_shop()
                } else if roll < monster_weight + shop_weight + self.act.question_treasure_weight {
                    self.act.question_monster_weight += QUESTION_MONSTER_BASE_WEIGHT;
                    self.act.question_shop_weight += QUESTION_SHOP_BASE_WEIGHT;
                    self.act.question_treasure_weight = QUESTION_TREASURE_BASE_WEIGHT;
                    self.goto_treasure()
                } else {
                    self.goto_event()
                }
            }
            RoomType::Shop => self.goto_shop(),
            RoomType::Treasure => self.goto_treasure(),
            RoomType::Rest => self.goto_rest_site(),
            RoomType::Monster => self.goto_fight(),
            RoomType::Elite => {
                let mut elites: SmallVec<[Encounter; 3]> = SmallVec::new();
                for encounter in &[
                    Encounter::GremlinNob,
                    Encounter::Lagavulin,
                    Encounter::Sentries,
                ] {
                    if self.act.prior_elite != Some(*encounter) {
                        elites.push(*encounter);
                    }
                }
                let encounter_idx = self.rng.sample(elites.len());
                self.setup_encounter(elites[encounter_idx])
            }
            RoomType::Unassigned => {
                panic!("Somehow reached an unassigned room!")
            }
        }
    }
}
