use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Alignment, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, BorderType, Paragraph, Wrap};

use crate::features::ui::traits::ui_component::{UIComponent};

const APP_NAME: &str = "Idewave TentaCLI";

pub struct Title;

impl UIComponent for Title {
    fn new() -> Self {
        Self
    }

    fn render<B: Backend>(&mut self, frame: &mut Frame<B>, rect: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Double);

        let paragraph = Paragraph::new(APP_NAME)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true })
            .style(Style::default().fg(Color::LightGreen).bg(Color::Black))
            .block(block);

        frame.render_widget(paragraph, rect);
    }
}