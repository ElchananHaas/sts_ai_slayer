use crate::{game::choice::ChoiceState, ui::fight_ui::UIState};
use crossterm::event::Event as CrosstermEvent;
use fliptui::Window;
use tokio::sync::mpsc::{self, Receiver};

pub enum UIEvent {
    NewState(ChoiceState),
    KeyPress(CrosstermEvent),
}

pub struct UIActor {
    receiver: mpsc::Receiver<UIEvent>,
    choice_state: Option<ChoiceState>,
    window: Window,
}

impl UIActor {
    pub fn new(receiver: mpsc::Receiver<UIEvent>) -> Self {
        Self {
            receiver,
            choice_state: None,
            window: Window::builder().build(),
        }
    }

    pub async fn run(&mut self) {
        while let Some(msg) = self.receiver.recv().await {
            match msg {
                UIEvent::NewState(choice_state) => self.choice_state = Some(choice_state),
                UIEvent::KeyPress(event) => match event {
                    CrosstermEvent::FocusGained => {}
                    CrosstermEvent::FocusLost => {}
                    CrosstermEvent::Key(_key_event) => {
                        break;
                    }
                    CrosstermEvent::Mouse(_mouse_event) => {}
                    CrosstermEvent::Paste(_) => {}
                    CrosstermEvent::Resize(_, _) => {}
                },
            };

            self.window
                .draw(|frame| {
                    if let Some(choice_state) = &self.choice_state {
                        UIState::new(&choice_state).render(frame.area(), frame.buffer_mut())
                    } else {
                        Paragraph::new("Waiting for game start")
                            .block(Block::bordered())
                            .render(frame.area(), frame.buffer_mut());
                    }
                })
                .expect("Frame rendered successfully.");
        }
        ratatui::restore();
    }
}
