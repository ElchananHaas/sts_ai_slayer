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
use futures::StreamExt;
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

#[derive(Clone, Copy)]
enum ActionClient {
    Human,
    Computer,
}

#[derive(Clone, Copy)]
struct GameAction {
    action: usize,
    state_counter: u32,
    client: ActionClient,
}

enum BrokerEvent {
    NewState(Arc<ChoiceState>),
    ActionTaken(GameAction),
    Exit,
}

impl From<Arc<ChoiceState>> for BrokerEvent {
    fn from(value: Arc<ChoiceState>) -> Self {
        Self::NewState(value)
    }
}

impl From<GameAction> for BrokerEvent {
    fn from(value: GameAction) -> Self {
        Self::ActionTaken(value)
    }
}

async fn run_game<T: From<Arc<ChoiceState>>>(
    mut reciever: Receiver<GameAction>,
    sender: Sender<T>,
) {
    let game = Game::new(Character::IRONCLAD);
    let game_seed = game.get_seed();
    let mut choice = Arc::new(game.start());
    let mut log = GameLog::new(game_seed);
    loop {
        if sender.send(Arc::clone(&choice).into()).await.is_err() {
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
        // If the game AI is computing an action and the user takes an action
        // while the AI running, it could end up sending a stale state.
        // This check skips messages sent with an out-of-date state counter.
        // The game still sends the current state whenever it gets
        // an action to help the clients recover.
        if action.state_counter != *choice.game().state_counter() {
            continue;
        }
        log.push(action.action);
        Arc::make_mut(&mut choice).take_action(action.action);
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

fn spawn_agent_thread<T: From<GameAction> + Send + 'static>(
    mut reciever: Receiver<Arc<ChoiceState>>,
    sender: Sender<T>,
) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut rng = Rng::new();
        let mut agent = SkipSingleChoiceAgent {
            agent: MctsAgent {},
        };
        loop {
            let Some(state) = reciever.blocking_recv() else {
                return;
            };
            let action = agent.action(&*state, &mut rng);
            if sender
                .blocking_send(
                    GameAction {
                        action,
                        state_counter: *state.game().state_counter(),
                        client: ActionClient::Computer,
                    }
                    .into(),
                )
                .is_err()
            {
                return;
            }
        }
    })
}

async fn broker_loop(
    mut broker_receiver: Receiver<BrokerEvent>,
    agent_state_sender: Sender<Arc<ChoiceState>>,
    ui_sender: Sender<UIEvent>,
    game_action_sender: Sender<GameAction>,
) -> Option<()> {
    loop {
        let event = broker_receiver.recv().await?;
        match event {
            BrokerEvent::NewState(choice_state) => {
                if !choice_state.is_over() {
                    agent_state_sender
                        .send(Arc::clone(&choice_state))
                        .await
                        .ok()?;
                }
                ui_sender.send(UIEvent::NewState(choice_state)).await.ok()?;
            }
            BrokerEvent::ActionTaken(game_action) => {
                game_action_sender.send(game_action).await.ok()?;
            }
            BrokerEvent::Exit => return Some(()),
        }
    }
}
async fn agent_play() -> Result<(), Box<dyn Error>> {
    let (broker_sender, broker_receiver) = mpsc::channel::<BrokerEvent>(2);
    let (game_action_sender, game_action_receiver) = mpsc::channel(2);
    let (ui_sender, ui_receiver) = mpsc::channel(2);
    let (agent_state_sender, agent_state_receiver) = mpsc::channel(2);
    spawn_agent_thread(agent_state_receiver, broker_sender.clone());
    setup_keystream(ui_sender.clone());
    let ui_handle = spawn_local({
        let broker_sender = broker_sender.clone();
        async move { UIActor::new(ui_receiver, broker_sender).run().await }
    });
    let game_handle =
        spawn_local(async move { run_game(game_action_receiver, broker_sender).await });
    broker_loop(
        broker_receiver,
        agent_state_sender,
        ui_sender,
        game_action_sender,
    )
    .await;
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
