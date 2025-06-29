use crate::game::{Choice, EventAction, Game, event::EventData};

pub struct Cleric;

fn heal_amount(game: &Game) -> i32 {
    return game.player_max_hp / 4;
}

impl EventData for Cleric {
    fn get_actions(&self, game: &Game) -> Vec<EventAction> {
        let mut res = Vec::new();
        if game.gold > 35 {
            res.push(EventAction(0));
        }
        if game.gold > 50 {
            res.push(EventAction(1));
        }
        res.push(EventAction(2));
        res
    }

    fn take_action(&self, game: &mut Game, action: EventAction) -> Choice {
        match action.0 {
            0 => {
                game.lose_gold(35);
                game.heal(heal_amount(game));
                game.goto_map()
            }
            1 => {
                game.lose_gold(50);
                game.goto_remove_card()
            }
            2 => game.goto_map(),
            _ => panic!("Invalid action: {}", action.0),
        }
    }

    fn action_str(&self, game: &Game, action: EventAction) -> String {
        match action.0 {
            0 => {
                format!("Pay 35 gold: Heal {}.", heal_amount(game))
            }
            1 => "Pay 50 gold: Remove a card from your deck.".to_string(),
            2 => "Leave".to_string(),
            _ => panic!("Invalid action: {}.", action.0),
        }
    }

    fn name(&self) -> &'static str {
        "Big Fish"
    }
}
