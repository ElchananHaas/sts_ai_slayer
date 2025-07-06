use smallvec::{SmallVec, smallvec};

use crate::{
    game::{
        Choice, EventAction, Game,
        encounter::Encounter,
        event::{Event, EventRoom},
    },
    rng::Rng,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DeadAdventurer {
    pub elite: DeadAdventurerElite,
    pub loots: SmallVec<[DeadAdventurerLoot; 3]>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DeadAdventurerElite {
    Lagavulin,
    GremlinNob,
    Sentries,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DeadAdventurerLoot {
    Nothing,
    Gold,
    Relic,
}

impl DeadAdventurer {
    fn loot_taken(&self) -> usize {
        3 - self.loots.len()
    }
    fn elite_arrives_chance(&self) -> usize {
        let c = self.loot_taken();
        25 + 25 * c
    }
}

impl EventRoom for DeadAdventurer {
    fn get_actions(&self, game: &Game) -> Vec<EventAction> {
        vec![EventAction(0), EventAction(1)]
    }

    fn take_action(mut self, game: &mut Game, action: EventAction) -> Choice {
        match action.0 {
            0 => game.goto_map(),
            1 => {
                let chance = self.elite_arrives_chance();
                let elite_arrived = game.rng.sample(100) < chance;
                if elite_arrived {
                    let encounter = game.setup_encounter(match self.elite {
                        DeadAdventurerElite::Lagavulin => Encounter::GremlinNob,
                        DeadAdventurerElite::GremlinNob => Encounter::GremlinNob,
                        DeadAdventurerElite::Sentries => Encounter::Sentries,
                    });
                    if self.elite == DeadAdventurerElite::Lagavulin {
                        //Lagavulin starts debuffing in this fight.
                        game.fight.enemies[0]
                            .as_mut()
                            .expect("Enemy generated")
                            .ai_state = 5;
                    }
                    let mut gold = 0;
                    let mut relics = 0;
                    for loot in &self.loots {
                        match loot {
                            DeadAdventurerLoot::Nothing => {}
                            DeadAdventurerLoot::Gold => {
                                gold = 30;
                            }
                            DeadAdventurerLoot::Relic => {
                                relics += 1;
                            }
                        }
                    }
                    game.fight.rewards.gold_min = gold;
                    game.fight.rewards.gold_max = gold;
                    game.fight.rewards.relic_count = relics;
                    encounter
                } else {
                    let c = game.rng.sample(self.loots.len());
                    let loot = (&mut self.loots).remove(c);
                    match loot {
                        DeadAdventurerLoot::Nothing => {}
                        DeadAdventurerLoot::Gold => {
                            game.gain_gold(30);
                        }
                        DeadAdventurerLoot::Relic => {
                            let relic = game.relic_pool.get_random_tier_relic(&mut game.rng);
                            game.relics.add(relic);
                        }
                    }
                    if self.loots.len() == 0 {
                        game.goto_map()
                    } else {
                        let actions = self.get_actions(game);
                        Choice::Event(Event::DeadAdventurer(self), actions)
                    }
                }
            }
            _ => panic!("Invalid action: {}", action.0),
        }
    }

    fn action_str(&self, _game: &Game, action: EventAction) -> String {
        match action.0 {
            0 => "Leave".to_string(),
            1 => format!(
                "Loot. {}% chance elite arrives.",
                self.elite_arrives_chance()
            ),
            _ => panic!("Invalid action: {}.", action.0),
        }
    }

    fn name(&self) -> &'static str {
        "Dead Adventurer"
    }

    fn new(rng: &mut Rng) -> Self {
        let elite = match rng.sample(3) {
            0 => DeadAdventurerElite::GremlinNob,
            1 => DeadAdventurerElite::Lagavulin,
            2 => DeadAdventurerElite::Sentries,
            _ => panic!("RNG returned an unexpected value."),
        };
        Self {
            elite,
            loots: smallvec![
                DeadAdventurerLoot::Nothing,
                DeadAdventurerLoot::Gold,
                DeadAdventurerLoot::Relic
            ],
        }
    }
}
