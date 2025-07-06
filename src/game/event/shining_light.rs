use crate::{
    game::{Choice, EventAction, Game, event::EventRoom},
    rng::Rng,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ShiningLight;

fn damage_amount(game: &Game) -> i32 {
    return (game.player_max_hp as f32 * 0.20) as i32;
}

impl EventRoom for ShiningLight {
    fn get_actions(&self, _game: &Game) -> Vec<EventAction> {
        (0..=1).map(EventAction).collect()
    }

    fn take_action(self, game: &mut Game, action: EventAction) -> Choice {
        match action.0 {
            0 => {
                game.player_lose_hp(damage_amount(game), false);
                let mut items = Vec::new();
                for i in 0..game.base_deck.len() {
                    if game.base_deck[i].can_upgrade() {
                        items.push(i);
                    }
                }
                for _ in 0..2 {
                    if items.len() == 0 {
                        break;
                    }
                    let idx = game.rng.sample(items.len());
                    let card_idx = items.swap_remove(idx);
                    game.base_deck[card_idx].upgrade();
                }
                game.goto_map()
            },
            1 => {
                game.goto_map()
            }
            _ => panic!("Invalid action: {}", action.0),
        }
    }

    fn action_str(&self, game: &Game, action: EventAction) -> String {
        match action.0 {
            0 => {
                format!("Upgrade 2 random cards. Lost {} hp.", damage_amount(game))
            }
            1 => {
                format!("Leave.")
            }
            _ => panic!("Invalid action: {}.", action.0),
        }
    }

    fn name(&self) -> &'static str {
        "Shining Light"
    }

    fn new(_rng: &mut Rng) -> Self {
        ShiningLight
    }
}
