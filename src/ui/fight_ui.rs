use std::fmt::Write;

use crossterm::event::{KeyCode, KeyEvent};
use fliptui::taffy::{FlexDirection, FlexWrap};
use fliptui::widgets::{BorderWidget, TextRegion, text_line};
use fliptui::{Element, Node, taffy};

use crate::card::Card;
use crate::game::Game;
use crate::game::choice::{
    ChooseEnemyAction, PlayCardAction, RestSiteAction, SelectDeckCardAction,
};
use crate::map::{self, NUM_FLOORS, ROW_WIDTH};
use crate::ui::ui_actor::UICtx;

//This forwards all input to the standard writeln/write macro, but ignores the result. This
//is useful for writing ot the TextRegion because those write calls will never error.
macro_rules! writeln {
    ($($arg:tt)*) => {
        {
            let _ = std::writeln!($($arg)*);
        }
    };
}
macro_rules! write {
    ($($arg:tt)*) => {
        let _ = std::write!($($arg)*);
    };
}

//The leftmost key on a keyboard is 1. When we are indexing from 0, offset them by 1 to make them line up nicely
fn rotate_key(x: usize) -> u32 {
    ((x + 1) % 10) as u32
}

//This checks if the key corresponding to rotate_key(x) was pressed
fn matches_rotated_key(event: KeyEvent, idx: usize) -> bool {
    event.code == KeyCode::Char(char::from_digit(rotate_key(idx), 10).expect("Number is in-bounds"))
}

fn simple_boxed_text<T: Element>(widget: &mut T, f: impl FnOnce(&mut TextRegion<T>)) {
    BorderWidget::builder(widget, |center| {
        let mut text_region = TextRegion::new(center);
        (f)(&mut text_region)
    })
    .build();
}
fn render_player(widget: &mut impl Element, state: &UICtx) {
    let game = state.game();
    BorderWidget::builder(widget, |center| {
        let mut text_region = TextRegion::new(center);
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
    })
    .title("Player")
    .build();
}

fn render_card(
    widget: &mut impl Element,
    state: &UICtx,
    card: &Card,
    card_idx: usize,
    action_idx: Option<usize>,
) {
    simple_boxed_text(widget, |text_region| {
        let upgraded = if card.is_upgraded() { "+" } else { "" };
        if let Some(cost) = state.game().fight().evaluate_cost(card) {
            writeln!(text_region, "{:?}{} [{}]", card.body, upgraded, cost);
        } else {
            writeln!(text_region, "{:?}{}", card.body, upgraded);
        };
        if action_idx.is_some() {
            writeln!(text_region, "Key {:?}", rotate_key(card_idx));
        } else {
            writeln!(text_region, "");
        }
    });
    if let Some(action_idx) = action_idx {
        widget.key_press(|event| {
            if matches_rotated_key(event, card_idx) {
                state.set_action(action_idx);
            }
        });
    }
}

fn render_hand_card(
    widget: &mut impl Element,
    state: &UICtx,
    card_idx: usize,
    action_idx: Option<usize>,
) {
    let game = state.game();
    let card = game.fight().hand().get(card_idx).cloned();
    card.map(|card| render_card(widget, state, &card, card_idx, action_idx));
}

fn render_cards(widget: &mut impl Element, state: &UICtx, play_card_actions: Vec<PlayCardAction>) {
    widget.layout().push_grid_template_row_fr(1.0);
    let mut hand_to_action = vec![None; state.game().fight().hand().len()];
    for (i, action) in play_card_actions.iter().enumerate() {
        if let PlayCardAction::PlayCard(idx) = action {
            hand_to_action[*idx as usize] = Some(i);
        }
    }
    for i in 0..Game::MAX_CARDS_IN_HAND {
        widget.layout().push_grid_template_column_fr(1.0);
        widget.child(|child| {
            child.layout().grid_col(i).grid_row(0);
            render_hand_card(child, state, i, hand_to_action.get(i).copied().flatten());
        });
    }
}

