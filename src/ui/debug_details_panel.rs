use crossterm::event::{KeyCode, KeyModifiers};
use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Alignment, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, BorderType, Clear, List, ListItem, ListState, Paragraph, Wrap};

use crate::ipc::pipe::types::LoggerOutput;
use crate::types::traits::UIComponent;
use crate::ui::MARGIN;
use crate::ui::types::{UIComponentOptions, UIStateFlags};

const PANEL_TITLE: &str = "DEBUG DETAILS";

pub struct DebugDetailsPanel<'a> {
    items: Vec<Spans<'a>>,
    state: ListState,
}

impl<'a> DebugDetailsPanel<'a> {
    pub fn add_item(&mut self, output: String) -> &mut Self {
        self.items = vec![Spans::from(output)];
        self
    }
}

impl<'a> UIComponent for DebugDetailsPanel<'a> {
    fn new(_: UIComponentOptions) -> Self {
        Self {
            items: vec![],
            state: ListState::default(),
        }
    }

    fn render<B: Backend>(&mut self, frame: &mut Frame<B>, rect: Rect) {
        let block = Block::default()
            .title(PANEL_TITLE)
            .borders(Borders::ALL)
            .border_type(BorderType::Plain);

        let items_amount = self.items.len();

        let mut start_index: usize = 0;
        if items_amount > rect.height as usize {
            start_index = items_amount - (rect.height - MARGIN * 2) as usize;
        }

        let paragraph = Paragraph::new(self.items[start_index..].to_vec())
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true })
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .block(block);

        frame.render_widget(paragraph, rect);
    }
}