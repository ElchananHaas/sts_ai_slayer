use std::{cell::Cell, fs::File, sync::Arc};

use crate::{
    ActionClient, BrokerEvent, GameAction,
    game::{
        Game,
        choice::{Choice, ChoiceState},
    },
    ui::fight_ui::draw_game,
};
use crossterm::event::{Event as CrosstermEvent, KeyCode, KeyEvent};
use fliptui::{
    WidgetRoot, Window,
    widgets::{BorderWidget, text_line},
};
use tokio::sync::mpsc::{self, Sender, error::SendError};

pub enum UIEvent {
    NewState(Arc<ChoiceState>),
    Crossterm(CrosstermEvent),
}

pub struct UIActor {
    receiver: mpsc::Receiver<UIEvent>,
    sender: Sender<BrokerEvent>,
    choice_state: Option<Arc<ChoiceState>>,
    window: Window,
}

pub(crate) struct UICtx {
    choice_state: Arc<ChoiceState>,
    action: Cell<Option<usize>>,
}

impl UICtx {
    pub fn game(&self) -> &Game {
        self.choice_state.game()
    }
    pub fn choice(&self) -> &Choice {
        self.choice_state.choice()
    }
    pub fn set_action(&self, action: usize) {
        self.action.set(Some(action))
    }
}
struct RootState {
    state: Option<UICtx>,
}

impl WidgetRoot for RootState {
    fn ui<T: fliptui::Element>(&mut self, root: &mut T) {
        if let Some(choice_state) = &mut self.state {
            draw_game(root, choice_state);
        } else {
            BorderWidget::builder(root, |center| text_line(center, "Waiting for game start"))
                .build();
        }
    }
}
impl UIActor {
    pub fn new(receiver: mpsc::Receiver<UIEvent>, sender: Sender<BrokerEvent>) -> Self {
        let file = File::create("uilog.txt").unwrap();
        Self {
            receiver,
            sender,
            choice_state: None,
            window: Window::builder().log_file(file).build(),
        }
    }

    fn build_root_state(&self) -> RootState {
        RootState {
            state: self.choice_state.as_ref().map(|state| UICtx {
                choice_state: Arc::clone(state),
                action: Cell::new(None),
            }),
        }
    }

    async fn handle_key_press(
        &mut self,
        key_event: KeyEvent,
    ) -> Result<bool, SendError<BrokerEvent>> {
        if key_event.code == KeyCode::Esc {
            // If the broker can't receive the event, ignore it and shut down anyways.
            let _ = self.sender.send(BrokerEvent::Exit).await;
            return Ok(false);
        } else if key_event.code == KeyCode::Char(' ') {
            self.sender.send(BrokerEvent::AdvanceAI).await?;
        } else if key_event.code == KeyCode::Char('p') {
            self.sender.send(BrokerEvent::PauseAI).await?;
        } else {
            let mut root_state = self.build_root_state();
            self.window.key_press(&mut root_state, key_event);
            if let Some(state) = root_state.state
                && let Some(action) = state.action.get()
            {
                self.sender
                    .send(BrokerEvent::ActionTaken(GameAction {
                        action,
                        state_counter: *state.game().state_counter(),
                        client: ActionClient::Human,
                    }))
                    .await?;
            };
        }
        Ok(true)
    }

    async fn handle_msg(&mut self, msg: UIEvent) -> Result<bool, SendError<BrokerEvent>> {
        match msg {
            UIEvent::NewState(choice_state) => {
                self.choice_state = Some(choice_state);
                self.window.draw(&mut self.build_root_state());
            }
            UIEvent::Crossterm(event) => match event {
                CrosstermEvent::FocusGained => {}
                CrosstermEvent::FocusLost => {}
                CrosstermEvent::Key(key_event) => {
                    if key_event.is_press() {
                        return self.handle_key_press(key_event).await;
                    }
                }
                CrosstermEvent::Mouse(_mouse_event) => {}
                CrosstermEvent::Paste(_) => {}
                CrosstermEvent::Resize(_, _) => {}
            },
        };
        Ok(true)
    }

    pub async fn run(&mut self) {
        while let Some(msg) = self.receiver.recv().await {
            if !self.handle_msg(msg).await.unwrap_or(false) {
                return;
            }
        }
    }
}
