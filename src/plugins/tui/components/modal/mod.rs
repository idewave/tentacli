use std::sync::Arc;
use async_event_emitter::AsyncEventEmitter;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;
use ratatui::layout::{Constraint, Rect};
use ratatui::prelude::Color::LightYellow;
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::symbols::border;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Clear, ListItem};

pub mod events;

use crate::events_runtime;
use crate::client::prelude::*;
use crate::plugins::tui::components::app::events::ChoicesEvent;
use crate::plugins::tui::components::list::{ItemType, StyledList};
use crate::plugins::tui::events::traits::{EventHandler, EventSystem, EventsRuntime, WithEventSystem};
use crate::plugins::tui::layout::center;
use crate::plugins::tui::theme::{KEY_BTN_BG, KEY_BTN_FG};
use crate::plugins::tui::traits::{Focusable, UIComponent};

#[derive(Default)]
pub struct Modal {
    list: StyledList,
    title: String,
    opened: bool,
    event_system: EventSystem,
}

impl Modal {
    pub fn set_items(&mut self, items: ChoiceItems) {
        self.list.set_items(ItemType::Message(MsgType::default()), items
            .iter()
            .map(|item| ListItem::new(
                Line::from(vec![
                    Span::styled(
                        item.to_string(),
                        Style::new().fg(Color::Magenta).add_modifier(Modifier::BOLD)
                    )
                ]))
            )
            .collect());
    }

    pub fn open(&mut self) {
        self.opened = true;
    }

    pub fn close(&mut self) {
        self.opened = false;
    }

    pub fn opened(&self) -> bool {
        self.opened
    }
}

impl UIComponent for Modal {
    fn render(&mut self, frame: &mut Frame, rect: Rect) {
        if self.opened() {
            let instructions = {
                let mut items = vec![
                    Span::styled("To navigate use ", Style::new().add_modifier(Modifier::BOLD)),
                    Span::styled(
                        "<ArrowDn>",
                        Style::new().fg(KEY_BTN_FG).bg(KEY_BTN_BG).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(" and ", Style::new().add_modifier(Modifier::BOLD)),
                    Span::styled(
                        "<ArrowUp>",
                        Style::new().fg(KEY_BTN_FG).bg(KEY_BTN_BG).add_modifier(Modifier::BOLD),
                    ),
                ];

                if self.list.selected().is_some() {
                    items.extend_from_slice(&[
                        Span::styled(
                            ", to confirm press ",
                            Style::new().add_modifier(Modifier::BOLD),
                        ),
                        Span::styled(
                            "<Enter>",
                            Style::new().fg(KEY_BTN_FG).bg(KEY_BTN_BG).add_modifier(Modifier::BOLD),
                        ),
                    ]);
                }

                if self.list.markable() {
                    items.extend_from_slice(&[
                        Span::styled(", to mark use ", Style::new().add_modifier(Modifier::BOLD)),
                        Span::styled(
                            "<Space>",
                            Style::new().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                        ),
                    ]);
                }

                Line::from(items)
            };

            let title = Line::from(
                Span::raw(&self.title)
                    .fg(Color::LightRed).bg(LightYellow).add_modifier(Modifier::BOLD)
            );

            let block = Block::bordered()
                .title(title.centered())
                .title_bottom(instructions.centered())
                .border_set(border::ROUNDED);

            frame.render_widget(Clear, rect);
            frame.render_widget(block, rect);

            self.list.render(
                frame,
                center(rect, Constraint::Percentage(50), Constraint::Percentage(50)),
            );
        }
    }
}

impl Focusable for Modal {
    fn holds_focus(&mut self) -> bool {
        self.opened()
    }
}

impl WithEventSystem for Modal {
    fn event_system(&mut self) -> &mut EventSystem {
        &mut self.event_system
    }
}

impl EventHandler<KeyEvent> for Modal {
    async fn callback(&mut self, event: KeyEvent) -> anyhow::Result<Vec<Echo>> {
        let mut output = vec![];

        if event.code == KeyCode::Enter
            && let Some(selected) = self.list.selected()
        {
            output.push(Echo::Choose(selected));
            self.close();
            self.emit_up(events::Close).await?;
        }

        self.list.emit_down(event).await?;

        Ok(output)
    }
}

impl EventHandler<ChoicesEvent> for Modal {
    async fn callback(&mut self, event: ChoicesEvent) -> anyhow::Result<Vec<Echo>> {
        self.open();

        let ChoicesEvent(items) = event;
        self.set_items(items);

        Ok(vec![])
    }
}

events_runtime! {
    impl EventsRuntime for Modal {
        local_events = [KeyEvent];
        parent_events = [KeyEvent, ChoicesEvent];
        children = [list];
    }
}