fn render_enemy(
    widget: &mut impl Element,
    state: &UICtx,
    enemy_idx: usize,
    action_idx: Option<usize>,
) {
    let game = state.game();
    let enemy = &game.fight().enemies().enemies[enemy_idx];
    let Some(enemy) = enemy else {
        return;
    };
    simple_boxed_text(widget, |text_region| {
        writeln!(text_region, "{:?}", enemy.name);
        writeln!(text_region, "{}/{} hp", enemy.hp, enemy.max_hp);
        if enemy.block > 0 {
            writeln!(text_region, "{} block", enemy.block);
        }
        if enemy.buffs.strength > 0 {
            writeln!(text_region, "{} str", enemy.buffs.strength);
        }
        if enemy.buffs.ritual > 0 || enemy.buffs.ritual_skip_first > 0 {
            writeln!(
                text_region,
                "{} ritual",
                enemy.buffs.ritual + enemy.buffs.ritual_skip_first
            );
        }
        if enemy.buffs.curl_up > 0 {
            writeln!(text_region, "{} curl up", enemy.buffs.curl_up);
        }
        if enemy.debuffs.vulnerable > 0 {
            writeln!(text_region, "{} vulnerable", enemy.debuffs.vulnerable);
        }
        if enemy.debuffs.weak > 0 {
            writeln!(text_region, "{} weak", enemy.debuffs.weak);
        }
        if action_idx.is_some() {
            writeln!(text_region, "Key {:?}", rotate_key(enemy_idx));
        }
    });
    if let Some(action_idx) = action_idx {
        widget.key_press(|event| {
            if matches_rotated_key(event, enemy_idx) {
                state.set_action(action_idx);
            }
        });
    }
}

fn render_enemies(
    widget: &mut impl Element,
    state: &UICtx,
    choose_enemy_actions: Vec<ChooseEnemyAction>,
) {
    let mut enemy_to_action = vec![None; Game::MAX_ENEMIES];
    for (i, action) in choose_enemy_actions.iter().enumerate() {
        enemy_to_action[action.enemy as usize] = Some(i);
    }
    //TODO - this should have a more clever layout.
    widget.layout().push_grid_template_row_fr(1.0);
    for i in 0..Game::MAX_ENEMIES {
        widget.layout().push_grid_template_column_fr(1.0);
        widget.child(|child| {
            child.layout().grid_col(i).grid_row(0);
            render_enemy(child, state, i, enemy_to_action[i]);
        });
    }
}

fn style_vertical_breakdown(parent: &mut impl Node, children: &mut [impl Node; 3]) {
    parent.layout().height_percent(1.0).width_percent(1.0);
    parent
        .layout()
        .push_grid_template_column_fr(1.0)
        .push_grid_template_row_px(6)
        .push_grid_template_row_fr(1.0)
        .push_grid_template_row_px(8);
    for i in 0..children.len() {
        children[i].layout().grid_col(0).grid_row(i);
    }
}
fn render_rest_site(
    widget: &mut impl Element,
    ui_ctx: &UICtx,
    rest_site_actions: Vec<RestSiteAction>,
) {
    let top = widget.child(|_child| {});
    let middle = widget.child(|child| {
        child
            .layout()
            .flex_direction(taffy::FlexDirection::Row)
            .justify_content(taffy::AlignContent::Center)
            .align_items(taffy::AlignItems::Center);
        for i in 0..rest_site_actions.len() {
            child.child(|child| {
                BorderWidget::builder(child, |center| {
                    render_rest_site_action(center, ui_ctx, rest_site_actions[i], i)
                })
                .build();
            });
        }
    });
    let bottom = widget.child(|_child| {});
    style_vertical_breakdown(widget, &mut [top, middle, bottom]);
}

fn render_rest_site_action(widget: &mut impl Element, ui_ctx: &UICtx, action: RestSiteAction, idx: usize) {
    widget
        .layout()
        .border_left_px(1)
        .border_top_px(1)
        .border_right_px(1)
        .border_bottom_px(1);
    let mut text_region = TextRegion::new(widget);
    writeln!(&mut text_region, "{:?}", action);
    writeln!(&mut text_region, "Key {:?}", rotate_key(idx));
    widget.key_press(|event| {
        if matches_rotated_key(event, idx) {
            ui_ctx.set_action(idx);
        }
    });
}
fn render_game_over(widget: &mut impl Element, ui_ctx: &UICtx) {
    let top = widget.child(|_child| {});
    let middle = widget.child(|child| {
        child
            .layout()
            .justify_content(taffy::AlignContent::Center)
            .align_items(taffy::AlignItems::Center);
        child.child(|child| render_game_over_box(child, ui_ctx));
    });
    let bottom = widget.child(|_child| {});
    style_vertical_breakdown(widget, &mut [top, middle, bottom]);
}

