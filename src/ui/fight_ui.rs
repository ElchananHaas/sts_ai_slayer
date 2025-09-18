use ratatui::widgets::Widget;
use ratatui::prelude::*;

use crate::game::choice::ChoiceState;

struct UIState<'a>{
    choice_state: &'a ChoiceState
}


impl<'a> Widget for UIState<'a> {
    fn render(self, area: Rect, buf: &mut Buffer){
        let layout = Layout::vertical([Constraint::Length(8), Constraint::Fill(1), Constraint::Length(12)]);
        let [top, middle, bottom] = layout.areas(buf.area);
    }
}