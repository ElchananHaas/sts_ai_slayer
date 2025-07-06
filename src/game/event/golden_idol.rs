use crate::{
    card::CardBody,
    game::{Choice, EventAction, Game, event::EventRoom},
    relic::Relic,
    rng::Rng,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GoldenIdol;

fn damage_amount(game: &Game) -> i32 {
    return (game.player_max_hp as f32 * 0.25) as i32;
}

fn lose_max_hp_amount(game: &Game) -> i32 {
    return (game.player_max_hp as f32 * 0.08) as i32;
}

impl EventRoom for GoldenIdol {
    fn get_actions(&self, _game: &Game) -> Vec<EventAction> {
        (0..=3).map(EventAction).collect()
    }

    fn take_action(self, game: &mut Game, action: EventAction) -> Choice {
        match action.0 {
            0 => {
                game.add_card_to_deck(CardBody::Injury);
                game.relics.add(Relic::GoldenIdol);
                game.goto_map()
            }
            1 => {
                game.player_lose_hp(damage_amount(game), false);
                game.relics.add(Relic::GoldenIdol);
                game.goto_map()
            }
            2 => {
                game.player_lose_max_hp(lose_max_hp_amount(game));
                game.relics.add(Relic::GoldenIdol);
                game.goto_map()
            }
            3 => game.goto_map(),
            _ => panic!("Invalid action: {}", action.0),
        }
    }

    fn action_str(&self, game: &Game, action: EventAction) -> String {
        match action.0 {
            0 => {
                format!("Become Cursed - Injury. Gain the golden idol.")
            }
            1 => {
                format!("Take {} damage. Gain the golden idol.", damage_amount(game))
            }
            2 => {
                format!(
                    "Lose {} max hp. Gain the golden idol.",
                    lose_max_hp_amount(game)
                )
            }
            3 => {
                format!("Leave.")
            }
            _ => panic!("Invalid action: {}.", action.0),
        }
    }

    fn name(&self) -> &'static str {
        "Golden Idol"
    }

    fn new(_rng: &mut Rng) -> Self {
        GoldenIdol
    }
}
