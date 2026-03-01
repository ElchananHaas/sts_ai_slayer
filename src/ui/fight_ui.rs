use std::array;
use std::fmt::Write;
 
use fliptui::{Widget, taffy};
use fliptui::widgets::{BorderWidget, TextRegion, text_line};

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

fn render_player(widget: &mut Widget, state: &ChoiceState) {
    let game = state.game();
    let mut widget = BorderWidget::builder(widget).build();
    text_line(&mut widget.title, "Player");
    let mut text_region = TextRegion::new(&mut widget.center);
    writeln!(&mut text_region, "{}", game.charachter().name());
    writeln!(
        &mut text_region,
        "{}/{} hp",
        game.player_hp(),
        game.player_max_hp()
    );
    writeln!(&mut text_region, "{} energy", game.fight().energy());
    writeln!(&mut text_region, "{} block", game.fight().player_block());
    if let Some(position) = game.act().position {
        writeln!(&mut text_region, "{} block", game.fight().player_block());
        writeln!(&mut text_region, "floor {}", position.y);
    }
}

fn render_card(widget: &mut Widget, state: &ChoiceState, card_idx: usize) {
    let game = state.game();
    let card = game.fight().hand().get(card_idx);
    let mut widget = BorderWidget::builder(widget).build();
    let mut text_region = TextRegion::new(&mut widget.center);
    if let Some(card) = card {
        let upgraded = if card.is_upgraded() { "+" } else { "" };
        if let Some(cost) = game.fight().evaluate_cost(card) {
            write!(&mut text_region, "{:?}{} [{}]", card.body, upgraded, cost);
        } else {
            write!(&mut text_region, "{:?}{}", card.body, upgraded);
        };
    }
}

fn render_cards(widget: &mut Widget, state: &ChoiceState) {
    widget.layout().push_grid_template_row_fr(1.0);
    for i in 0..Game::MAX_CARDS_IN_HAND {
        widget.layout().push_grid_template_column_fr(1.0);
        let mut child = widget.child();
        child.layout().grid_col(i).grid_row(0);
        render_card(&mut child, state, i);
    }
}

fn render_enemy(widget: &mut Widget, state: &ChoiceState, enemy_idx: usize) {
    let game = state.game();
    let enemy = &game.fight().enemies().enemies[enemy_idx];
    let Some(enemy) = enemy else {
        return;
    };
    let mut widget = BorderWidget::builder(widget).build();
    let mut text_region = TextRegion::new(&mut widget.center);
    writeln!(&mut text_region, "{}", enemy.name);
    writeln!(&mut text_region, "{}/{} hp", enemy.hp, enemy.max_hp);
    if enemy.block > 0 {
        writeln!(&mut text_region, "{} block", enemy.block);
    }
    if enemy.buffs.strength > 0 {
        writeln!(&mut text_region, "{} str", enemy.buffs.strength);
    }
    if enemy.buffs.ritual > 0 || enemy.buffs.ritual_skip_first > 0 {
        writeln!(
            &mut text_region,
            "{} ritual",
            enemy.buffs.ritual + enemy.buffs.ritual_skip_first
        );
    }
    if enemy.buffs.curl_up > 0 {
        writeln!(&mut text_region, "{} curl up", enemy.buffs.curl_up);
    }
    if enemy.debuffs.vulnerable > 0 {
        writeln!(&mut text_region, "{} vulnerable", enemy.debuffs.vulnerable);
    }
    if enemy.debuffs.weak > 0 {
        writeln!(&mut text_region, "{} weak", enemy.debuffs.weak);
    }
}

fn render_enemies(widget: &mut Widget, state: &ChoiceState) {
    //TODO - this should have a more clever layout.
    widget.layout().push_grid_template_row_fr(1.0);
    for i in 0..Game::MAX_ENEMIES {
        widget.layout().push_grid_template_column_fr(1.0);
        let mut child = widget.child();
        child.layout().grid_col(i).grid_row(0);
        render_enemy(&mut child, state, i);
    }
}

fn vertical_breakdown<'a>(widget: &mut Widget<'a>) -> [Widget<'a>; 3] {
    widget
        .layout()
        .push_grid_template_column_fr(1.0)
        .push_grid_template_row_px(8)
        .push_grid_template_row_fr(1.0)
        .push_grid_template_row_px(12);
    array::from_fn(|i| {
        widget.child().apply(|w| {
            w.layout().grid_col(0).grid_row(i);
        })
    })
}

fn render_rest_site(
    widget: &mut Widget,
    _choice_state: &ChoiceState,
    rest_site_actions: Vec<RestSiteAction>,
) {
    let [_top, mut middle, _bottom] = vertical_breakdown(widget);
    middle.layout().flex_direction(taffy::FlexDirection::Row)
                   .justify_content(taffy::AlignContent::Center)
                   .align_items(taffy::AlignItems::Center);
    const MAX_RESTSITE_ACTIONS: usize = 5;
    for i in 0..rest_site_actions.len() {
        let mut child = widget.child();
        child.layout().border_left_px(1).border_top_px(1).border_right_px(1).border_bottom_px(1);
        let mut text_region = TextRegion::new(&mut child);
        writeln!(&mut text_region, "{:?}", rest_site_actions[i]);
    }
}

fn render_battlefield(widget: &mut Widget, choice_state: &ChoiceState) {
    let [_top, mut middle, mut bottom] = vertical_breakdown(widget);
    middle.layout().push_grid_template_row_fr(1.0);
    let [mut player_box, mut fight_box] = array::from_fn(|i| {
        widget.child().apply(|w| {
            middle.layout().push_grid_template_column_fr(1.0);
            w.layout().grid_col(i).grid_row(0);
        })
    });
    render_player(&mut player_box, choice_state);
    render_cards(&mut bottom, choice_state);
    render_enemies(&mut fight_box, choice_state);
}

pub fn draw_ui(widget: &mut Widget, choice_state: &ChoiceState) {
    match choice_state.choice().clone() {
        crate::game::choice::Choice::PlayCardState(play_card_actions) => {
            render_battlefield(widget, choice_state)
        }
        crate::game::choice::Choice::ChooseEnemyState(choose_enemy_actions, _) => {
            render_battlefield(widget, choice_state)
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
            render_rest_site(widget, choice_state, rest_site_actions);
        }
    }
}
