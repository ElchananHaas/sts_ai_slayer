use std::array;
use std::fmt::Write;

use fliptui::taffy::{FlexDirection, FlexWrap};
use fliptui::widgets::{BorderWidget, TextRegion, text_line};
use fliptui::{Element, Node, WidgetRoot, taffy};

use crate::card::Card;
use crate::game::Game;
use crate::game::choice::{ChoiceState, RestSiteAction, SelectDeckCardAction};
use crate::map::{self, NUM_FLOORS, ROW_WIDTH};

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

fn simple_boxed_text<T: Element>(widget: &mut T, f: impl FnOnce(&mut TextRegion<T>)) {
    BorderWidget::builder(widget, |center| {
        let mut text_region = TextRegion::new(center);
        (f)(&mut text_region)
    })
    .build();
}
fn render_player(widget: &mut impl Element, state: &ChoiceState) {
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

fn render_card(widget: &mut impl Element, state: &ChoiceState, card: &Card) {
    simple_boxed_text(widget, |text_region| {
        let upgraded = if card.is_upgraded() { "+" } else { "" };
        if let Some(cost) = state.game().fight().evaluate_cost(card) {
            write!(text_region, "{:?}{} [{}]", card.body, upgraded, cost);
        } else {
            write!(text_region, "{:?}{}", card.body, upgraded);
        };
    });
}

fn render_hand_card(widget: &mut impl Element, state: &ChoiceState, card_idx: usize) {
    let game = state.game();
    let card = game.fight().hand().get(card_idx);
    card.map(|card| render_card(widget, state, card));
}

fn render_cards(widget: &mut impl Element, state: &ChoiceState) {
    widget.layout().push_grid_template_row_fr(1.0);
    for i in 0..Game::MAX_CARDS_IN_HAND {
        widget.layout().push_grid_template_column_fr(1.0);
        widget.child(|child| {
            child.layout().grid_col(i).grid_row(0);
            render_hand_card(child, state, i);
        });
    }
}

fn render_enemy(widget: &mut impl Element, state: &ChoiceState, enemy_idx: usize) {
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
    });
}

fn render_enemies(widget: &mut impl Element, state: &ChoiceState) {
    //TODO - this should have a more clever layout.
    widget.layout().push_grid_template_row_fr(1.0);
    for i in 0..Game::MAX_ENEMIES {
        widget.layout().push_grid_template_column_fr(1.0);
        widget.child(|child| {
            child.layout().grid_col(i).grid_row(0);
            render_enemy(child, state, i);
        });
    }
}

fn vertical_breakdown<T: Element>(
    widget: &mut T,
    top: impl FnOnce(&mut T),
    middle: impl FnOnce(&mut T),
    bottom: impl FnOnce(&mut T),
) {
    widget.layout().height_percent(1.0).width_percent(1.0);
    widget
        .layout()
        .push_grid_template_column_fr(1.0)
        .push_grid_template_row_px(6)
        .push_grid_template_row_fr(1.0)
        .push_grid_template_row_px(8);
    let mut children = [
        widget.child(top),
        widget.child(middle),
        widget.child(bottom),
    ];
    for i in 0..children.len() {
        children[i].layout().grid_col(0).grid_row(i);
    }
}

fn render_rest_site(
    widget: &mut impl Element,
    _choice_state: &ChoiceState,
    rest_site_actions: Vec<RestSiteAction>,
) {
    vertical_breakdown(
        widget,
        |_top| {},
        |middle| {
            middle
                .layout()
                .flex_direction(taffy::FlexDirection::Row)
                .justify_content(taffy::AlignContent::Center)
                .align_items(taffy::AlignItems::Center);
            for i in 0..rest_site_actions.len() {
                middle.child(|child| {
                    BorderWidget::builder(child, |center| {
                        center
                            .layout()
                            .border_left_px(1)
                            .border_top_px(1)
                            .border_right_px(1)
                            .border_bottom_px(1);
                        let mut text_region = TextRegion::new(center);
                        writeln!(&mut text_region, "{:?}", rest_site_actions[i]);
                    })
                    .build();
                });
            }
        },
        |_bottom| {},
    );
}

