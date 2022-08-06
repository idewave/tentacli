use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Alignment, Rect};
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, BorderType, Paragraph, Wrap};

use crate::ipc::pipe::types::LoggerOutput;
use crate::types::traits::UIComponent;
use crate::ui::MARGIN;
use crate::ui::types::{UIComponentOptions};

const PANEL_TITLE: &str = "DEBUG";

pub struct DebugPanel<'a> {
    items: Vec<Spans<'a>>,
    debug_mode: bool,
}

impl<'a> DebugPanel<'a> {
    pub fn set_mode(&mut self, debug_mode: bool) -> &mut Self {
        self.debug_mode = debug_mode;
        self
    }

    pub fn add_item(&mut self, output: LoggerOutput) -> &mut Self {
        let message = self.generate_message(output);

        if !message.content.is_empty() {
            self.items.push(Spans::from(message));
        }

        self
    }

    fn generate_message(&mut self, output: LoggerOutput) -> Span<'a> {
        match output {
            LoggerOutput::Debug(data) if !data.is_empty() && self.debug_mode => Span::styled(
                format!("[DEBUG]: {}\n", data), Style::default().fg(Color::DarkGray)
            ),
            LoggerOutput::Error(data) if !data.is_empty() => Span::styled(
                format!("[ERROR]: {}\n", data), Style::default().fg(Color::Red)
            ),
            LoggerOutput::Success(data) if !data.is_empty() => Span::styled(
                format!("[SUCCESS]: {}\n", data), Style::default().fg(Color::LightGreen)
            ),
            LoggerOutput::Server(data) if !data.is_empty() => Span::styled(
                format!("[SERVER]: {}\n", data), Style::default().fg(Color::LightMagenta)
            ),
            LoggerOutput::Client(data) if !data.is_empty() => Span::styled(
                format!("[CLIENT]: {}\n", data), Style::default().fg(Color::LightBlue)
            ),
            _ => Span::raw(""),
        }
    }
}

impl<'a> UIComponent for DebugPanel<'a> {
    fn new(_: UIComponentOptions) -> Self {
        Self {
            items: vec![],
            debug_mode: false,
        }
    }

    fn render<B: Backend>(&mut self, frame: &mut Frame<B>, rect: Rect) {
        let block = Block::default()
            .title(PANEL_TITLE)
            .borders(Borders::ALL)
            .border_type(BorderType::Plain);

        let mut offset: usize = 0;
        if self.items.len() > rect.height as usize {
            offset = self.items.len() - (rect.height - MARGIN * 2) as usize;
        }

        let paragraph = Paragraph::new(self.items[offset..].to_vec())
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true })
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .block(block);

        frame.render_widget(paragraph, rect);
    }
}