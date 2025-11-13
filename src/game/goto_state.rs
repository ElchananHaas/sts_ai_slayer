use smallvec::SmallVec;

use crate::game::{
    Game,
    choice::{
        Choice, MapStateAction, RemoveCardAction, RestSiteAction, TransformCardAction,
        UpgradeCardAction,
    },
    encounter::{self, Encounter},
};

impl Game {
    pub(super) fn goto_transform_card(&mut self) -> Choice {
        let mut res = Vec::new();
        for i in 0..self.base_deck.len() {
            if self.base_deck[i].body.removable() {
                res.push(TransformCardAction(i));
            }
        }
        if res.len() == 0 {
            return self.goto_map();
        }
        Choice::TransformCardState(res)
    }

    pub(super) fn goto_upgrade_card(&mut self) -> Choice {
        let mut res = Vec::new();
        for i in 0..self.base_deck.len() {
            if self.base_deck[i].can_upgrade() {
                res.push(UpgradeCardAction(i));
            }
        }
        if res.len() == 0 {
            return self.goto_map();
        }
        Choice::UpgradeCardState(res)
    }

    pub(super) fn goto_rest_site(&mut self) -> Choice {
        Choice::RestSite(vec![RestSiteAction::Heal, RestSiteAction::Upgrade])
    }

    pub(super) fn goto_map(&self) -> Choice {
        let mut actions = Vec::new();
        if let Some(position) = self.act.position {
            if position.y as usize == self.map.rooms.len() - 1 {
                return Choice::Win;
            //TODO - handle going to the boss room.
            //actions.push(MapStateAction::Forwards);
            } else {
                let room = &self.map.rooms[position.y as usize][position.x as usize];
                if room.has_left_child {
                    actions.push(MapStateAction::Left);
                }
                if room.has_front_child {
                    actions.push(MapStateAction::Forwards);
                }
                if room.has_right_child {
                    actions.push(MapStateAction::Right);
                }
            }
        } else {
            let row = &self.map.rooms[0];
            for i in 0..row.len() {
                if row[i].reachable {
                    actions.push(MapStateAction::Jump(i as i32));
                }
            }
        }
        Choice::MapState(actions)
    }

    pub(super) fn goto_remove_card(&mut self) -> Choice {
        let mut res = Vec::new();
        for i in 0..self.base_deck.len() {
            if self.base_deck[i].body.removable() {
                res.push(RemoveCardAction(i));
            }
        }
        if res.len() == 0 {
            return self.goto_map();
        }
        Choice::RemoveCardState(res)
    }

    fn update_act_from_fight(&mut self, encounter: Encounter) {
        self.act.prior_fights[1] = self.act.prior_fights[0];
        self.act.prior_fights[0] = Some(encounter);
        self.act.number_of_fights += 1;
    }
    pub(super) fn goto_fight(&mut self) -> Choice {
        if self.act.number_of_fights < 3 {
            let mut encounters: SmallVec<[Encounter; 4]> = SmallVec::new();
            for encounter in [
                Encounter::StarterCultist,
                Encounter::StarterJawWorm,
                Encounter::StarterLouse,
                Encounter::StarterSlimes,
            ] {
                if Some(encounter) != self.act.prior_fights[0]
                    && Some(encounter) != self.act.prior_fights[1]
                {
                    encounters.push(encounter);
                }
            }
            let encounter = encounters[self.rng.sample(encounters.len())];
            self.update_act_from_fight(encounter);
            self.setup_encounter(encounter)
        } else {
            let hard_pool = [
                Encounter::BlueSlaver,
                Encounter::GremlinGang,
                Encounter::Looter,
                Encounter::LargeSlime,
                Encounter::FiveSmallSlimes,
                Encounter::ExordiumThugs,
                Encounter::ExordiumWildlife,
                Encounter::RedSlaver,
                Encounter::ThreeLouse,
                Encounter::TwoMushrooms,
            ];
            let weights = [4, 2, 4, 4, 2, 3, 3, 2, 4, 4];
            let encounter = loop {
                let idx = self.rng.sample_weighted(&weights);
                let encounter = hard_pool[idx];
                if Some(encounter) == self.act.prior_fights[0]
                    || Some(encounter) == self.act.prior_fights[1]
                {
                    continue;
                }
                if self.act.number_of_fights == 3 {
                    if encounter == Encounter::ThreeLouse
                        && self.act.prior_fights[0] == Some(Encounter::StarterLouse)
                    {
                        continue;
                    }
                    if (encounter == Encounter::LargeSlime
                        || encounter == Encounter::FiveSmallSlimes)
                        && self.act.prior_fights[0] == Some(Encounter::StarterSlimes)
                    {
                        continue;
                    }
                }
                break encounter;
            };
            self.update_act_from_fight(encounter);
            self.setup_encounter(encounter)
        }
    }

    pub(super) fn goto_shop(&mut self) -> Choice {
        //TODO - go to real shop!
        self.goto_map()
    }

    pub(super) fn goto_treasure(&mut self) -> Choice {
        //TODO - go to real treasure!
        self.goto_map()
    }

    pub(super) fn goto_event(&mut self) -> Choice {
        //TODO - go to real event!
        self.goto_map()
    }
}
