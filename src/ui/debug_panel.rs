use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Alignment, Rect};
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, BorderType, Paragraph, Wrap};

use crate::ipc::pipe::types::LoggerOutput;
use crate::types::traits::UIComponent;
use crate::ui::MARGIN;

const PANEL_TITLE: &str = "DEBUG";

pub struct DebugPanel<'a> {
    items: Vec<Spans<'a>>,
}

impl<'a> DebugPanel<'a> {
    pub fn add_item(&mut self, output: LoggerOutput) -> &mut Self {
        let message = Self::generate_message(output);

        if !message.content.is_empty() {
            self.items.push(Spans::from(message));
        }

        self
    }

    fn generate_message(output: LoggerOutput) -> Span<'a> {
        match output {
            LoggerOutput::Info(data) if !data.is_empty() => Span::styled(
                format!("[INFO]: {}\n", data), Style::default().fg(Color::Gray)
            ),
            LoggerOutput::Debug(data) if !data.is_empty() => Span::styled(
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
    fn new() -> Self {
        Self {
            items: vec![],
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