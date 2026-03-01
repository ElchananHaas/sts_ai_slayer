use crate::{
    agents::agent_helper::Agent,
    card::IRONCLAD_ATTACK_CARDS,
    ui::{
        ui_actor::{UIActor, UIEvent},
    },
};
use agents::agent_helper::SkipSingleChoiceAgent;
use agents::mcts_agent::MctsAgent;
use agents::random_agent::RandomAgent;
use futures::StreamExt;
use game::{Charachter, Game};
use rng::Rng;
use tokio::{sync::mpsc::{self, Sender}, task::{LocalSet, spawn_local}};

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
    let local_set = LocalSet::new();
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    local_set.block_on(&runtime, agent_play())?;
    Ok(())
}

fn human_play() {
    todo!()
}

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
    println!("Spawning UI");
    let ui_handle = spawn_local(async move { ui_actor.run().await });
    while !choice.is_over() {
        println!("Choicing UI");
        //This returns Err if the display is closed. In this case, exit the program.
        let Ok(_) = sender.send(UIEvent::NewState(choice.clone())).await else {
            break;
        };
        agent.take_action(&mut choice, &mut rng);
    }
    println!("Exiting..");
    //The UI actor has some code to restore terminal settings on drop. This 
    //join ensures it will be run. It already has a panic hook by default.
    ui_handle.await.expect("UI exited");
    //print!("{}", &choice);
    Ok(())
}

fn setup_keystream(sender: Sender<UIEvent>) {
    let mut reader = crossterm::event::EventStream::new();
    spawn_local(async move {
        while let Some(x) = reader.next().await {
            let event = x.expect("The crossterm event stream isn't broken.");
            let Ok(_) = sender.send(UIEvent::KeyPress(event)).await else {
                //If the UI is dead, shut down the keystream.
                return;
            };
        }
    });
}
