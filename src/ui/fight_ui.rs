use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph, Widget};

use crate::game::Game;
use crate::game::choice::ChoiceState;

pub struct UIState<'a> {
    choice_state: &'a ChoiceState,
}

impl<'a> UIState<'a> {
    pub fn new(choice_state: &'a ChoiceState) -> Self {
        Self { choice_state }
    }
}

fn render_player(state: &ChoiceState, area: Rect, buf: &mut Buffer) {
    let game = state.game();
    let text = Text::from(vec![
        format!("{}", game.charachter().name()).into(),
        format!("{}/{} hp", game.player_hp(), game.player_max_hp()).into(),
        format!("{} energy", game.fight().energy()).into(),
        format!("{} block", game.fight().player_block()).into(),
        format!("floor {}", game.act().map_y).into(),
    ]);
    Paragraph::new(text)
        .block(Block::bordered())
        .render(area, buf);
}

fn render_card(state: &ChoiceState, card_idx: usize, area: Rect, buf: &mut Buffer) {
    let game = state.game();
    let card = game.fight().hand().get(card_idx);
    if let Some(card) = card {
        let upgraded = if card.is_upgraded() { "+" } else { "" };
        let card_contents = if let Some(cost) = game.fight().evaluate_cost(card) {
            format!("{:?}{} [{}]", card.body, upgraded, cost)
        } else {
            format!("{:?}{} [x]", card.body, upgraded)
        };
        let text = Text::from(card_contents);
        Paragraph::new(text)
            .block(Block::bordered())
            .render(area, buf);
    }
}

fn render_cards(state: &ChoiceState, area: Rect, buf: &mut Buffer) {
    let areas = Layout::horizontal([Constraint::Fill(1); Game::MAX_CARDS_IN_HAND])
        .areas::<{ Game::MAX_CARDS_IN_HAND }>(area);
    for i in 0..Game::MAX_CARDS_IN_HAND {
        render_card(state, i, areas[i], buf);
    }
}

fn render_enemy(state: &ChoiceState, enemy_idx: usize, area: Rect, buf: &mut Buffer) {
    let game = state.game();
    let enemy = &game.fight().enemies().enemies[enemy_idx];
    let Some(enemy) = enemy else {return;};
    let mut text: Vec<Line> = Vec::new();
    text.push(format!("{}", enemy.name).into());
    text.push(format!("{}/{} hp", enemy.hp, enemy.max_hp).into());
    if enemy.block > 0 {
        text.push(format!("{} block", enemy.block).into());
    }
    if enemy.buffs.strength > 0 {
        text.push(format!("{} str", enemy.buffs.strength).into());
    }
    if enemy.buffs.ritual > 0 || enemy.buffs.ritual_skip_first > 0 {
         text.push(format!("{} ritual", enemy.buffs.ritual + enemy.buffs.ritual_skip_first).into());
    }
    if enemy.buffs.curl_up > 0 {
        text.push(format!("{} curl up", enemy.buffs.curl_up).into());
    }
    if enemy.debuffs.vulnerable > 0 {
        text.push(format!("{} vulnerable", enemy.debuffs.vulnerable).into());
    }
    if enemy.debuffs.weak > 0 {
        text.push(format!("{} weak", enemy.debuffs.weak).into());
    }
    Paragraph::new(text)
        .block(Block::bordered())
        .render(area, buf);

}


fn render_enemies(state: &ChoiceState, area: Rect, buf: &mut Buffer) {
    //TODO - this should have a more clever layout.
    let areas = Layout::horizontal([Constraint::Fill(1); Game::MAX_ENEMIES])
        .areas::<{ Game::MAX_ENEMIES }>(area);
    for i in 0..Game::MAX_ENEMIES {
        render_enemy(state, i, areas[i], buf);
    }
}

impl<'a> Widget for UIState<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::vertical([
            Constraint::Length(8),
            Constraint::Fill(1),
            Constraint::Length(12),
        ]);
        let [top, middle, bottom] = layout.areas(buf.area);
        let [player_box, fight_box] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(middle);
        render_player(self.choice_state, player_box, buf);
        render_cards(self.choice_state, bottom, buf);
        render_enemies(self.choice_state, fight_box, buf);
    }
}
