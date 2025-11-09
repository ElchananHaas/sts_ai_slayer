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
use futures::StreamExt;
use game::{Charachter, Game};
use ratatui::widgets::Widget;
use rng::Rng;
use tokio::sync::mpsc::{self, Sender};

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
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()?
        .block_on(async { agent_play().await })
}

fn human_play() {}

async fn agent_play() -> Result<(), Box<dyn std::error::Error>> {
    let game = Game::new(Charachter::IRONCLAD);
    let mut rng = Rng::new();
    let mut agent = SkipSingleChoiceAgent {
        agent: MctsAgent {},
    };
    let mut choice = game.start();

    let (sender, receiver) = mpsc::channel(8);
    setup_keystream(sender.clone());
    let mut ui_actor = UIActor::new(receiver);
    tokio::spawn(async move { ui_actor.run().await });
    while !choice.is_over() {
        //This errors if the display is closed. In this case, exit the program.
        let Ok(_) = sender.send(UIEvent::NewState(choice.clone())).await else {
            break;
        };
        agent.take_action(&mut choice, &mut rng);
    }
    //print!("{}", &choice);
    Ok(())
}

fn setup_keystream(sender: Sender<UIEvent>) {
    let mut reader = ratatui::crossterm::event::EventStream::new();
    tokio::spawn(async move {
        while let Some(x) = reader.next().await {
            let event = x.expect("The crossterm event stream isn't broken.");
            let Ok(_) = sender.send(UIEvent::KeyPress(event)).await else {
                //If the UI is dead, shut down the keystream.
                return;
            };
        }
    });
}
