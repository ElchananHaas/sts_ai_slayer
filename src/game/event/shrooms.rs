use crate::{
    card::CardBody,
    game::{Choice, choice::EventAction, Game, encounter::Encounter, event::EventRoom},
    relic::Relic,
    rng::Rng,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HypnotizingShrooms;

fn heal_amount(game: &Game) -> i32 {
    return game.player_max_hp / 4;
}

impl EventRoom for HypnotizingShrooms {
    fn get_actions(&self, _game: &Game) -> Vec<EventAction> {
        (0..=1).map(EventAction).collect()
    }

    fn take_action(self, game: &mut Game, action: EventAction) -> Choice {
        match action.0 {
            0 => {
                game.add_card_to_deck(CardBody::Injury);
                let encounter = game.setup_encounter(Encounter::EventMushrooms);
                game.fight.rewards.fixed_relic = Some(Relic::OddMushroom);
                encounter
            }
            1 => {
                game.heal(heal_amount(game));
                game.add_card_to_deck(CardBody::Parasite);
                game.goto_map()
            }
            _ => panic!("Invalid action: {}", action.0),
        }
    }

    fn action_str(&self, game: &Game, action: EventAction) -> String {
        match action.0 {
            0 => {
                format!("Fight.")
            }
            1 => {
                format!("Heal {}. Become Cursed - Parasite.", heal_amount(game))
            }
            _ => panic!("Invalid action: {}.", action.0),
        }
    }

    fn name(&self) -> &'static str {
        "Hypnotizing Mushrooms"
    }

    fn new(_rng: &mut Rng) -> Self {
        HypnotizingShrooms
    }
}
