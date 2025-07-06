use crate::{
    game::{
        Choice, EventAction, Game,
        encounter::Encounter,
        event::{Event, EventRoom},
    },
    rng::Rng,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ScrapOoze {
    pub click_count: i32,
}


impl ScrapOoze {
    fn odds(&self) -> i32 {
        25 + 10 * self.click_count
    }
    fn hp_loss(&self) -> i32 {
        3 + self.click_count
    }
}

impl EventRoom for ScrapOoze {
    fn get_actions(&self, _game: &Game) -> Vec<EventAction> {
        (0..=1).map(EventAction).collect()
    }

    fn take_action(mut self, game: &mut Game, action: EventAction) -> Choice {
        match action.0 {
            0 => {
                game.player_lose_hp(self.hp_loss(), false);
                if (game.rng.sample(100) as i32) < self.odds() {
                    let relic = game.relic_pool.get_random_tier_relic(&mut game.rng);
                    game.relics.add(relic);
                    game.goto_map()
                } else {
                    self.click_count += 1;
                    let actions = self.get_actions(game);
                    Choice::Event(Event::ScrapOoze(self), actions)
                }
            }
            1 => game.goto_map(),
            _ => panic!("Invalid action: {}", action.0),
        }
    }

    fn action_str(&self, _game: &Game, action: EventAction) -> String {
        match action.0 {
            0 => format!(
                "Lose {} hp. {}% chance of reward.",
                self.hp_loss(),
                self.odds()
            ),
            1 => "Leave".to_string(),
            _ => panic!("Invalid action: {}.", action.0),
        }
    }

    fn name(&self) -> &'static str {
        "Scrap Ooze"
    }

    fn new(_rng: &mut Rng) -> Self {
        Self {
            click_count: 0
        }
    }
}
