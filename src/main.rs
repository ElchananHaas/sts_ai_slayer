#![feature(random)]

use agents::agent_helper::SkipSingleChoiceAgent;
use game::{Charachter, ChoiceState, Game};
use agents::mcts_agent::MctsAgent;
use agents::random_agent::RandomAgent;
use rng::Rng;
use crate::agents::agent_helper::Agent;

mod card;
mod deck;
mod enemies;
mod fight;
mod game;
mod agents;
mod rng;
mod util;
fn main() {
    let mut game = Game::new(Charachter::IRONCLAD);
    let mut choice = game.setup_cultist_fight();
    let mut rng = Rng::new();
    let mut agent = SkipSingleChoiceAgent {agent: MctsAgent {}};
    while !choice.is_over() {
        println!("{}", &choice);
        agent.take_action(&mut choice, &mut rng);
    }
    println!("{}", &choice);
}
