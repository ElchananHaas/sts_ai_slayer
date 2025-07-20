use crate::{
    enemies::{
        fungi_beast::generate_fungi_beast, gremlin_nob::generate_gremlin_nob,
        lagavulin::generate_lagavulin, sentry::generate_sentry,
    },
    game::{Choice, Game},
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Encounter {
    Lagavulin,
    GremlinNob,
    Sentries,
    EventMushrooms,
}

impl Game {
    pub(super) fn setup_encounter(&mut self, encounter: Encounter) -> Choice {
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
                self.fight.enemies[1] = Some(generate_sentry(&mut self.rng, 1));
                self.fight.enemies[2] = Some(generate_sentry(&mut self.rng, 0));
            }
            Encounter::EventMushrooms => {
                self.fight.enemies[0] = Some(generate_fungi_beast(&mut self.rng));
                self.fight.enemies[1] = Some(generate_fungi_beast(&mut self.rng));
                self.fight.enemies[2] = Some(generate_fungi_beast(&mut self.rng));
            }
        }
        self.play_card_choice()
    }
}
