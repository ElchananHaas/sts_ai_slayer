use crate::{
    card::CardBody,
    game::{Choice, EventAction, Game, event::EventData},
};

pub struct BigFish;

fn heal_amount(game: &Game) -> i32 {
    return game.player_max_hp / 3;
}

impl EventData for BigFish {
    fn get_actions(&self, game: &Game) -> Vec<EventAction> {
        (0..=2).map(EventAction).collect()
    }

    fn take_action(&self, game: &mut Game, action: EventAction) -> Choice {
        match action.0 {
            0 => {
                game.heal(heal_amount(game));
                game.goto_map()
            }
            1 => {
                game.gain_max_hp(5);
                game.goto_map()
            }
            2 => {
                game.add_card_to_deck(CardBody::Regret);
                let relic = game.relic_pool.get_random_tier_relic(&mut game.rng);
                game.relics.add(relic); //TODO - handle bottle relics.
                game.goto_map()
            }
            _ => panic!("Invalid action: {}", action.0),
        }
    }

    fn action_str(&self, game: &Game, action: EventAction) -> String {
        match action.0 {
            0 => {
                format!("Heal {}.", heal_amount(game))
            }
            1 => "Heal 5 hp.".to_string(),
            2 => "Get cursed by Regret. Gain a random relic.".to_string(),
            _ => panic!("Invalid action: {}.", action.0),
        }
    }

    fn name(&self) -> &'static str {
        "Big Fish"
    }
}
