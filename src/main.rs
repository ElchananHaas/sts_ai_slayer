use crate::{agents::agent_helper::Agent, card::IRONCLAD_ATTACK_CARDS, ui::fight_ui::UIState};
use agents::agent_helper::SkipSingleChoiceAgent;
use agents::mcts_agent::MctsAgent;
use agents::random_agent::RandomAgent;
use game::{Charachter, Game};
use ratatui::widgets::Widget;
use rng::Rng;

mod act;
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
mod ui;
mod util;
fn main() {
    agent_play();
}

fn human_play() {}

fn agent_play() -> Result<(), Box<dyn std::error::Error>> {
    let game = Game::new(Charachter::IRONCLAD);
    let mut rng = Rng::new();
    let mut agent = SkipSingleChoiceAgent {
        agent: MctsAgent {},
    };
    let mut choice = game.start();
    let mut terminal = ratatui::init();
    while !choice.is_over() {
        terminal.draw(|frame|
         UIState::new(&choice).render(frame.area(), frame.buffer_mut()))
         .expect("Successfully drew frame");
        //println!("{}", &choice);
        agent.take_action(&mut choice, &mut rng);
    }    
    //print!("{}", &choice);
    ratatui::restore();
    Ok(())
}
