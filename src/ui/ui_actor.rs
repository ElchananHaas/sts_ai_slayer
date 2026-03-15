use std::fs::File;

use crate::{game::choice::ChoiceState, ui::fight_ui::draw_ui};
use crossterm::event::Event as CrosstermEvent;
use fliptui::{
    Window,
    widgets::{BorderWidget, text_line},
};
use tokio::sync::mpsc;

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
        let file = File::create("uilog.txt").unwrap();
        Self {
            receiver,
            choice_state: None,
            window: Window::builder().log_file(file).build(),
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

            self.window.draw(|mut frame| {
                if let Some(choice_state) = &self.choice_state {
                    draw_ui(&mut frame, choice_state);
                } else {
                    BorderWidget::builder(&mut frame, |center| {
                        text_line(center, "Waiting for game start")
                    })
                    .build();
                }
            });
        }
    }
}
