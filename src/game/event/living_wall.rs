use crate::{
    game::{Choice, choice::EventAction, Game, event::EventRoom},
    rng::Rng,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LivingWall;

impl EventRoom for LivingWall {
    fn get_actions(&self, _game: &Game) -> Vec<EventAction> {
        (0..=2).map(EventAction).collect()
    }

    fn take_action(self, game: &mut Game, action: EventAction) -> Choice {
        match action.0 {
            0 => game.goto_remove_card(),
            1 => game.goto_transform_card(),
            2 => game.goto_upgrade_card(),
            _ => panic!("Invalid action: {}", action.0),
        }
    }

    fn action_str(&self, _game: &Game, action: EventAction) -> String {
        match action.0 {
            0 => {
                format!("Remove a card.")
            }
            1 => {
                format!("Transform a card.")
            }
            2 => {
                format!("Upgrade a card.")
            }
            _ => panic!("Invalid action: {}.", action.0),
        }
    }

    fn name(&self) -> &'static str {
        "Living Wall"
    }

    fn new(_rng: &mut Rng) -> Self {
        LivingWall
    }
}
