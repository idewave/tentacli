use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Alignment, Rect};
use tui::style::{Color, Style};
use tui::text::{Spans, Text};
use tui::widgets::{Block, Borders, BorderType, Paragraph, Wrap};

use crate::traits::ui_component::UIComponent;
use crate::ui::MARGIN;
use crate::ui::types::{UIComponentOptions};

const PANEL_TITLE: &str = "DEBUG DETAILS";

pub struct DebugDetailsPanel<'a> {
    text: Text<'a>,
}

impl<'a> DebugDetailsPanel<'a> {
    pub fn add_item(&mut self, output: String) -> &mut Self {
        self.text = Text::styled(output, Style::default());
        self
    }
}

impl<'a> UIComponent for DebugDetailsPanel<'a> {
    fn new(_: UIComponentOptions) -> Self {
        Self {
            text: Text::default(),
        }
    }

    fn render<B: Backend>(&mut self, frame: &mut Frame<B>, rect: Rect) {
        let block = Block::default()
            .title(PANEL_TITLE)
            .borders(Borders::ALL)
            .border_type(BorderType::Plain);

        let paragraph = Paragraph::new(self.text.clone())
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: false })
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .block(block);

        frame.render_widget(paragraph, rect);
    }
}