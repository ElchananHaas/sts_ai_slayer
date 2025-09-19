use ratatui::widgets::{Block, Paragraph, Widget};
use ratatui::prelude::*;

use crate::game::choice::ChoiceState;

struct UIState<'a>{
    choice_state: &'a ChoiceState
}

fn render_player(state: &ChoiceState, area: Rect, buf: &mut Buffer) {
    let game = state.game();
    let text = Text::from(vec![
        format!("{}", game.charachter().name()).into(),
        format!("{}/{} hp", game.player_hp(), game.player_max_hp()).into(),
        format!("{} energy", game.fight().energy()).into(),
        format!("{} block", game.fight().player_block()).into(),
        format!("floor {}", game.floor()).into(),
    ]);
    Paragraph::new(text).block(Block::bordered()).render(area, buf);
}
impl<'a> Widget for UIState<'a> {
    fn render(self, area: Rect, buf: &mut Buffer){
        let layout = Layout::vertical([Constraint::Length(8), Constraint::Fill(1), Constraint::Length(12)]);
        let [top, middle, bottom] = layout.areas(buf.area);
        let [player_box, fight_box] = Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]).areas(middle);
        render_player(self.choice_state, player_box, buf);
    }
}