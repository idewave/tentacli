use crossterm::event::KeyCode;
use ratatui::Frame;
use ratatui::layout::{Constraint, Rect};
use ratatui::prelude::Color::LightYellow;
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::symbols::border;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Clear, ListItem};
use tentacli_traits::types::Outputs;

use crate::features::ui2::components::ComponentEvent;
use crate::features::ui2::components::list::StyledList;
use crate::features::ui2::layout::center;
use crate::features::ui2::traits::{EventHandler, UIComponent};

pub type ModalCallback = Box<dyn FnMut(Vec<usize>) -> anyhow::Result<ModalResponse> + Send>;

#[derive(Default)]
pub enum ModalResponse {
    #[default]
    None,
    External(Outputs),
    Internal(Vec<usize>),
}

#[derive(Default)]
pub struct Modal<'a> {
    list: StyledList<'a>,
    title: String,
    opened: bool,
    callback: Option<ModalCallback>,
}

impl<'a> Modal<'a> {
    pub fn set_title<S: ToString>(mut self, title: S) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn set_items(mut self, items: Vec<ListItem<'a>>) -> Self {
        self.list.set_items(items);
        self
    }

    pub fn set_callback(mut self, callback: ModalCallback) -> Self {
        self.callback = Some(callback);
        self
    }

    pub fn open(&mut self) {
        self.opened = true;
    }

    pub fn close(&mut self) {
        self.opened = false;
    }

    pub fn toggle(&mut self) {
        self.opened = !self.opened;
    }

    pub fn opened(&mut self) -> bool {
        self.opened
    }
}

impl<'a> UIComponent for Modal<'a> {
    fn render(&mut self, frame: &mut Frame, rect: Rect) {
        if self.opened() {
            let instructions = {
                let mut items = vec![
                    Span::styled("To navigate use ", Style::new().add_modifier(Modifier::BOLD)),
                    Span::styled(
                        "<ArrowDn>",
                        Style::new().fg(Color::LightBlue).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(" and ", Style::new().add_modifier(Modifier::BOLD)),
                    Span::styled(
                        "<ArrowUp>",
                        Style::new().fg(Color::LightBlue).add_modifier(Modifier::BOLD),
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
                            Style::new().fg(Color::LightGreen).add_modifier(Modifier::BOLD),
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
                // .style(Style::default().bg(Color::Rgb(140, 160, 140)))
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

impl<'a> EventHandler for Modal<'a> {
    type Output = ModalResponse;

    fn handle_event(&mut self, event: &ComponentEvent) -> anyhow::Result<Self::Output> {
        let mut output = ModalResponse::None;

        if self.opened() {
            if let ComponentEvent::KeyEvent(code, _) = event {
                // allow Enter only if something was selected
                if *code == KeyCode::Enter && self.list.selected().is_some() {
                    if let Some(callback) = self.callback.as_mut() {
                        output = if !self.list.markable() {
                            // single select list with selected item
                            callback(vec![self.list.selected().unwrap()])?
                        } else if !self.list.marked.is_empty() {
                            // multi select list with selected item(s)
                            callback(self.list.marked.iter().cloned().collect())?
                        } else {
                            // multi select list without selected items
                            ModalResponse::None
                        };
                    }
                } else {
                    self.list.handle_event(event)?;
                }
            }
        }

        Ok(output)
    }
}