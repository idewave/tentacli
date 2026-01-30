use std::collections::HashMap;
use std::sync::{Arc};
use std::time::Duration;
use async_event_emitter::AsyncEventEmitter;
use crossterm::event::{Event, EventStream, KeyCode, KeyEvent};
use futures::{FutureExt, StreamExt};
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Margin, Rect};
use ratatui::prelude::{Color, Line, Modifier, Span};
use ratatui::style::{Style, Stylize};
use ratatui::symbols::border;
use ratatui::widgets::{Block, Clear, Paragraph};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::Sender;
use tokio_util::sync::CancellationToken;

pub mod events;

use crate::events_runtime;
use crate::client::prelude::*;
use crate::plugins::tui::components::app::events::{ChoicesEvent, OutputEvent};
use crate::plugins::tui::components::json_navigator::{self, JsonNavigator};
use crate::plugins::tui::components::log_viewer::{self, LogViewer};
use crate::plugins::tui::components::modal::Modal;
use crate::plugins::tui::events::traits::{
    EventHandler, EventSystem, EventsRuntime, WithEventSystem
};
use crate::plugins::tui::layout::{center, split_horizontal, Values};
use crate::plugins::tui::theme::{APP_TITLE_FG, BLACK_BG};
use crate::plugins::tui::traits::{Focusable, UIComponent};

#[derive(Default)]
pub struct App {
    app_name: String,
    event_system: EventSystem,
    output_items: Vec<OutputItem>,
    echo_senders: Arc<HashMap<ServerLabel, Sender<Echo>>>,
    shutdown: CancellationToken,

    // app components
    modal: Modal,
    log_viewer: LogViewer,
    json_navigator: JsonNavigator,
}

impl App {
    pub fn new(app_name: &str) -> Self {
        Self {
            app_name: app_name.to_string(),
            ..Default::default()
        }
    }

    pub fn handle_run(
        broadcast_rx: async_broadcast::Receiver<OrderedOutput>,
        echo_senders: Arc<HashMap<ServerLabel, Sender<Echo>>>,
        shutdown: CancellationToken,
    ) -> Task {
        tokio::spawn(async move {
            let mut app = App::new("TENTACLI");
            app.echo_senders = echo_senders.clone();
            app.shutdown = shutdown.clone();

            let mut ordered_rx = OrderedReceiver::new(broadcast_rx);
            let emitter = Arc::new(AsyncEventEmitter::new());

            app.event_system = EventSystem::new(emitter.clone());
            app.register_all(emitter.clone()).await?;

            let mut reader = EventStream::new();
            // RAII guard: ensures the terminal is restored on any exit
            let _guard = TerminalGuard;
            let mut terminal = ratatui::init();
            let mut ticker = tokio::time::interval(Duration::from_millis(25));
            let mut target: ServerLabel = "";
            let mut next_event = reader.next().fuse();

            loop {
                let mut all_echoes: Vec<Echo> = app.try_update_all().await?;

                tokio::select! {
                    biased;

                    _ = shutdown.cancelled() => {
                        ratatui::restore();
                        break;
                    },

                    Some(Ok(event)) = &mut next_event => {
                        next_event = reader.next().fuse();

                        if let Event::Key(event) = event {
                            if event.code == KeyCode::Char('q') {
                                app.emit_down(event).await?;
                                continue;
                            }

                            if app.modal.holds_focus() {
                                app.modal.emit_down(event).await?;
                                continue;
                            }

                            if app.json_navigator.holds_focus() {
                                app.json_navigator.emit_down(event).await?;
                                continue;
                            }

                            if app.log_viewer.holds_focus() {
                                if event.code == KeyCode::Right {
                                    app.json_navigator.on_focus();
                                }

                                app.log_viewer.emit_down(event).await?;
                                continue;
                            }
                        }
                    },
                    Ok(OrderedOutput { label, outputs, .. }) = ordered_rx.recv() => {
                        target = label;

                        let mut output_items = vec![];

                        for output in &*outputs {
                            match output {
                                HandlerOutput::Messages(messages) => {
                                    output_items.extend(
                                        messages.iter().map(|msg| OutputItem::Message(msg.clone()))
                                    );
                                },
                                HandlerOutput::Packets(packets) => {
                                    output_items.extend(
                                        packets.iter().map(|pkt| OutputItem::Packet(pkt.clone()))
                                    );
                                },
                                HandlerOutput::Requests(requests) => {
                                    for req in requests {
                                        if let Request::InitChoice(items, _) = &req {
                                            app.emit_down(ChoicesEvent(items.clone())).await?;
                                        }
                                    }
                                },
                            }
                        }

                        // keep the output order according as expected
                        if !output_items.is_empty() {
                            app.output_items.extend_from_slice(&output_items);
                            app.emit_down(OutputEvent { items: output_items }).await?;
                        }
                    },
                    _ = ticker.tick() => {
                        terminal.draw(|frame| {
                            app.render(frame, frame.area());
                        })?;
                    },
                }

                if !all_echoes.is_empty()
                    && let Some(sender) = echo_senders.get(target)
                {
                    for echo in all_echoes.drain(..) {
                        sender.send(echo).await?;
                    }
                }
            }

            Ok(())
        })
    }
}

