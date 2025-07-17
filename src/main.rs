use crate::{agents::agent_helper::Agent, card::IRONCLAD_ATTACK_CARDS};
use agents::agent_helper::SkipSingleChoiceAgent;
use agents::mcts_agent::MctsAgent;
use agents::random_agent::RandomAgent;
use game::{Charachter, Game};
use rng::Rng;

mod agents;
mod card;
mod deck;
mod enemies;
mod fight;
mod game;
mod map;
mod potion;
mod relic;
mod rng;
mod util;
fn main() {
    let mut game = Game::new(Charachter::IRONCLAD);
    let mut choice = game.setup_cultist_fight();
    let mut rng = Rng::new();
    let mut agent = SkipSingleChoiceAgent {
        agent: MctsAgent {},
    };
    while !choice.is_over() {
        println!("{}", &choice);
        agent.take_action(&mut choice, &mut rng);
    }
    print!("{}", &choice);
}
