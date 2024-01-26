use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Alignment, Rect};
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, BorderType, Clear, Paragraph, Wrap};

use crate::features::ui::traits::ui_component::{UIComponent};

pub struct InfoPanel {
    selected_index: usize,
    total_items: usize,
    total_response_amount: usize,
    total_request_amount: usize,
    total_errors_amount: usize,
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

    pub fn set_total_response_amount(&mut self, total_income: usize) -> &mut Self {
        self.total_response_amount = total_income;
        self
    }

    pub fn set_total_request_amount(&mut self, total_outcome: usize) -> &mut Self {
        self.total_request_amount = total_outcome;
        self
    }

    pub fn set_total_errors_amount(&mut self, total_errors: usize) -> &mut Self {
        self.total_errors_amount = total_errors;
        self
    }
}

impl UIComponent for InfoPanel {
    fn new() -> Self {
        Self {
            selected_index: 0,
            total_items: 0,
            total_response_amount: 0,
            total_request_amount: 0,
            total_errors_amount: 0,
        }
    }

    fn render<B: Backend>(&mut self, frame: &mut Frame<B>, rect: Rect) {
        let mut spans = vec![
            Span::styled(
                format!("[CURRENT {}/{}]", self.selected_index + 1, self.total_items),
                Style::default().fg(Color::Gray)
            ),
            Span::raw(" "),
            Span::styled(
                format!("[IN {}]", self.total_response_amount),
                Style::default().fg(Color::LightMagenta)
            ),
            Span::raw(" "),
            Span::styled(
                format!("[OUT {}]", self.total_request_amount),
                Style::default().fg(Color::LightBlue)
            ),
        ];

        if self.total_errors_amount > 0 {
            spans.extend(
                vec![
                    Span::raw(" "),
                    Span::styled(
                        format!("[ERR {}]", self.total_errors_amount),
                        Style::default().fg(Color::Red)
                    )
                ]
            );
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