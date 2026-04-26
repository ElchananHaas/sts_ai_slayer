use std::{fs::File, sync::Arc};

use crate::{BrokerEvent, game::choice::ChoiceState, ui::fight_ui::draw_game};
use crossterm::event::{Event as CrosstermEvent, KeyCode};
use fliptui::{
    WidgetRoot, Window,
    widgets::{BorderWidget, text_line},
};
use tokio::sync::mpsc::{self, Sender};

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

struct RootState<'a> {
    state: Option<&'a Arc<ChoiceState>>,
}

impl<'a> WidgetRoot for RootState<'a> {
    fn ui<T: fliptui::Element>(&mut self, root: &mut T) {
        if let Some(choice_state) = self.state {
            draw_game(root, &*choice_state);
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

    pub async fn run(&mut self) {
        while let Some(msg) = self.receiver.recv().await {
            match msg {
                UIEvent::NewState(choice_state) => self.choice_state = Some(choice_state),
                UIEvent::Crossterm(event) => match event {
                    CrosstermEvent::FocusGained => {}
                    CrosstermEvent::FocusLost => {}
                    CrosstermEvent::Key(key_event) => {
                        if key_event.code == KeyCode::Esc {
                            // If the broker can't receive the event, ignore it and shut down anyways.
                            let _ = self.sender.send(BrokerEvent::Exit).await;
                            return;
                        }
                        if key_event.code == KeyCode::Char(' ') {
                            if self.sender.send(BrokerEvent::AdvanceAI).await.is_err() {
                                return;
                            }
                        }
                        if key_event.code == KeyCode::Char('p') {
                            if self.sender.send(BrokerEvent::PauseAI).await.is_err() {
                                return;
                            }
                        }
                    }
                    CrosstermEvent::Mouse(_mouse_event) => {}
                    CrosstermEvent::Paste(_) => {}
                    CrosstermEvent::Resize(_, _) => {}
                },
            };

            self.window.draw(RootState {
                state: self.choice_state.as_ref(),
            });
        }
    }
}
