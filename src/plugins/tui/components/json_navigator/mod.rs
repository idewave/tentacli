use async_event_emitter::AsyncEventEmitter;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;
use ratatui::layout::{Margin, Rect};
use ratatui::prelude::{Color, Line, Modifier, Span, Style};
use ratatui::style::Stylize;
use ratatui::symbols::border;
use ratatui::widgets::{Block, Paragraph, Wrap};
use std::sync::Arc;

mod cursor;
pub mod events;
mod helpers;

use crate::client::Echo;
use crate::events_runtime;
use crate::plugins::tui::components::app::OutputItem;
use crate::plugins::tui::components::json_navigator::cursor::JsonCursor;
use crate::plugins::tui::components::json_navigator::helpers::{
    aggregate_field_span, clamp_scroll, ensure_visible, format_json, hex_with_highlight,
    json_with_highlight,
};
use crate::plugins::tui::events::traits::{
    EventHandler, EventSystem, EventsRuntime, WithEventSystem,
};
use crate::plugins::tui::layout::{Values, split_horizontal};
use crate::plugins::tui::theme::{KEY_BTN_BG, KEY_BTN_FG, TITLE_BG, TITLE_FG};
use crate::plugins::tui::traits::{Focusable, UIComponent};

#[derive(Default)]
pub struct JsonNavigator {
    cursor: JsonCursor,
    output_item: Option<OutputItem>,
    event_system: EventSystem,
    nav_active: bool,
    json_scroll: u16,
    hex_scroll: u16,
    ensure_visible: bool,
}

