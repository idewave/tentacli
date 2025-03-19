use std::collections::{HashMap, HashSet};

use crossterm::event::KeyCode;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style, Styled, Stylize};
use ratatui::widgets::{List, ListItem, ListState};
use tentacli_traits::types::Outputs;

use crate::features::ui2::components::ComponentEvent;
use crate::features::ui2::traits::{EventHandler, Paginator, UIComponent};

pub type Marker = u8;

#[derive(Default)]
pub struct StyledList<'a> {
    pub counter: HashMap<Marker, usize>,
    pub total_items: usize,
    pub marked: HashSet<usize>,
    items: Vec<(Marker, ListItem<'a>)>,
    state: ListState,
    markable: bool,
    scroll_mode: ScrollMode,
}

impl<'a> StyledList<'a> {
    pub fn set_markable(mut self) -> Self {
        self.markable = true;
        self
    }

    pub fn markable(&mut self) -> bool {
        self.markable
    }

    pub fn selected(&mut self) -> Option<usize> {
        self.state.selected()
    }

    pub fn add_item(&mut self, marker: Marker, item: ListItem<'a>) {
        self.items.push((marker, item));
        *self.counter.entry(marker).or_insert(0) += 1;
        self.total_items += 1;

        if self.scroll_mode == ScrollMode::Auto {
            self.state.select_last();
        }
    }

    pub fn set_items(&mut self, items: Vec<ListItem<'a>>) {
        self.items = items.into_iter().map(|item| (Marker::default(), item)).collect();
        self.total_items = self.items.len();
    }

    pub fn mark_item(&mut self) {
        if let Some(index) = self.state.selected() {
            self.toggle(index);
        }
    }

    fn toggle(&mut self, index: usize) {
        if self.marked.take(&index).is_none() {
            self.marked.insert(index);
        }
    }

    pub fn contains(&mut self, index: usize) -> bool {
        self.marked.contains(&index)
    }

    pub fn autoscroll(&mut self, enabled: bool) {
        self.scroll_mode = if enabled { ScrollMode::Auto } else { ScrollMode::Manual };
    }
}

impl<'a> UIComponent for StyledList<'a> {
    fn render(&mut self, frame: &mut Frame, rect: Rect) {
        let items: Vec<ListItem> = self.items
            .iter()
            .map(|(_, item)| item.clone())
            .collect();

        let list = List::new(items).highlight_style(
            Style::new().fg(Color::LightRed).bg(Color::LightYellow).add_modifier(Modifier::BOLD)
        );

        frame.render_stateful_widget(list, rect, &mut self.state);
    }
}

impl<'a> EventHandler for StyledList<'a> {
    type Output = Option<Outputs>;

    fn handle_event(&mut self, event: &ComponentEvent) -> anyhow::Result<Self::Output> {
        let output = vec![];

        if let ComponentEvent::KeyEvent(code, _) = event {
            match code {
                KeyCode::Down => {
                    self.autoscroll(false);
                    self.next();
                }
                KeyCode::Up => {
                    self.autoscroll(false);
                    self.prev();
                }
                KeyCode::Home => {
                    self.autoscroll(false);
                    self.first();
                }
                KeyCode::End => {
                    self.autoscroll(true);
                    self.last();
                }
                KeyCode::Esc => {
                    self.autoscroll(true);
                }
                KeyCode::Char(' ') if self.markable() => self.mark_item(),
                _ => {}
            }
        }

        Ok(Some(output))
    }
}

impl<'a> Paginator for StyledList<'a> {
    fn prev(&mut self) {
        let first_index = 0;

        match self.state.selected() {
            Some(index) if index == first_index => self.state.select_last(),
            Some(index) => self.state.select(Some(index - 1)),
            None => self.state.select_last()
        }
    }

    fn next(&mut self) {
        match self.state.selected() {
            Some(index) if index == (self.total_items - 1) => self.state.select_first(),
            Some(index) => self.state.select(Some(index + 1)),
            None => self.state.select_first(),
        }
    }

    fn first(&mut self) {
        self.state.select_first();
    }

    fn last(&mut self) {
        self.state.select_last();
    }
}

#[derive(Default, PartialEq)]
enum ScrollMode {
    Auto,
    #[default]
    Manual,
}