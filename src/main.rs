#![feature(random)]

use agent::RandomAgent;
use game::{Charachter, ChoiceState, Game};
use rng::Rng;

mod agent;
mod card;
mod deck;
mod enemies;
mod fight;
mod game;
mod rng;
fn main() {
    let mut game = Game::new(Charachter::IRONCLAD);
    let mut choice = game.setup_cultist_fight();
    let mut rng = Rng::new();
    let agent = RandomAgent {};
    while !choice.is_over() && !matches!(choice, ChoiceState::MapState { .. }) {
        println!("{}", &choice);
        choice = agent.take_action(choice, &mut rng);
    }
    println!("{}", &choice);
}
