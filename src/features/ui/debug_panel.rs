use std::sync::{Arc, Mutex as SyncMutex};
use chrono::Local;
use crossterm::event::{KeyCode, KeyModifiers};
use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, BorderType, Clear, List, ListItem, ListState};

use crate::features::ui::debug_details_panel::DebugDetailsPanel;
use crate::features::ui::info_panel::InfoPanel;

use crate::primary::ipc::pipe::types::LoggerOutput;
use crate::primary::traits::paginator::Paginator;
use crate::features::ui::traits::ui_component::{UIComponent};
use crate::features::ui::types::{UIEventFlags};
use crate::primary::types::HandlerOutput;

const PANEL_TITLE: &str = "I/O MONITOR";

struct Item {
    title: String,
    local_time: String,
    details: Option<String>,
    label: String,
    color: Color,
}

pub struct DebugPanel {
    items: Vec<Item>,
    state: ListState,
    start_index: usize,
    per_page: u16,
    absolute_index: Option<usize>,
    total_response_amount: usize,
    total_request_amount: usize,
    total_errors_amount: usize,
    selected_output: String,

    // internal components
    details_panel: DebugDetailsPanel,
    info_panel: InfoPanel,
}

impl DebugPanel {
    pub fn get_selected_index(&mut self) -> usize {
        self.absolute_index.unwrap_or(0)
    }

    pub fn get_total_items(&mut self) -> usize {
        self.items.len()
    }

    pub fn get_total_response_amount(&mut self) -> usize {
        self.total_response_amount
    }

    pub fn get_total_request_amount(&mut self) -> usize {
        self.total_request_amount
    }

    pub fn get_total_errors_amount(&mut self) -> usize {
        self.total_errors_amount
    }

    pub fn add_item(&mut self, output: LoggerOutput) -> &mut Self {
        if let Some(item) = self.generate_item(output) {
            self.items.push(item);
        }
        self
    }

    fn generate_item(&mut self, output: LoggerOutput) -> Option<Item> {
        let local_time = Local::now().format("[%H:%M:%S]").to_string();

        match output {
            LoggerOutput::Debug(title, details) if !title.is_empty() => {
                Some(Item {
                    title,
                    local_time,
                    details,
                    label: "[DEBUG]".to_string(),
                    color: Color::DarkGray,
                })
            },
            LoggerOutput::Error(title, details) if !title.is_empty() => {
                self.total_errors_amount += 1;
                Some(Item {
                    title,
                    local_time,
                    details,
                    label: "[ERROR]".to_string(),
                    color: Color::Red,
                })
            },
            LoggerOutput::Success(title, details) if !title.is_empty() => {
                Some(Item {
                    title,
                    local_time,
                    details,
                    label: "[SUCCESS]".to_string(),
                    color: Color::LightGreen,
                })
            },
            LoggerOutput::Response(title, details) if !title.is_empty() => {
                self.total_response_amount += 1;

                Some(Item {
                    title,
                    local_time,
                    details,
                    label: "[RESPONSE]".to_string(),
                    color: Color::LightMagenta,
                })
            },
            LoggerOutput::Request(title, details) if !title.is_empty() => {
                self.total_request_amount += 1;

                Some(Item {
                    title,
                    local_time,
                    details,
                    label: "[REQUEST]".to_string(),
                    color: Color::LightBlue,
                })
            },
            _ => {
                None
            }
        }
    }

    pub fn prev(&mut self) {
        let next_index = match self.absolute_index {
            Some(i) => if i == 0 { self.items.len() - 1 } else { i - 1 },
            None => self.items.len() - 1,
        };
        self.calculate_indexes(next_index);
        self.set_selected_output(self.items[next_index].details.clone());
    }

    pub fn next(&mut self) {
        let next_index = match self.absolute_index {
            Some(i) => if i >= self.items.len() - 1 { 0 } else { i + 1 },
            None => 0,
        };
        self.calculate_indexes(next_index);
        self.set_selected_output(self.items[next_index].details.clone());
    }

    pub fn set_pagination(&mut self, per_page: u16) {
        self.per_page = per_page;
    }