impl UIComponent for JsonNavigator {
    fn render(&mut self, frame: &mut Frame, rect: Rect) {
        let total = rect.width;
        let hex_width = 58.min(total / 2).max(24);

        let [left, right] = split_horizontal(rect, Values::Lengths([total - hex_width, hex_width]));

        let instructions = if self.nav_active {
            Line::from(vec![
                Span::styled(
                    "<ArrowUp/Down>",
                    Style::new()
                        .fg(KEY_BTN_FG)
                        .bg(KEY_BTN_BG)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(" move · "),
                Span::styled(
                    "<ArrowRight>",
                    Style::new()
                        .fg(KEY_BTN_FG)
                        .bg(KEY_BTN_BG)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(" enter · "),
                Span::styled(
                    "<ArrowLeft>",
                    Style::new()
                        .fg(KEY_BTN_FG)
                        .bg(KEY_BTN_BG)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(" up/exit"),
            ])
        } else {
            Line::from(vec![
                Span::styled(
                    "To enter NAV mode use ",
                    Style::new().add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "<ArrowRight>",
                    Style::new()
                        .fg(KEY_BTN_FG)
                        .bg(KEY_BTN_BG)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(" then arrows", Style::new().add_modifier(Modifier::BOLD)),
            ])
        };

        let show_path = matches!(self.output_item, Some(OutputItem::Packet(_)));

        let path_line = if show_path {
            let path_title = if self.nav_active {
                self.cursor.current_path().unwrap_or("/".into()).to_string()
            } else {
                "/".to_string()
            };

            Some(Line::from(Span::styled(
                format!(" {} ", path_title),
                Style::default()
                    .fg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
            )))
        } else {
            None
        };

        let mut left_block = Block::bordered()
            .title_bottom(instructions.centered())
            .border_set(border::ROUNDED);

        if let Some(hint) = path_line {
            left_block = left_block.title(hint.centered());
        }

        let is_packet = matches!(self.output_item, Some(OutputItem::Packet(_)));

        let hex_title = if is_packet {
            Line::from(
                Span::raw("HEX")
                    .fg(TITLE_FG)
                    .bg(TITLE_BG)
                    .add_modifier(Modifier::BOLD),
            )
        } else {
            Line::from(
                Span::raw("HEX — disabled")
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::DIM),
            )
        };

        let right_block = if is_packet {
            Block::bordered()
                .title(hex_title.right_aligned())
                .border_set(border::ROUNDED)
        } else {
            Block::bordered()
                .title(hex_title.right_aligned())
                .border_set(border::ROUNDED)
                .style(Style::default().fg(Color::DarkGray))
        };

        frame.render_widget(left_block, left);
        frame.render_widget(right_block, right);

        let left_inner = left.inner(Margin {
            vertical: 1,
            horizontal: 1,
        });

        let right_inner = right.inner(Margin {
            vertical: 1,
            horizontal: 1,
        });

        let Some(item) = &self.output_item else {
            return;
        };

        match item {
            OutputItem::Message(msg) => {
                frame.render_widget(
                    Paragraph::new(Span::styled(
                        msg.text.clone(),
                        Style::new().fg(Color::White).add_modifier(Modifier::BOLD),
                    ))
                    .wrap(Wrap { trim: false }),
                    left_inner,
                );

                frame.render_widget(Paragraph::new(Line::from("")), right_inner);
            }
            OutputItem::Packet(packet) => {
                let json = format_json(&packet.content.json);
                let highlight = if self.nav_active {
                    self.cursor.current_span()
                } else {
                    None
                };

                let (json_lines, hl_range) = json_with_highlight(&json, highlight);
                let total_lines = json_lines.len();
                let view_height = left_inner.height as usize;
                let max_scroll = total_lines.saturating_sub(view_height);

                if self.nav_active {
                    if let Some((start, end)) = hl_range {
                        if self.ensure_visible {
                            ensure_visible(
                                &mut self.json_scroll,
                                start,
                                end,
                                left_inner.height as usize,
                            );
                        }

                        clamp_scroll(
                            &mut self.json_scroll,
                            start,
                            end,
                            left_inner.height as usize,
                        );
                    }
                } else {
                    self.json_scroll = 0;
                }

                self.json_scroll = self.json_scroll.min(max_scroll as u16);

                let json_paragraph = Paragraph::new(json_lines)
                    .scroll((self.json_scroll, 0))
                    .wrap(Wrap { trim: false });

                frame.render_widget(json_paragraph, left_inner);

                let hex_highlight = if self.nav_active {
                    self.cursor.current_path().and_then(|p| {
                        packet
                            .metadata
                            .offsets_info
                            .get(&p)
                            .cloned()
                            .or_else(|| aggregate_field_span(&packet.metadata.offsets_info, &p))
                    })
                } else {
                    None
                };

                let hex_lines = hex_with_highlight(&packet.content.body, hex_highlight.clone());

                let total_hex_lines = hex_lines.len();
                let hex_view_height = right_inner.height as usize;
                let max_hex_scroll = total_hex_lines.saturating_sub(hex_view_height);

                if self.nav_active && self.ensure_visible {
                    if let Some(span) = &hex_highlight {
                        let line = span.offset / 16;
                        ensure_visible(
                            &mut self.hex_scroll,
                            line,
                            line,
                            right_inner.height as usize,
                        );
                    }
                } else if !self.nav_active {
                    self.hex_scroll = 0;
                }

                self.hex_scroll = self.hex_scroll.min(max_hex_scroll as u16);

                let hex_paragraph = Paragraph::new(hex_lines)
                    .scroll((self.hex_scroll, 0))
                    .wrap(Wrap { trim: false });

                frame.render_widget(hex_paragraph, right_inner);

                self.ensure_visible = false;
            }
        }
    }
}

impl Focusable for JsonNavigator {
    fn holds_focus(&mut self) -> bool {
        self.nav_active
    }

    fn on_focus(&mut self) {
        self.nav_active = true;
        self.cursor.step_forward();
    }
}

impl WithEventSystem for JsonNavigator {
    fn event_system(&mut self) -> &mut EventSystem {
        &mut self.event_system
    }
}

impl EventHandler<KeyEvent> for JsonNavigator {
    async fn callback(&mut self, event: KeyEvent) -> anyhow::Result<Vec<Echo>> {
        let mut moved = false;

        match event.code {
            KeyCode::Up => {
                if self.nav_active && {
                    moved = self.cursor.step_backward().is_some();
                    !moved
                } {
                    self.json_scroll = self.json_scroll.saturating_sub(1);
                    self.hex_scroll = self.hex_scroll.saturating_sub(1);
                }
            }
            KeyCode::Down => {
                if self.nav_active && {
                    moved = self.cursor.step_forward().is_some();
                    !moved
                } {
                    self.json_scroll = self.json_scroll.saturating_add(1);
                    self.hex_scroll = self.hex_scroll.saturating_add(1);
                }
            }
            KeyCode::Left => {
                if self.cursor.is_at_root() {
                    self.nav_active = false;
                    self.json_scroll = 0;
                    self.hex_scroll = 0;
                } else {
                    let before = self.cursor.state.path_len();
                    self.cursor.step_out();
                    moved = self.cursor.state.path_len() != before;
                }
            }
            KeyCode::Right => {
                self.cursor.step_into();
                moved = self.cursor.step_forward().is_some();
            }
            _ => {}
        }

        if moved {
            self.ensure_visible = true;
        }

        Ok(vec![])
    }
}

impl EventHandler<events::SetItem> for JsonNavigator {
    async fn callback(&mut self, event: events::SetItem) -> anyhow::Result<Vec<Echo>> {
        let events::SetItem(item) = event;

        self.cursor.reset();
        self.nav_active = false;

        if let OutputItem::Packet(packet) = &item {
            let _ = self.cursor.set_json(&packet.content.json);
        }

        self.output_item = Some(item);
        self.json_scroll = 0;
        self.hex_scroll = 0;
        self.ensure_visible = false;

        Ok(vec![])
    }
}

// impl EventHandler<events::SetItem> for JsonNavigator {
//     async fn callback(&mut self, event: events::SetItem) -> anyhow::Result<Vec<Echo>> {
//         let events::SetItem(item) = event;
//
//         if let OutputItem::Packet(packet) = &item {
//             let _ = self.iterator.set_json(&packet.content.json);
//         }
//
//         self.output_item = Some(item);
//         self.json_scroll = 0;
//         self.hex_scroll = 0;
//
//         Ok(vec![])
//     }
// }

events_runtime! {
    impl EventsRuntime for JsonNavigator {
        local_events = [KeyEvent];
        parent_events = [KeyEvent, events::SetItem];
    }
}
