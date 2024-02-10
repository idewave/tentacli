use std::sync::{Arc, Mutex as SyncMutex};
use crossterm::event::{KeyCode, KeyModifiers};
use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Alignment, Rect};
use tui::style::{Color, Style};
use tui::text::{Text};
use tui::widgets::{Block, Borders, BorderType, Paragraph, Wrap};

use crate::features::ui::traits::ui_component::{UIComponent};
use crate::features::ui::MARGIN;
use crate::features::ui::types::{UIEventFlags};

const PANEL_TITLE: &str = "DEBUG DETAILS";

pub struct DebugDetailsPanel {
    output: String,
    scroll_offset: u16,
    panel_height: u16,
}

impl DebugDetailsPanel {
    pub fn set_output(&mut self, output: String) -> &mut Self {
        self.scroll_offset = 0;
        self.output = output;
        self
    }

    pub fn handle_key_event(
        &mut self,
        key_modifiers: KeyModifiers,
        key_code: KeyCode,
        _: Arc<SyncMutex<UIEventFlags>>
    ) {
        let text = Text::styled(self.output.clone(), Style::default());
        match key_code {
            KeyCode::Down if key_modifiers.contains(KeyModifiers::CONTROL) => {
                let text_height = text.height();
                let panel_height = self.panel_height as usize;

                if text_height > panel_height &&
                    (self.scroll_offset as usize) < text_height - panel_height {
                    self.scroll_offset += 1;
                }
            },
            KeyCode::Up if key_modifiers.contains(KeyModifiers::CONTROL) => {
                if self.scroll_offset > 0 {
                    self.scroll_offset -= 1;
                }
            },
            _ => {},
        }
    }
}

impl UIComponent for DebugDetailsPanel {
    fn new() -> Self {
        Self {
            output: String::default(),
            scroll_offset: 0,
            panel_height: 0,
        }
    }

    fn render<B: Backend>(&mut self, frame: &mut Frame<B>, rect: Rect) {
        let text = Text::styled(self.output.clone(), Style::default());

        self.panel_height = rect.height - MARGIN * 2;

        let block = Block::default()
            .title(PANEL_TITLE)
            .borders(Borders::ALL)
            .border_type(BorderType::Plain);

        let paragraph = Paragraph::new(text)
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: false })
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .scroll((self.scroll_offset, 0))
            .block(block);

        frame.render_widget(paragraph, rect);
    }
}