    pub fn handle_key_event(
        &mut self,
        key_modifiers: KeyModifiers,
        key_code: KeyCode,
        event_flags: Arc<SyncMutex<UIEventFlags>>
    ) -> Option<HandlerOutput> {
        let output = None;

        self.details_panel.handle_key_event(key_modifiers, key_code, event_flags.clone());
        let mut guard = event_flags.lock().unwrap();

        match key_code {
            KeyCode::Up if !key_modifiers.contains(KeyModifiers::CONTROL) => {
                if !guard.contains(UIEventFlags::IS_EVENT_HANDLED) {
                    self.prev();
                    guard.set(UIEventFlags::IS_EVENT_HANDLED, true);
                }
            },
            KeyCode::Down if !key_modifiers.contains(KeyModifiers::CONTROL) => {
                if !guard.contains(UIEventFlags::IS_EVENT_HANDLED) {
                    self.next();
                    guard.set(UIEventFlags::IS_EVENT_HANDLED, true);
                }
            },
            KeyCode::PageUp if !key_modifiers.contains(KeyModifiers::CONTROL) => {
                if !guard.contains(UIEventFlags::IS_EVENT_HANDLED) {
                    let next_index = match self.absolute_index {
                        Some(i) => if i >= self.per_page as usize {
                            i - (self.per_page as usize)
                        } else {
                            0
                        },
                        None => 0,
                    };
                    self.calculate_indexes(next_index);
                    self.set_selected_output(self.items[next_index].details.clone());
                    guard.set(UIEventFlags::IS_EVENT_HANDLED, true);
                }
            },
            KeyCode::PageDown if !key_modifiers.contains(KeyModifiers::CONTROL) => {
                if !guard.contains(UIEventFlags::IS_EVENT_HANDLED) {
                    let next_index = match self.absolute_index {
                        Some(i) => if i + (self.per_page as usize) <= self.items.len() - 1 {
                            i + (self.per_page as usize)
                        } else {
                            self.items.len() - 1
                        },
                        None => 0,
                    };
                    self.calculate_indexes(next_index);
                    self.set_selected_output(self.items[next_index].details.clone());
                    guard.set(UIEventFlags::IS_EVENT_HANDLED, true);
                }
            },
            KeyCode::Home if !key_modifiers.contains(KeyModifiers::CONTROL) => {
                if !guard.contains(UIEventFlags::IS_EVENT_HANDLED) {
                    let next_index = 0;
                    self.calculate_indexes(next_index);
                    self.set_selected_output(self.items[next_index].details.clone());
                    guard.set(UIEventFlags::IS_EVENT_HANDLED, true);
                }
            },
            KeyCode::End if !key_modifiers.contains(KeyModifiers::CONTROL) => {
                if !guard.contains(UIEventFlags::IS_EVENT_HANDLED) {
                    let next_index = self.items.len() - 1;
                    self.calculate_indexes(next_index);
                    self.set_selected_output(self.items[next_index].details.clone());
                    guard.set(UIEventFlags::IS_EVENT_HANDLED, true);
                }
            },
            _ => {},
        };

        output
    }

    fn calculate_indexes(&mut self, absolute_index: usize) {
        self.absolute_index = Some(absolute_index);

        let page = Self::get_page_number(
            absolute_index, self.items.len(), self.per_page as usize
        ).unwrap();

        self.start_index = page * self.per_page as usize;

        let relative_index = Self::get_relative_index(
            page, self.per_page as usize, absolute_index
        ).unwrap();

        self.state.select(Some(relative_index));
    }

    fn set_selected_output(&mut self, output: Option<String>) {
        if let Some(value) = output {
            self.selected_output = value;
        } else {
            self.selected_output = String::new();
        }
    }
}

impl Paginator for DebugPanel {}
impl UIComponent for DebugPanel {
    fn new() -> Self {
        let details_panel = DebugDetailsPanel::new();
        let info_panel = InfoPanel::new();

        Self {
            items: vec![],
            state: ListState::default(),
            start_index: 0,
            per_page: 0,
            absolute_index: None,
            total_response_amount: 0,
            total_request_amount: 0,
            total_errors_amount: 0,
            selected_output: String::default(),
            details_panel,
            info_panel,
        }
    }

    fn render<B: Backend>(&mut self, frame: &mut Frame<B>, rect: Rect) {
        let total_response_amount = { self.get_total_response_amount() };
        let total_request_amount = { self.get_total_request_amount() };
        let total_errors_amount = { self.get_total_errors_amount() };
        let total_items = { self.get_total_items() };
        let selected_index = { self.get_selected_index() };

        self.details_panel.set_output(self.selected_output.clone());

        let horizontal_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(70),
                Constraint::Percentage(30),
            ])
            .split(rect);

        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Max(88),
                Constraint::Length(3),
            ])
            .split(horizontal_chunks[0]);

        self.set_pagination(horizontal_chunks[0].height);

        self.info_panel
            .set_total_response_amount(total_response_amount)
            .set_total_request_amount(total_request_amount)
            .set_total_errors_amount(total_errors_amount);

        self.info_panel.set_total_items(total_items).set_selected_index(selected_index);

        let block = Block::default()
            .title(PANEL_TITLE)
            .borders(Borders::ALL)
            .border_type(BorderType::Plain);

        let items_amount = self.items.len();

        let mut start_index: usize = 0;
        if items_amount > rect.height as usize {
            start_index = self.start_index;
        }

        let items: Vec<ListItem> = self.items
            .iter()
            .map(|item| {
                ListItem::new(Spans::from(vec![
                    Span::styled(
                        format!("{} ", item.local_time),
                        Style::default().fg(Color::LightYellow),
                    ),
                    Span::styled(
                        format!("{}: {}", item.label, item.title),
                        Style::default().fg(item.color),
                    ),
                ]))
            })
            .collect();

        let mut list = List::new(items[start_index..].to_vec())
            .block(block)
            .style(Style::default().fg(Color::White));

        list = list.highlight_style(
            Style::default().add_modifier(Modifier::ITALIC).bg(Color::Gray)
        );

        frame.render_widget(Clear, rect);
        frame.render_stateful_widget(list, vertical_chunks[0], &mut self.state);

        self.details_panel.render(frame, horizontal_chunks[1]);

        self.info_panel.render(frame, vertical_chunks[1]);
    }
}