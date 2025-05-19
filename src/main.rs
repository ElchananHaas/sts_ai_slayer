#![feature(random)]

use agent::RandomAgent;
use game::{Charachter, Game};
use rng::Rng;

mod agent;
mod card;
mod deck;
mod fight;
mod game;
mod player_actions;
mod rng;

fn main() {
    let mut game = Game::new(Charachter::IRONCLAD);
    let mut choice = game.setup_jawworm_fight();
    let mut rng = Rng::new();
    let agent = RandomAgent {};
    while !choice.is_over() {
        choice = agent.take_action(choice, &mut rng);
        dbg!(&choice);
    }
    println!("Hello, world!");
}
