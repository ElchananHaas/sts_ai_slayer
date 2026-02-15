use std::cmp::min;

use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph, Widget};

use crate::game::Game;
use crate::game::choice::{ChoiceState, RestSiteAction};

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
    let mut text: Vec<Line> = vec![
        format!("{}", game.charachter().name()).into(),
        format!("{}/{} hp", game.player_hp(), game.player_max_hp()).into(),
        format!("{} energy", game.fight().energy()).into(),
        format!("{} block", game.fight().player_block()).into(),
    ];
    if let Some(position) = game.act().position {
        text.push(format!("floor {}", position.y).into())
    }
    Paragraph::new(Text::from(text))
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
    let Some(enemy) = enemy else {
        return;
    };
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
        text.push(
            format!(
                "{} ritual",
                enemy.buffs.ritual + enemy.buffs.ritual_skip_first
            )
            .into(),
        );
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

fn render_battlefield(state: &UIState, area: Rect, buf: &mut Buffer) {
    let [top, middle, bottom] = main_breakdown(area);
    let [player_box, fight_box] =
        Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(middle);
    render_player(state.choice_state, player_box, buf);
    render_cards(state.choice_state, bottom, buf);
    render_enemies(state.choice_state, fight_box, buf);
}

fn main_breakdown(area: Rect) -> [Rect; 3] {
    let layout = Layout::vertical([
        Constraint::Length(8),
        Constraint::Fill(1),
        Constraint::Length(12),
    ]);
    layout.areas(area)
}

fn render_rest_site(
    _state: &UIState,
    area: Rect,
    buf: &mut Buffer,
    rest_site_actions: Vec<RestSiteAction>,
) {
    let [_top, middle, _bottom] = main_breakdown(area);
    const MAX_RESTSITE_ACTIONS: usize = 5;
    let areas: [Rect; MAX_RESTSITE_ACTIONS] =
        Layout::horizontal([Constraint::Length(10); MAX_RESTSITE_ACTIONS])
            .flex(layout::Flex::Center)
            .areas::<{ MAX_RESTSITE_ACTIONS }>(middle);
    for i in 0..(min(MAX_RESTSITE_ACTIONS, rest_site_actions.len())) {
        let area = areas[i];
        let area = Layout::vertical([Constraint::Length(10); 1])
            .flex(layout::Flex::Center)
            .areas::<1>(area)[0];
        Paragraph::new(vec![format!("{:?}", rest_site_actions[i]).into()])
            .block(Block::bordered())
            .render(area, buf);
    }
}

impl<'a> Widget for UIState<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.choice_state.choice().clone() {
            crate::game::choice::Choice::PlayCardState(play_card_actions) => {
                render_battlefield(&self, area, buf)
            }
            crate::game::choice::Choice::ChooseEnemyState(choose_enemy_actions, _) => {
                render_battlefield(&self, area, buf)
            }
            crate::game::choice::Choice::Win => {}
            crate::game::choice::Choice::Loss => {}
            crate::game::choice::Choice::MapState(map_state_actions) => {}
            crate::game::choice::Choice::SelectCardState(
                play_card_context,
                select_card_effect,
                select_card_actions,
                selection_pile,
            ) => {}
            crate::game::choice::Choice::Event(event, event_actions) => {}
            crate::game::choice::Choice::RemoveCardState(remove_card_actions) => {}
            crate::game::choice::Choice::TransformCardState(transform_card_actions) => {}
            crate::game::choice::Choice::UpgradeCardState(upgrade_card_actions) => {}
            crate::game::choice::Choice::RestSite(rest_site_actions) => {
                render_rest_site(&self, area, buf, rest_site_actions);
            }
        }
    }
}
