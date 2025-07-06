use std::cmp::min;

use crate::{
    game::{Choice, EventAction, Game, event::EventRoom},
    rng::Rng,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WorldOfGoop {
    loss: i32,
}

impl EventRoom for WorldOfGoop {
    fn get_actions(&self, _game: &Game) -> Vec<EventAction> {
        (0..=1).map(EventAction).collect()
    }

    fn take_action(self, game: &mut Game, action: EventAction) -> Choice {
        match action.0 {
            0 => {
                game.gain_gold(75);
                game.player_lose_hp(11, false);
                game.goto_map()
            }
            1 => {
                game.lose_gold(min(self.loss, game.gold));
                game.goto_map()
            }
            _ => panic!("Invalid action: {}", action.0),
        }
    }

    fn action_str(&self, game: &Game, action: EventAction) -> String {
        match action.0 {
            0 => {
                format!("Gain 75 gold. Lose 11 hp.")
            }
            1 => {
                format!("Lose {} gold.", min(self.loss, game.gold))
            }
            _ => panic!("Invalid action: {}.", action.0),
        }
    }

    fn name(&self) -> &'static str {
        "World of Goop"
    }

    fn new(rng: &mut Rng) -> Self {
        WorldOfGoop {
            loss: rng.sample_i32_inclusive(20, 50),
        }
    }
}
