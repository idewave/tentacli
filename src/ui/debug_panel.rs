use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Alignment, Rect};
use tui::style::{Color, Style};
use tui::text::{Spans};
use tui::widgets::{Block, Borders, BorderType, Paragraph, Wrap};
use crate::ui::MARGIN;

const PANEL_TITLE: &str = "DEBUG";

pub struct DebugPanel;

impl DebugPanel {
    pub fn render<B: Backend>(frame: &mut Frame<B>, rect: Rect, spans: Vec<Spans>) {
        let block = Block::default()
            .title(PANEL_TITLE)
            .borders(Borders::ALL)
            .border_type(BorderType::Plain);

        let mut offset: usize = 0;
        if spans.len() > rect.height as usize {
            offset = spans.len() - (rect.height - MARGIN * 2) as usize;
        }

        let paragraph = Paragraph::new(spans[offset..].to_vec())
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true })
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .block(block);

        frame.render_widget(paragraph, rect);
    }
}