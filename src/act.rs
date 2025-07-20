use crate::game::{encounter::Encounter, QUESTION_MONSTER_BASE_WEIGHT, QUESTION_SHOP_BASE_WEIGHT, QUESTION_TREASURE_BASE_WEIGHT};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Act {
    pub question_monster_weight: i32,
    pub question_shop_weight: i32,
    pub question_treasure_weight: i32,
    pub question_mark_visits: i32,
    pub prior_floor_shop: bool,
    pub prior_elite: Option<Encounter>,
}

impl Act {
    pub fn new() -> Act {
        Self {
            question_monster_weight: QUESTION_MONSTER_BASE_WEIGHT,
            question_shop_weight: QUESTION_SHOP_BASE_WEIGHT,
            question_treasure_weight: QUESTION_TREASURE_BASE_WEIGHT,
            question_mark_visits: 0,
            prior_floor_shop: false,
            prior_elite: None
        }
    }
}