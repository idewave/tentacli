use std::sync::mpsc::Sender;
use chrono::Local;
use crossterm::event::{KeyCode, KeyModifiers};
use serde_json::Value;
use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, BorderType, Clear, List, ListItem, ListState};

use crate::ipc::pipe::types::LoggerOutput;
use crate::traits::paginator::Paginator;
use crate::traits::ui_component::UIComponent;
use crate::ui::MARGIN;
use crate::ui::types::{UIComponentOptions, UIStateFlags};

const PANEL_TITLE: &str = "DEBUG";

pub struct DebugPanel<'a> {
    items: Vec<ListItem<'a>>,
    details: Vec<String>,
    state: ListState,
    debug_mode: bool,
    sender: Sender<String>,
    start_index: usize,
    per_page: u16,
    absolute_index: Option<usize>,
}

impl<'a> DebugPanel<'a> {
    pub fn set_mode(&mut self, debug_mode: bool) -> &mut Self {
        self.debug_mode = debug_mode;
        self
    }

    pub fn add_item(&mut self, output: LoggerOutput) -> &mut Self {
        let message = self.generate_message(output);

        if message.is_some() {
            self.items.push(ListItem::new(message.unwrap()));
        }

        self
    }

    fn generate_message(&mut self, output: LoggerOutput) -> Option<Spans<'a>> {
        let time_block = Self::generate_time_block();

        let message = match output {
            LoggerOutput::Debug(data) if !data.is_empty() => {
                let (message, details) = Self::parse_input(data);
                self.details.push(details);

                Spans::from(vec![
                    time_block,
                    Span::styled(
                        format!("[DEBUG]: {}", message),
                        Style::default().fg(Color::DarkGray),
                    ),
                ])
            },
            LoggerOutput::Error(data) if !data.is_empty() => {
                let (message, details) = Self::parse_input(data);
                self.details.push(details);

                Spans::from(vec![
                    time_block,
                    Span::styled(
                        format!("[ERROR]: {}", message),
                        Style::default().fg(Color::Red),
                    ),
                ])
            },
            LoggerOutput::Success(data) if !data.is_empty() => {
                let (message, details) = Self::parse_input(data);
                self.details.push(details);

                Spans::from(vec![
                    time_block,
                    Span::styled(
                        format!("[SUCCESS]: {}", message),
                        Style::default().fg(Color::LightGreen),
                    ),
                ])
            },
            LoggerOutput::Server(data) if !data.is_empty() => {
                let (message, details) = Self::parse_input(data);
                self.details.push(details);

                Spans::from(vec![
                    time_block,
                    Span::styled(
                        format!("[SERVER]: {}", message),
                        Style::default().fg(Color::LightMagenta),
                    ),
                ])
            },
            LoggerOutput::Client(data) if !data.is_empty() => {
                let (message, details) = Self::parse_input(data);
                self.details.push(details);

                Spans::from(vec![
                    time_block,
                    Span::styled(
                        format!("[CLIENT]: {}", message),
                        Style::default().fg(Color::LightBlue),
                    ),
                ])
            },
            _ => {
                return None;
            },
        };

        Some(message)
    }

    pub fn prev(&mut self) {
        let absolute_index = match self.absolute_index {
            Some(i) => if i == 0 { self.items.len() - 1 } else { i - 1 },
            None => self.items.len() - 1,
        };
        self.calculate_indexes(absolute_index);
        self.sender.send(self.details[absolute_index].to_string()).unwrap();
    }

    pub fn next(&mut self) {
        let absolute_index = match self.absolute_index {
            Some(i) => if i >= self.items.len() - 1 { 0 } else { i + 1 },
            None => 0,
        };
        self.calculate_indexes(absolute_index);
        self.sender.send(self.details[absolute_index].to_string()).unwrap();
    }

    pub fn set_pagination(&mut self, per_page: u16) {
        self.per_page = per_page;
    }

    pub fn handle_key_event(
        &mut self,
        _: KeyModifiers,
        key_code: KeyCode,
        state_flags: &mut UIStateFlags
    ) {
        match key_code {
            KeyCode::Up => {
                if !state_flags.contains(UIStateFlags::IS_EVENT_HANDLED) {
                    if self.debug_mode {
                        self.prev();
                        state_flags.set(UIStateFlags::IS_EVENT_HANDLED, true);
                    }
                }
            },
            KeyCode::Down => {
                if !state_flags.contains(UIStateFlags::IS_EVENT_HANDLED) {
                    if self.debug_mode {
                        self.next();
                        state_flags.set(UIStateFlags::IS_EVENT_HANDLED, true);
                    }
                }
            },
            _ => {},
        }
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

    fn parse_input(data: String) -> (String, String) {
        let mut message = data.to_string();
        let mut details = String::new();
        if let Ok(json) = serde_json::from_str::<Value>(&data) {
            message = json["title"].to_string();
            if let Some(info) = json["details"].as_str() {
                details = info.to_string();
            }
        }

        (message, details)
    }

    fn generate_time_block() -> Span<'a> {
        let local_time = Local::now();

        Span::styled(
            format!("{} ", local_time.format("[%H:%M:%S]")),
            Style::default().fg(Color::LightYellow),
        )
    }
}

impl<'a> Paginator for DebugPanel<'a> {}
impl<'a> UIComponent for DebugPanel<'a> {
    fn new(options: UIComponentOptions) -> Self {
        Self {
            items: vec![],
            details: vec![],
            state: ListState::default(),
            debug_mode: false,
            sender: options.sender.clone(),
            start_index: 0,
            per_page: 0,
            absolute_index: None,
        }
    }

    fn render<B: Backend>(&mut self, frame: &mut Frame<B>, rect: Rect) {
        let block = Block::default()
            .title(PANEL_TITLE)
            .borders(Borders::ALL)
            .border_type(BorderType::Plain);

        let items_amount = self.items.len();

        let mut start_index: usize = 0;
        if items_amount > rect.height as usize {
            start_index = match self.debug_mode {
                true => self.start_index,
                _ => items_amount - (rect.height - MARGIN * 2) as usize,
            }
        }

        let mut list = List::new(self.items[start_index..].to_vec())
            .block(block)
            .style(Style::default().fg(Color::White));

        if self.debug_mode {
            list = list.highlight_style(
                Style::default().add_modifier(Modifier::ITALIC).bg(Color::Gray)
            );
        }

        frame.render_widget(Clear, rect);
        frame.render_stateful_widget(list, rect, &mut self.state);
    }
}