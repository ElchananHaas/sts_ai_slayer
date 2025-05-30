#![feature(random)]

use mcts_agent::MctsAgent;
use random_agent::RandomAgent;
use game::{Charachter, ChoiceState, Game};
use rng::Rng;

mod random_agent;
mod card;
mod deck;
mod enemies;
mod fight;
mod game;
mod rng;
mod mcts_agent;
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
