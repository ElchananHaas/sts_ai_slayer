use crate::{
    card::CardBody,
    game::{Choice, Game, choice::EventAction, event::EventRoom},
    rng::Rng,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TheSsserpent;

impl EventRoom for TheSsserpent {
    fn get_actions(&self, _game: &Game) -> Vec<EventAction> {
        (0..=1).map(EventAction).collect()
    }

    fn take_action(self, game: &mut Game, action: EventAction) -> Choice {
        match action.0 {
            0 => {
                game.gain_gold(175);
                game.add_card_to_deck(CardBody::Doubt);
                game.goto_map()
            }
            1 => game.goto_map(),
            _ => panic!("Invalid action: {}", action.0),
        }
    }

    fn action_str(&self, _game: &Game, action: EventAction) -> String {
        match action.0 {
            0 => {
                format!("Gain 175 gold. Become cursed - Doubt")
            }
            1 => {
                format!("Refuse.")
            }
            _ => panic!("Invalid action: {}.", action.0),
        }
    }

    fn name(&self) -> &'static str {
        "The Ssssserpent"
    }

    fn new(_rng: &mut Rng) -> Self {
        TheSsserpent
    }
}
