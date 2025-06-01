#![feature(random)]

use game::{Charachter, ChoiceState, Game};
use mcts_agent::MctsAgent;
use random_agent::RandomAgent;
use rng::Rng;

mod card;
mod deck;
mod enemies;
mod fight;
mod game;
mod mcts_agent;
mod random_agent;
mod rng;
mod util;
fn main() {
    let mut game = Game::new(Charachter::IRONCLAD);
    let mut choice = game.setup_cultist_fight();
    let mut rng = Rng::new();
    let agent = MctsAgent {};
    while !choice.is_over() {
        println!("{}", &choice);
        choice = agent.take_action(choice, &mut rng);
    }
    println!("{}", &choice);
}
