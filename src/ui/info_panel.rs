use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Alignment, Rect};
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, BorderType, Clear, Paragraph, Wrap};

use crate::traits::ui_component::UIComponent;
use crate::ui::types::{UIComponentOptions};

pub struct InfoPanel {
    selected_index: usize,
    total_items: usize,
    total_income: usize,
    total_outcome: usize,
    debug_mode: bool,
}

impl InfoPanel {
    pub fn set_selected_index(&mut self, selected_index: usize) -> &mut Self {
        self.selected_index = selected_index;
        self
    }

    pub fn set_total_items(&mut self, total_items: usize) -> &mut Self {
        self.total_items = total_items;
        self
    }

    pub fn set_total_income(&mut self, total_income: usize) -> &mut Self {
        self.total_income = total_income;
        self
    }

    pub fn set_total_outcome(&mut self, total_outcome: usize) -> &mut Self {
        self.total_outcome = total_outcome;
        self
    }

    pub fn set_debug_mode(&mut self, debug_mode: bool) -> &mut Self {
        self.debug_mode = debug_mode;
        self
    }
}

impl UIComponent for InfoPanel {
    fn new(_: UIComponentOptions) -> Self {
        Self {
            selected_index: 0,
            total_items: 0,
            total_income: 0,
            total_outcome: 0,
            debug_mode: false,
        }
    }

    fn render<B: Backend>(&mut self, frame: &mut Frame<B>, rect: Rect) {
        let mut spans = vec![
            Span::styled(
                format!("↓{}", self.total_income),
                Style::default().fg(Color::LightMagenta)
            ),
            Span::raw(" "),
            Span::styled(
                format!("↑{}", self.total_outcome),
                Style::default().fg(Color::LightBlue)
            ),
        ];

        if self.debug_mode {
            spans.push(Span::styled(
                format!(" [DEBUG] {}/{}", self.selected_index + 1, self.total_items),
                Style::default().fg(Color::Gray)
            ));
        }

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Plain);

        let paragraph = Paragraph::new(Spans::from(spans))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: false })
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .block(block);

        frame.render_widget(Clear, rect);
        frame.render_widget(paragraph, rect);
    }
}