fn render_game_over(widget: &mut impl Element, choice_state: &ChoiceState) {
    vertical_breakdown(
        widget,
        |_top| {},
        |middle| {
            middle
                .layout()
                .justify_content(taffy::AlignContent::Center)
                .align_items(taffy::AlignItems::Center);
            middle.child(|child| render_game_over_box(child, choice_state));
        },
        |_bottom| {},
    );
}

fn render_game_over_box(widget: &mut impl Element, choice_state: &ChoiceState) {
    BorderWidget::builder(widget, |center| {
        let mut text_region = TextRegion::new(center);
        match &choice_state.choice() {
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
fn render_battlefield(widget: &mut impl Element, choice_state: &ChoiceState) {
    let [_top, mut middle, mut bottom] = vertical_breakdown(widget);
    middle.layout().push_grid_template_row_fr(1.0);
    let [mut player_box, mut fight_box] = array::from_fn(|i| {
        middle.child().apply(|w| {
            middle.layout().push_grid_template_column_fr(1.0);
            w.layout().grid_col(i).grid_row(0);
        })
    });
    render_player(&mut player_box, choice_state);
    render_cards(&mut bottom, choice_state);
    render_enemies(&mut fight_box, choice_state);
}

pub fn render_card_view(
    widget: &mut impl Element,
    choice_state: &ChoiceState,
    actions: Vec<SelectDeckCardAction>,
) {
    widget.layout().height_percent(1.0).width_percent(1.0);
    widget
        .layout()
        .push_grid_template_column_fr(1.0)
        .push_grid_template_row_px(6)
        .push_grid_template_row_fr(1.0);
    array::from_fn(|i| {
        widget.child(|w| {
            w.layout().grid_col(0).grid_row(i);
            //TODO - Fix layout!
            render_card_view_inner(&mut middle, choice_state, actions);
        })
    });
}

pub fn render_card_view_inner(
    widget: &mut impl Element,
    choice_state: &ChoiceState,
    actions: Vec<SelectDeckCardAction>,
) {
    widget
        .layout()
        .flex_direction(FlexDirection::Row)
        .flex_wrap(FlexWrap::Wrap);
    for action in &actions {
        choice_state.game().base_deck().get(action.0).map(|card| {
            widget.child(|child| render_card(child, choice_state, card));
        });
    }
}

pub fn render_map_state(widget: &mut impl Element, choice_state: &ChoiceState) {
    let game = &**choice_state.game();
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

pub struct RootState<'a> {
    choice_state: &'a ChoiceState,
}

impl<'a> WidgetRoot for RootState<'a> {
    fn ui<T: Element>(&mut self, root: &mut T) {
        match self.choice_state.choice().clone() {
            crate::game::choice::Choice::PlayCardState(play_card_actions) => {
                root.child(|elem| {
                    render_battlefield(elem, self.choice_state);
                });
            }
            crate::game::choice::Choice::ChooseEnemyState(choose_enemy_actions, _) => {
                root.child(|elem| {
                    render_battlefield(elem, self.choice_state);
                });
            }
            crate::game::choice::Choice::Win => {
                root.child(|elem| {
                    render_game_over(elem, self.choice_state);
                });
            }
            crate::game::choice::Choice::Loss => {
                root.child(|elem| {
                    render_game_over(elem, self.choice_state);
                });
            }
            crate::game::choice::Choice::MapState(map_state_actions) => {
                root.child(|elem| {
                    render_map_state(elem, self.choice_state);
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
                root.child(|elem| {
                    render_card_view(elem, self.choice_state, actions);
                });
            }
            crate::game::choice::Choice::RestSite(actions) => {
                root.child(|elem| {
                    render_rest_site(elem, self.choice_state, actions);
                });
            }
        }
    }
}
