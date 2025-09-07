use crate::game::{
    QUESTION_MONSTER_BASE_WEIGHT, QUESTION_SHOP_BASE_WEIGHT, QUESTION_TREASURE_BASE_WEIGHT,
    encounter::Encounter,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Act {
    pub question_monster_weight: i32,
    pub question_shop_weight: i32,
    pub question_treasure_weight: i32,
    pub question_mark_visits: i32,
    pub prior_floor_shop: bool,
    pub prior_elite: Option<Encounter>,
    pub number_of_fights: i32,
    pub prior_fights: [Option<Encounter>; 2],
    pub map_x: i32,
    pub map_y: i32,
}

impl Act {
    pub fn new() -> Act {
        Self {
            question_monster_weight: QUESTION_MONSTER_BASE_WEIGHT,
            question_shop_weight: QUESTION_SHOP_BASE_WEIGHT,
            question_treasure_weight: QUESTION_TREASURE_BASE_WEIGHT,
            question_mark_visits: 0,
            prior_floor_shop: false,
            prior_elite: None,
            number_of_fights: 0,
            prior_fights: [None, None],
            map_x: 0,
            map_y: -1
        }
    }
}
