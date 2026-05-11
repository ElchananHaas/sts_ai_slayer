use std::fmt::Write;

use crossterm::event::{KeyCode, KeyEvent};
use fliptui::taffy::{FlexDirection, FlexWrap};
use fliptui::widgets::{BorderWidget, text_line};
use fliptui::{Element, Node, taffy};

use crate::card::Card;
use crate::game::Game;
use crate::game::choice::{
    ChooseEnemyAction, MapStateAction, PlayCardAction, RestSiteAction, SelectDeckCardAction,
    SelectDeckCardReason,
};
use crate::map::{self, NUM_FLOORS, ROW_WIDTH};
use crate::ui::ui_actor::UICtx;

//This forwards all input to the standard writeln/write macro, but ignores the result. This
//is useful for writing to the TextRegion because those write calls will never error.
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

fn render_player(widget: &mut impl Element, state: &UICtx) {
    let game = state.game();
    BorderWidget::builder(widget, |center| {
        writeln!(center.cursor(), "{}", game.charachter().name());
        writeln!(
            center.cursor(),
            "{}/{} hp",
            game.player_hp(),
            game.player_max_hp()
        );
        writeln!(center.cursor(), "{} energy", game.fight().energy());
        writeln!(center.cursor(), "{} block", game.fight().player_block());
        if let Some(position) = game.act().position {
            writeln!(center.cursor(), "{} block", game.fight().player_block());
            writeln!(center.cursor(), "floor {}", position.y);
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
    BorderWidget::builder(widget, |center| {
        let upgraded = if card.is_upgraded() { "+" } else { "" };
        if let Some(cost) = state.game().fight().evaluate_cost(card) {
            writeln!(center.cursor(), "{:?}{} [{}]", card.body, upgraded, cost);
        } else {
            writeln!(center.cursor(), "{:?}{}", card.body, upgraded);
        };
        if action_idx.is_some() {
            writeln!(center.cursor(), "Key {:?}", rotate_key(card_idx));
        } else {
            writeln!(center.cursor(), "");
        }
    })
    .build();
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
    let card = game.fight().hand().get(card_idx);
    card.map(|card| render_card(widget, state, card, card_idx, action_idx));
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
    BorderWidget::builder(widget, |center| {
        writeln!(center.cursor(), "{:?}", enemy.name);
        writeln!(center.cursor(), "{}/{} hp", enemy.hp, enemy.max_hp);
        if enemy.block > 0 {
            writeln!(center.cursor(), "{} block", enemy.block);
        }
        if enemy.buffs.strength > 0 {
            writeln!(center.cursor(), "{} str", enemy.buffs.strength);
        }
        if enemy.buffs.ritual > 0 || enemy.buffs.ritual_skip_first > 0 {
            writeln!(
                center.cursor(),
                "{} ritual",
                enemy.buffs.ritual + enemy.buffs.ritual_skip_first
            );
        }
        if enemy.buffs.curl_up > 0 {
            writeln!(center.cursor(), "{} curl up", enemy.buffs.curl_up);
        }
        if enemy.debuffs.vulnerable > 0 {
            writeln!(center.cursor(), "{} vulnerable", enemy.debuffs.vulnerable);
        }
        if enemy.debuffs.weak > 0 {
            writeln!(center.cursor(), "{} weak", enemy.debuffs.weak);
        }
        if action_idx.is_some() {
            writeln!(center.cursor(), "Key {:?}", rotate_key(enemy_idx));
        }
    })
    .build();
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

fn render_rest_site_action(
    widget: &mut impl Element,
    ui_ctx: &UICtx,
    action: RestSiteAction,
    idx: usize,
) {
    widget
        .layout()
        .border_left_px(1)
        .border_top_px(1)
        .border_right_px(1)
        .border_bottom_px(1);

    writeln!(widget.cursor(), "{:?}", action);
    writeln!(widget.cursor(), "Key {:?}", rotate_key(idx));
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
        match &ui_ctx.choice() {
            crate::game::choice::Choice::Win => {
                writeln!(center.cursor(), "Victory!");
            }
            crate::game::choice::Choice::Loss => {
                writeln!(center.cursor(), "Loss");
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
    reason: SelectDeckCardReason,
    actions: Vec<SelectDeckCardAction>,
) {
    let top = widget.child(|child| {
        writeln!(child.cursor(), "Select a card to {:?}", reason);
    });
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
    //TODO - Add pagination.
    for (action_idx, action) in actions.iter().enumerate() {
        ui_ctx.game().base_deck().get(action.0).map(|card| {
            widget.child(|child| render_card(child, ui_ctx, card, action.0, Some(action_idx)));
        });
    }
}

pub fn render_map_state(
    widget: &mut impl Element,
    ui_ctx: &UICtx,
    map_state_actions: Vec<MapStateAction>,
) {
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
                    BorderWidget::builder(child, |text_elem|
                        writeln!(text_elem.cursor(), "{text}")
                    ).build();
                } else {
                    text_line(child, text);
                }
                //This ensures the layout is correct even when the player doesn't have a map position.
                child.layout().min_width_px(7).min_height_px(3);
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
                render_map_state(elem, ui_ctx, map_state_actions);
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
                render_card_view(elem, ui_ctx, reason, actions);
            });
        }
        crate::game::choice::Choice::RestSite(actions) => {
            widget.child(|elem| {
                render_rest_site(elem, ui_ctx, actions);
            });
        }
    }
}
