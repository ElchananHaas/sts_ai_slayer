use crate::{
    card::{
        COLORLESS_CARDS, CURSE_CARDS, CardCharachter, CardType, IRONCLAD_CARDS, SelectCardEffect,
        sample_card,
    },
    fight::PlayCardContext,
    game::{
        Game,
        choice::{
            Choice, ChooseEnemyAction, MapStateAction, PlayCardAction, RemoveCardAction,
            RestSiteAction, SelectCardAction, TransformCardAction, UpgradeCardAction,
        },
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
        self.map_y += 1;
        match &action {
            MapStateAction::Forwards => {
                //Nothing
            }
            MapStateAction::Jump(x) => {
                self.map_x = *x;
            }
            MapStateAction::Left => {
                self.map_x -= 1;
            }
            MapStateAction::Right => {
                self.map_x += 1;
            }
        };
        match self.map.rooms[self.map_y as usize][self.map_x as usize].room_type {
            RoomType::QuestionMark => (),
            RoomType::Shop => (),
            RoomType::Treasure => (),
            RoomType::Rest => self.goto_rest_site(),
            RoomType::Monster => (),
            RoomType::Elite => (),
            RoomType::Unassigned => {
                panic!("Somehow reached an unassigned room!")
            }
        }
    }
}
