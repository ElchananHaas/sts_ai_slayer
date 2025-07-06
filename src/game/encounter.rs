use crate::{
    enemies::{
        gremlin_nob::generate_gremlin_nob, lagavulin::generate_lagavulin, sentry::generate_sentry,
    },
    game::{Choice, Game},
};

pub enum Encounter {
    Lagavulin,
    GremlinNob,
    Sentries,
}

impl Game {
    pub fn setup_encounter(&mut self, encounter: Encounter) -> Choice {
        self.setup_fight();
        match encounter {
            Encounter::Lagavulin => {
                self.fight.enemies[0] = Some(generate_lagavulin(&mut self.rng));
            }
            Encounter::GremlinNob => {
                self.fight.enemies[0] = Some(generate_gremlin_nob(&mut self.rng));
            }
            Encounter::Sentries => {
                self.fight.enemies[0] = Some(generate_sentry(&mut self.rng, 0));
                self.fight.enemies[0] = Some(generate_sentry(&mut self.rng, 1));
                self.fight.enemies[0] = Some(generate_sentry(&mut self.rng, 0));
            }
        }
        self.play_card_choice()
    }
}