fn render_game_over_box(widget: &mut impl Element, ui_ctx: &UICtx) {
    BorderWidget::builder(widget, |center| {
        let mut text_region = TextRegion::new(center);
        match &ui_ctx.choice() {
            crate::game::choice::Choice::Win => {
                writeln!(&mut text_region, "Victory!");
            }
            crate::game::choice::Choice::Loss => {
                writeln!(&mut text_region, "Loss");
            }
            _ => panic!("render_game_over_box called in a non game over state"),
        }
    })
    .build();
}
fn render_battlefield(
    widget: &mut impl Element,
    ui_ctx: &UICtx,
    play_card_actions: Vec<PlayCardAction>,
    choose_enemy_actions: Vec<ChooseEnemyAction>,
) {
    let top = widget.child(|_child| {});
    let middle = widget.child(|child| {
        child.layout().push_grid_template_row_fr(1.0);
        let player_box = child.child(|child| {
            render_player(child, ui_ctx);
        });
        let fight_box = child.child(|child| {
            render_enemies(child, ui_ctx, choose_enemy_actions);
        });
        for elem in (&mut [player_box, fight_box]).iter_mut().enumerate() {
            child.layout().push_grid_template_column_fr(1.0);
            elem.1.layout().grid_col(elem.0).grid_row(0);
        }
    });
    let bottom = widget.child(|child| {
        render_cards(child, ui_ctx, play_card_actions);
    });
    style_vertical_breakdown(widget, &mut [top, middle, bottom]);
}

pub fn render_card_view(
    widget: &mut impl Element,
    ui_ctx: &UICtx,
    actions: Vec<SelectDeckCardAction>,
) {
    let top = widget.child(|_child| {});
    let middle = widget.child(|child| {
        render_card_view_inner(child, ui_ctx, actions);
    });
    let bottom = widget.child(|_child| {});
    style_vertical_breakdown(widget, &mut [top, middle, bottom]);
}

pub fn render_card_view_inner(
    widget: &mut impl Element,
    ui_ctx: &UICtx,
    actions: Vec<SelectDeckCardAction>,
) {
    widget
        .layout()
        .flex_direction(FlexDirection::Row)
        .flex_wrap(FlexWrap::Wrap);
    for (action_idx, action) in actions.iter().enumerate() {
        ui_ctx
            .game()
            .base_deck()
            .get(action.0)
            .cloned()
            .map(|card| {
                widget.child(|child| render_card(child, ui_ctx, &card, action.0, Some(action_idx)));
            });
    }
}

pub fn render_map_state(widget: &mut impl Element, ui_ctx: &UICtx) {
    let game = ui_ctx.game();
    let position = game.act().position;
    for _ in 0..ROW_WIDTH {
        widget.layout().push_grid_template_row_fr(1.0);
    }
    for _ in 0..NUM_FLOORS {
        widget.layout().push_grid_template_column_fr(1.0);
    }
    for i in 0..map::NUM_FLOORS {
        for j in 0..map::ROW_WIDTH {
            widget.child(|child| {
                child
                    .layout()
                    .grid_row(j)
                    .grid_col(i)
                    .align_self(taffy::AlignItems::Center)
                    .justify_self(taffy::AlignItems::Center);
                let room = game.map().rooms[i][j];
                let text = match room.room_type {
                    map::RoomType::QuestionMark => "?",
                    map::RoomType::Shop => "Shop",
                    map::RoomType::Treasure => "Chest",
                    map::RoomType::Rest => "Rest",
                    map::RoomType::Monster => "Fight",
                    map::RoomType::Elite => "Elite",
                    map::RoomType::Unassigned => "",
                };
                if position.is_some_and(|pos| pos.x as usize == j && pos.y as usize == i) {
                    simple_boxed_text(child, |writer| writeln!(writer, "{text}"));
                } else {
                    text_line(child, text);
                }
            });
        }
    }
}

pub fn draw_game(widget: &mut impl Element, ui_ctx: &UICtx) {
    match ui_ctx.choice().clone() {
        crate::game::choice::Choice::PlayCardState(play_card_actions) => {
            widget.child(|elem| {
                render_battlefield(elem, ui_ctx, play_card_actions, vec![]);
            });
        }
        crate::game::choice::Choice::ChooseEnemyState(choose_enemy_actions, _) => {
            widget.child(|elem| {
                render_battlefield(elem, ui_ctx, vec![], choose_enemy_actions);
            });
        }
        crate::game::choice::Choice::Win => {
            widget.child(|elem| {
                render_game_over(elem, ui_ctx);
            });
        }
        crate::game::choice::Choice::Loss => {
            widget.child(|elem| {
                render_game_over(elem, ui_ctx);
            });
        }
        crate::game::choice::Choice::MapState(map_state_actions) => {
            widget.child(|elem| {
                render_map_state(elem, ui_ctx);
            });
        }
        crate::game::choice::Choice::SelectCardState(
            play_card_context,
            select_card_effect,
            select_card_actions,
            selection_pile,
        ) => {}
        crate::game::choice::Choice::Event(event, event_actions) => {}
        crate::game::choice::Choice::SelectDeckCardState(reason, actions) => {
            widget.child(|elem| {
                render_card_view(elem, ui_ctx, actions);
            });
        }
        crate::game::choice::Choice::RestSite(actions) => {
            widget.child(|elem| {
                render_rest_site(elem, ui_ctx, actions);
            });
        }
    }
}
