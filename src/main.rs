use std::{
    error::Error,
    fs::File,
    io::{BufWriter, Write},
    sync::Arc,
    thread::{self, JoinHandle},
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    agents::agent_helper::Agent,
    game::choice::ChoiceState,
    ui::ui_actor::{UIActor, UIEvent},
    util::GameLog,
};
use agents::agent_helper::SkipSingleChoiceAgent;
use agents::mcts_agent::MctsAgent;
use agents::random_agent::RandomAgent;
use crossterm::event::EventStream;
use futures::{StreamExt, channel::oneshot};
use game::{Character, Game};
use rng::Rng;
use tokio::{
    sync::mpsc::{self, Receiver, Sender},
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

async fn run_game(mut reciever: Receiver<usize>, sender: Sender<Arc<ChoiceState>>) {
    let game = Game::new(Character::IRONCLAD);
    let game_seed = game.get_seed();
    let mut choice = Arc::new(game.start());
    let mut log = GameLog::new(game_seed);
    loop {
        if sender.send(Arc::clone(&choice)).await.is_err() {
            // If the main thread isn't listening for messages anymore, return.
            // This can happen if the UI is shut down.
            break;
        }
        if choice.is_over() {
            break;
        }
        let Some(action) = reciever.recv().await else {
            break;
        };
        log.push(action);
        Arc::make_mut(&mut choice).take_action(action);
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
}

fn human_play() {
    todo!()
}

fn spawn_agent_thread(
    mut reciever: Receiver<(Arc<ChoiceState>, oneshot::Sender<usize>)>,
) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut rng = Rng::new();
        let mut agent = SkipSingleChoiceAgent {
            agent: MctsAgent {},
        };
        loop {
            let Some((state, sender)) = reciever.blocking_recv() else {
                return;
            };
            let action = agent.action(&*state, &mut rng);
            if sender.send(action).is_err() {
                return;
            }
        }
    })
}

async fn agent_play() -> Result<(), Box<dyn Error>> {
    let (game_action_sender, game_action_receiver) = mpsc::channel(1);
    let (game_state_sender, mut game_state_receiver) = mpsc::channel(1);
    let (ui_sender, ui_receiver) = mpsc::channel(1);
    let (agent_state_sender, agent_state_receiver) = mpsc::channel(1);
    spawn_agent_thread(agent_state_receiver);
    setup_keystream(ui_sender.clone());
    let mut ui_actor = UIActor::new(ui_receiver);
    let ui_handle = spawn_local(async move { ui_actor.run().await });
    let game_handle =
        spawn_local(async move { run_game(game_action_receiver, game_state_sender).await });
    loop {
        let Some(()) = async {
            let state = game_state_receiver.recv().await?;
            ui_sender
                .send(UIEvent::NewState(Arc::clone(&state)))
                .await
                .ok()?;
            let (send, recv) = oneshot::channel();
            agent_state_sender.send((state, send)).await.ok()?;
            let action = recv.await.ok()?;
            game_action_sender.send(action).await.ok()?;
            Some(())
        }
        .await
        else {
            break;
        };
    }
    //Drop all the queues to prevent deadlocks.
    drop(game_state_receiver);
    drop(game_action_sender);
    drop(ui_sender);
    drop(agent_state_sender);
    //The UI actor has some code to restore terminal settings on drop. This
    //join ensures it will be run. It already has a panic hook by default.
    ui_handle.await.expect("UI exited");
    //Let the game write its log before the program exits.
    game_handle.await.expect("Game exited");
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
