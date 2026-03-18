use std::{
    error::Error,
    fs::File,
    io::{BufWriter, Write},
    thread::{self, JoinHandle},
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    agents::agent_helper::Agent,
    ui::ui_actor::{UIActor, UIEvent},
    util::GameLog,
};
use agents::agent_helper::SkipSingleChoiceAgent;
use agents::mcts_agent::MctsAgent;
use agents::random_agent::RandomAgent;
use crossterm::event::EventStream;
use futures::StreamExt;
use game::{Character, Game};
use rng::Rng;
use tokio::{
    sync::mpsc::{self, Sender},
    task::{LocalSet, spawn_local},
};

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

fn spawn_game_thread(sender: Sender<UIEvent>) -> JoinHandle<()> {
    thread::spawn(move || {
        let game = Game::new(Character::IRONCLAD);
        let game_seed = game.get_seed();
        let mut rng = Rng::new();
        let mut agent = SkipSingleChoiceAgent {
            agent: MctsAgent {},
        };
        let mut choice = game.start();
        let mut log = GameLog::new(game_seed);
        loop {
            if sender
                .blocking_send(UIEvent::NewState(choice.clone()))
                .is_err()
            {
                // If the main thread isn't listening for messages anymore, return.
                // This can happen if the UI is shut down.
                break;
            }
            if choice.is_over() {
                break;
            }
            let action = agent.action(&mut choice, &mut rng);
            log.push(action);
            choice.take_action(action);
        }
        let file = File::create(format!(
            "logs/{:?}.json",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Got time")
                .as_millis()
        ))
        .expect("Created log file");
        let mut writer = BufWriter::new(file);
        serde_json::to_writer_pretty(&mut writer, &log).expect("Wrote to file");
        writer.flush().expect("Flushed file");
    })
}

async fn agent_play() -> Result<(), Box<dyn Error>> {
    let (sender, receiver) = mpsc::channel(8);
    setup_keystream(sender.clone());
    let thread = spawn_game_thread(sender.clone());
    let mut ui_actor = UIActor::new(receiver);
    let ui_handle = spawn_local(async move { ui_actor.run().await });
    //The UI actor has some code to restore terminal settings on drop. This
    //join ensures it will be run. It already has a panic hook by default.
    ui_handle.await.expect("UI exited");
    //This is waiting for the thread to join in an async context. This isn't great,
    //but I want to let the thread write its log before the program exits.
    thread.join().expect("Joined game thread");
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
