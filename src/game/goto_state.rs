use crate::game::{
    Game,
    choice::{
        Choice, MapStateAction, RemoveCardAction, RestSiteAction, TransformCardAction,
        UpgradeCardAction,
    },
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
        if self.map_y == -1 {
            let row = &self.map.rooms[0];
            for i in 0..row.len() {
                if row[i].reachable {
                    actions.push(MapStateAction::Jump(i as i32));
                }
            }
        } else if self.map_y as usize == self.map.rooms.len() - 1 {
            actions.push(MapStateAction::Forwards);
        } else {
            let room = &self.map.rooms[self.map_y as usize][self.map_x as usize];
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

    pub(super) fn goto_fight(&mut self) -> Choice {
        //TODO - go to real fight!
        self.goto_map()
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
