use std::{error::Error, thread::{self, JoinHandle}};

use crate::{
    agents::agent_helper::Agent, card::IRONCLAD_ATTACK_CARDS, game::choice::ChoiceState, ui::ui_actor::{UIActor, UIEvent}
};
use agents::agent_helper::SkipSingleChoiceAgent;
use agents::mcts_agent::MctsAgent;
use agents::random_agent::RandomAgent;
use crossterm::event::EventStream;
use futures::StreamExt;
use game::{Charachter, Game};
use rng::Rng;
use tokio::{sync::mpsc::{self, Sender, UnboundedSender, unbounded_channel}, task::{LocalSet, spawn_local}};

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
fn main() -> Result<(), Box<dyn Error>> {
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

fn spawn_game_thread(sender: UnboundedSender<ChoiceState>) -> JoinHandle<()> {
    thread::spawn(move || {
        let game = Game::new(Charachter::IRONCLAD);
        let mut rng = Rng::new();
        let mut agent = SkipSingleChoiceAgent {
            agent: MctsAgent {},
        };
        let mut choice = game.start();
        while !choice.is_over() {
            if sender.send(choice.clone()).is_err() {
                // If the main thread isn't listening for messages anymore, return.
                // This can happen if the UI is shut down.
                return;
            }
            agent.take_action(&mut choice, &mut rng);
        }
    })
}
async fn agent_play() -> Result<(), Box<dyn Error>> {
    let (sender, receiver) = mpsc::channel(8);
    setup_keystream(sender.clone());
    let mut ui_actor = UIActor::new(receiver);
    let ui_handle = spawn_local(async move { ui_actor.run().await });
    let (game_sender, mut game_reciever) = unbounded_channel();
    spawn_game_thread(game_sender);
    while let Some(state) = game_reciever.recv().await {
        //This returns Err if the display is closed. In this case, exit the program.
        let Ok(_) = sender.send(UIEvent::NewState(state)).await else {
            break;
        };
        
    }
    //The UI actor has some code to restore terminal settings on drop. This 
    //join ensures it will be run. It already has a panic hook by default.
    ui_handle.await.expect("UI exited");
    Ok(())
}

fn setup_keystream(sender: Sender<UIEvent>) {
    let mut reader = EventStream::new();
    spawn_local(async move {
        while let Some(x) = StreamExt::next(&mut reader).await {
            let event = x.expect("The crossterm event stream isn't broken.");
            let Ok(_) = sender.send(UIEvent::KeyPress(event)).await else {
                //If the UI is dead, shut down the keystream.
                return;
            };
        }
    });
}
