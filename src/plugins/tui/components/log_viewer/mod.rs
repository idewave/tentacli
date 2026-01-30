use std::sync::{Arc};
use async_event_emitter::AsyncEventEmitter;
use chrono::Local;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;
use ratatui::layout::{Margin, Rect};
use ratatui::prelude::{Color, Line, Modifier, Span, Style};
use ratatui::style::Stylize;
use ratatui::symbols::border;
use ratatui::widgets::{Block, ListItem, Paragraph};

pub mod events;

use crate::events_runtime;
use crate::client::{Echo, MsgType, PacketType};
use crate::plugins::tui::components::app::events::{OutputEvent};
use crate::plugins::tui::components::app::{OutputItem};
use crate::plugins::tui::components::list::{self, ItemType, StyledList};
use crate::plugins::tui::events::traits::{
    EventHandler, EventSystem, EventsRuntime, WithEventSystem
};
use crate::plugins::tui::layout::{split_vertical, Values};
use crate::plugins::tui::theme::{KEY_BTN_BG, KEY_BTN_FG, TITLE_BG, TITLE_FG};
use crate::plugins::tui::traits::{Focusable, UIComponent};

const STATS_BLOCK_HEIGHT: u16 = 3;

pub struct LogViewer {
    list: StyledList,
    event_system: EventSystem,
}

impl Default for LogViewer {
    fn default() -> Self {
        let mut list = StyledList::default();
        list.autoscroll(true);

        Self {
            list,
            event_system: EventSystem::default(),
        }
    }
}

impl LogViewer {
    fn format_count(&self, key: ItemType) -> String {
        let value = *self.list.counter.get(&key).unwrap_or(&0);
        match value {
            1_000_000_000.. => format!("{:.1}B", value as f64 / 1_000_000_000.0),
            1_000_000.. => format!("{:.1}M", value as f64 / 1_000_000.0),
            1_000.. => format!("{:.1}k", value as f64 / 1_000.0),
            _ => value.to_string(),
        }
    }

    fn make_item(item_type: ItemType, text: String) -> ListItem<'static> {
        let text = match item_type {
            ItemType::Message(MsgType::Success) => format!("[DONE]: {text}"),
            ItemType::Message(MsgType::Error) => format!("[FAIL]: {text}"),
            ItemType::Message(MsgType::Info) => format!("[INFO]: {text}"),
            ItemType::Packet(PacketType::Incoming) => format!("[RECV]: {text}"),
            ItemType::Packet(PacketType::Outgoing) => format!("[SEND]: {text}"),
        };

        let local_time = Local::now().format("[%H:%M:%S]").to_string();
        let item = ListItem::new(Line::from(vec![
            Span::styled(
                format!("{local_time}"),
                Style::new().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                text,
                Style::new()
                    .fg(match item_type {
                        ItemType::Message(MsgType::Success) => Color::Green,
                        ItemType::Message(MsgType::Error) => Color::Red,
                        ItemType::Message(MsgType::Info) => Color::Gray,
                        ItemType::Packet(PacketType::Incoming) => Color::Magenta,
                        ItemType::Packet(PacketType::Outgoing) => Color::LightBlue,
                    })
                    .add_modifier(Modifier::BOLD),
            ),
        ]));

        item
    }
}

impl UIComponent for LogViewer {
    fn render(&mut self, frame: &mut Frame, rect: Rect) {
        let [top, bottom] = split_vertical(
            rect,
            Values::Mins([rect.height.saturating_sub(STATS_BLOCK_HEIGHT), STATS_BLOCK_HEIGHT]),
        );

        let title = Line::from(
            Span::raw("LOG OUTPUT").fg(TITLE_FG).bg(TITLE_BG).add_modifier(Modifier::BOLD)
        );

        let instructions = Line::from(vec![
            Span::styled(
                "To navigate use ",
                Style::new().add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "<ArrowDown/Up>",
                Style::new().fg(KEY_BTN_FG).bg(KEY_BTN_BG).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                " and ",
                Style::new().add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "<Home/End>",
                Style::new().fg(KEY_BTN_FG).bg(KEY_BTN_BG).add_modifier(Modifier::BOLD),
            ),
        ]);

        let block = Block::bordered()
            .title(title.right_aligned())
            .title_bottom(instructions.centered())
            .border_set(border::ROUNDED);

        frame.render_widget(block, top);

        self.list.render(frame, top.inner(Margin { horizontal: 1, vertical: 1 }));

        let title = Line::from(
            Span::raw("STATS").fg(TITLE_FG).bg(TITLE_BG).add_modifier(Modifier::BOLD)
        );

        let block = Block::bordered()
            .title(title.right_aligned())
            .border_set(border::ROUNDED);

        let (done, fail, debg, recv, sent) = (
            self.format_count(ItemType::Message(MsgType::Success)),
            self.format_count(ItemType::Message(MsgType::Error)),
            self.format_count(ItemType::Message(MsgType::Info)),
            self.format_count(ItemType::Packet(PacketType::Incoming)),
            self.format_count(ItemType::Packet(PacketType::Outgoing)),
        );

        let paragraph = Paragraph::new(Line::from(vec![
            Span::styled(format!("[DONE]:{done} "), Style::new().fg(Color::Green)),
            Span::styled(format!("[FAIL]:{fail} "), Style::new().fg(Color::Red)),
            Span::styled(format!("[INFO]:{debg} "), Style::new().fg(Color::Gray)),
            Span::styled(format!("[RECV]:{recv} "), Style::new().fg(Color::Magenta)),
            Span::styled(format!("[SENT]:{sent}"), Style::new().fg(Color::LightBlue)),
        ])).left_aligned().block(block);

        frame.render_widget(paragraph, bottom);
    }
}

impl Focusable for LogViewer {}

impl WithEventSystem for LogViewer {
    fn event_system(&mut self) -> &mut EventSystem {
        &mut self.event_system
    }
}

impl EventHandler<OutputEvent> for LogViewer {
    async fn callback(&mut self, event: OutputEvent) -> anyhow::Result<Vec<Echo>> {
        for item in event.items {
            match item {
                OutputItem::Message(message) => {
                    let item = Self::make_item(
                        ItemType::Message(message.msg_type),
                        message.text,
                    );
                    self.list.add_item(ItemType::Message(message.msg_type), item).await?;
                }

                OutputItem::Packet(packet) => {
                    let item = Self::make_item(
                        ItemType::Packet(packet.metadata.packet_type),
                        packet.metadata.packet_name,
                    );
                    self.list.add_item(ItemType::Packet(packet.metadata.packet_type), item).await?;
                }
            }
        }

        Ok(vec![])
    }
}

impl EventHandler<KeyEvent> for LogViewer {
    async fn callback(&mut self, event: KeyEvent) -> anyhow::Result<Vec<Echo>> {
        if event.code == KeyCode::Right {
            self.list.autoscroll(false);
        } else {
            self.list.emit_down(event).await?;
        }

        Ok(vec![])
    }
}

impl EventHandler<list::events::SelectionChanged> for LogViewer {
    async fn callback(&mut self, _: list::events::SelectionChanged) -> anyhow::Result<Vec<Echo>> {
        // get actual list state and emit root event
        if let Some(indices) = self.list.selected() {
            self.emit_up(events::SelectedIndex(indices[0])).await?;
        }

        Ok(vec![])
    }
}

events_runtime! {
    impl EventsRuntime for LogViewer {
        local_events = [list::events::SelectionChanged, KeyEvent];
        parent_events = [KeyEvent, OutputEvent];
        children = [list];
    }
}