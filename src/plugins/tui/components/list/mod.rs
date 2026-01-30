use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use async_event_emitter::AsyncEventEmitter;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{List, ListItem, ListState};

pub mod events;

use crate::events_runtime;
use crate::client::prelude::*;
use crate::plugins::tui::events::traits::{
    EventHandler, EventSystem, EventsRuntime, WithEventSystem
};
use crate::plugins::tui::theme::{LIST_HIGHLIGHT_BG, LIST_HIGHLIGHT_FG};
use crate::plugins::tui::traits::{Paginator, UIComponent};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum ItemType {
    Message(MsgType),
    Packet(PacketType),
}

#[derive(Default)]
pub struct StyledList {
    pub counter: HashMap<ItemType, usize>,
    pub total_items: usize,
    pub marked: HashSet<usize>,
    items: Vec<(ItemType, ListItem<'static>)>,
    state: ListState,
    markable: bool,
    scroll_mode: ScrollMode,
    event_system: EventSystem,
}

impl StyledList {
    #[allow(dead_code)]
    pub fn set_markable(mut self) -> Self {
        self.markable = true;
        self
    }

    #[allow(dead_code)]
    pub fn markable(&self) -> bool {
        self.markable
    }

    pub fn selected(&self) -> Option<Vec<usize>> {
        let mut selected = vec![];

        if self.markable {
            selected.extend(self.marked.iter().copied());
        } else if let Some(index) = self.state.selected() {
            selected.push(index);
        }

        if selected.is_empty() {
            None
        } else {
            Some(selected)
        }
    }

    pub async fn add_item(
        &mut self,
        item_type: ItemType,
        item: ListItem<'static>
    ) -> anyhow::Result<()> {
        self.items.push((item_type, item));
        *self.counter.entry(item_type).or_insert(0) += 1;
        self.total_items += 1;

        if self.scroll_mode == ScrollMode::Auto {
            self.state.select_last();
            self.emit_up(events::SelectionChanged).await?;
        }

        Ok(())
    }

    pub fn set_items(&mut self, item_type: ItemType, items: Vec<ListItem<'static>>) {
        self.items = items.into_iter().map(|item| (item_type, item)).collect();
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

    #[allow(dead_code)]
    pub fn contains(&mut self, index: usize) -> bool {
        self.marked.contains(&index)
    }

    pub fn autoscroll(&mut self, enabled: bool) {
        self.scroll_mode = if enabled { ScrollMode::Auto } else { ScrollMode::Manual };
    }
}

impl UIComponent for StyledList {
    fn render(&mut self, frame: &mut Frame, rect: Rect) {
        let items: Vec<ListItem> = self.items
            .iter()
            .map(|(_, item)| item.clone())
            .collect();

        let list = List::new(items).highlight_style(
            Style::new()
                .fg(LIST_HIGHLIGHT_FG)
                .bg(LIST_HIGHLIGHT_BG)
                .add_modifier(Modifier::BOLD),
        );

        frame.render_stateful_widget(list, rect, &mut self.state);
    }

}

impl WithEventSystem for StyledList {
    fn event_system(&mut self) -> &mut EventSystem {
        &mut self.event_system
    }
}

impl EventHandler<KeyEvent> for StyledList {
    async fn callback(&mut self, event: KeyEvent) -> anyhow::Result<Vec<Echo>> {
        match event.code {
            KeyCode::Down => {
                self.autoscroll(false);
                self.next();
                self.emit_up(events::SelectionChanged).await?;
            }
            KeyCode::Up => {
                self.autoscroll(false);
                self.prev();
                self.emit_up(events::SelectionChanged).await?;
            }
            KeyCode::Home => {
                self.autoscroll(false);
                self.first();
                self.emit_up(events::SelectionChanged).await?;
            }
            KeyCode::End => {
                self.autoscroll(true);
                self.last();
                self.emit_up(events::SelectionChanged).await?;
            }
            KeyCode::Esc => {
                self.autoscroll(true);
            }
            KeyCode::Char(' ') if self.markable() => self.mark_item(),
            _ => {}
        }

        Ok(vec![])
    }
}

events_runtime! {
    impl EventsRuntime for StyledList {
        local_events = [KeyEvent];
    }
}

impl Paginator for StyledList {
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