impl WithEventSystem for App {
    fn event_system(&mut self) -> &mut EventSystem {
        &mut self.event_system
    }
}

impl EventHandler<log_viewer::events::SelectedIndex> for App {
    async fn callback(
        &mut self,
        event: log_viewer::events::SelectedIndex
    ) -> anyhow::Result<Vec<Echo>> {
        let index = event.0;
        if let Some(item) = self.output_items.get(index) {
            self.emit_down(json_navigator::events::SetItem(item.clone())).await?;
        }

        Ok(vec![])
    }
}

impl EventHandler<KeyEvent> for App {
    async fn callback(&mut self, event: KeyEvent) -> anyhow::Result<Vec<Echo>> {
        if let KeyCode::Char('q') = event.code {
            for sender in self.echo_senders.values() {
                let _ = sender.send(Echo::Drop).await;
            }

            self.shutdown.cancel();
        }

        Ok(vec![])
    }
}

events_runtime! {
    impl EventsRuntime for App {
        local_events = [log_viewer::events::SelectedIndex, KeyEvent];
        children = [modal, log_viewer, json_navigator];
    }
}

impl UIComponent for App {
    fn render(&mut self, frame: &mut Frame, rect: Rect) {
        frame.render_widget(Clear, rect);
        frame.render_widget(
            Block::default().style(Style::default().bg(BLACK_BG)),
            rect,
        );
        // --------LOG OUTPUT------------------------------------------
        // |                  |                   |                   |
        // |                  |                   |                   |
        // |                  |                   |                   |
        // |                  |                   |                   |
        // |                  |                   |                   |
        // |------------STATS-|                   |                   |
        // |                  |                   |                   |
        // |                  |                   |                   |
        // |                  |                   |                   |
        // ------------------------------------------------------------

        let [left_column, right_column] = split_horizontal(
            rect.inner(Margin { horizontal: 1, vertical: 1 }),
            Values::Percentages([40, 60]),
        );

        let info = Line::from(vec![
            Span::raw(format!("{} v{}", self.app_name, env!("CARGO_PKG_VERSION")))
                .fg(APP_TITLE_FG)
                .add_modifier(Modifier::BOLD),
        ]);

        let block = Block::bordered()
            .title_bottom(info.right_aligned())
            .border_set(border::EMPTY);

        frame.render_widget(block, rect);

        let hint_area = Rect {
            x: rect.x + 2,
            y: rect.y + rect.height.saturating_sub(1),
            width: rect.width.saturating_sub(2),
            height: 1,
        };

        let hint = Paragraph::new(Line::from(vec![
            Span::raw("q: quit").fg(Color::LightBlue).add_modifier(Modifier::BOLD),
        ]))
            .alignment(Alignment::Left);

        frame.render_widget(hint, hint_area);

        let modal_layout = center(rect, Constraint::Percentage(70), Constraint::Percentage(70));

        self.log_viewer.render(frame, left_column);
        self.json_navigator.render(frame, right_column);

        // modal on top
        self.modal.render(frame, modal_layout);
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum OutputItem {
    Message(Message),
    Packet(Packet),
}

struct TerminalGuard;

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        ratatui::restore();
    }
}
