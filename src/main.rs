use crate::{
    agents::agent_helper::Agent,
    card::IRONCLAD_ATTACK_CARDS,
    ui::{
        fight_ui::UIState,
        ui_actor::{UIActor, UIEvent},
    },
};
use agents::agent_helper::SkipSingleChoiceAgent;
use agents::mcts_agent::MctsAgent;
use agents::random_agent::RandomAgent;
use game::{Charachter, Game};
use ratatui::widgets::Widget;
use rng::Rng;
use tokio::sync::mpsc;

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
    let (sender, receiver) = mpsc::channel(8);
    let mut ui_actor = UIActor::new(receiver, terminal);
    tokio::spawn(async move { ui_actor.run().await });
    while !choice.is_over() {
        sender
            .blocking_send(UIEvent::NewState(choice.clone()))
            .expect("Send message to UI actor");
        //println!("{}", &choice);
        agent.take_action(&mut choice, &mut rng);
    }
    //print!("{}", &choice);
    ratatui::restore();
    Ok(())
}
