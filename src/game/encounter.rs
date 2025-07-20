use crate::{
    enemies::{
        blue_slaver::generate_blue_slaver, cultist::generate_cultist,
        fungi_beast::generate_fungi_beast, green_louse::generate_green_louse,
        gremlin_nob::generate_gremlin_nob, jaw_worm::generate_jaw_worm,
        lagavulin::generate_lagavulin, large_green_slime::generate_med_green_slime,
        looter::generate_looter, med_black_slime::generate_med_black_slime,
        red_louse::generate_red_louse, red_slaver::generate_red_slaver, sentry::generate_sentry,
        small_black_slime::generate_small_black_slime,
        small_green_slime::generate_small_green_slime,
    },
    fight::Enemy,
    game::{Choice, Game},
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Encounter {
    Lagavulin,
    GremlinNob,
    Sentries,
    EventMushrooms,
    StarterCultist,
    StarterJawWorm,
    StarterLouse,
    StarterSlimes,
    GremlinGang,
    LargeSlime,
    FiveSmallSlimes,
    BlueSlaver,
    RedSlaver,
    ThreeLouse,
    TwoMushrooms,
    ExordiumThugs,
    ExordiumWildlife,
    Looter,
}

impl Game {
    fn generate_random_louse(&mut self) -> Enemy {
        if self.rng.sample(2) == 0 {
            generate_green_louse(&mut self.rng)
        } else {
            generate_red_louse(&mut self.rng)
        }
    }

    fn generate_slaver(&mut self) -> Enemy {
        if self.rng.sample(2) == 0 {
            generate_blue_slaver(&mut self.rng)
        } else {
            generate_red_slaver(&mut self.rng)
        }
    }

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
            Encounter::StarterCultist => {
                self.fight.enemies[0] = Some(generate_cultist(&mut self.rng));
            }
            Encounter::StarterJawWorm => {
                self.fight.enemies[0] = Some(generate_jaw_worm(&mut self.rng))
            }
            Encounter::StarterLouse => {
                for i in 0..2 {
                    self.fight.enemies[i] = Some(self.generate_random_louse());
                }
            }
            Encounter::StarterSlimes => {
                if self.rng.sample(2) == 0 {
                    self.fight.enemies[0] = Some(generate_small_black_slime(&mut self.rng));
                    self.fight.enemies[1] = Some(generate_med_green_slime(&mut self.rng));
                } else {
                    self.fight.enemies[0] = Some(generate_small_green_slime(&mut self.rng));
                    self.fight.enemies[1] = Some(generate_med_black_slime(&mut self.rng));
                }
            }
            Encounter::BlueSlaver => {
                self.fight.enemies[0] = Some(generate_blue_slaver(&mut self.rng));
            }
            Encounter::ExordiumThugs => {
                let sample = self.rng.sample(3);
                let front_enemy = match sample {
                    0 => self.generate_random_louse(),
                    1 => generate_med_black_slime(&mut self.rng),
                    2 => generate_med_green_slime(&mut self.rng),
                    _ => {
                        panic!("Unexpected rng result!")
                    }
                };
                self.fight.enemies[0] = Some(front_enemy);
                let sample = self.rng.sample(3);
                let back_enemy = match sample {
                    0 => generate_cultist(&mut self.rng),
                    1 => generate_looter(&mut self.rng),
                    2 => self.generate_slaver(),
                    _ => {
                        panic!("Unexpected rng result!")
                    }
                };
                self.fight.enemies[1] = Some(back_enemy);
            }
            Encounter::ExordiumWildlife => {}
        }
        self.play_card_choice()
    }
}
