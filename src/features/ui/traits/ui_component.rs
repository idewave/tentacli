use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Alignment, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, BorderType, Clear, List, ListItem, ListState};

pub trait UIComponent {
    fn new() -> Self where Self: Sized;
    fn render<B: Backend>(&mut self, frame: &mut Frame<B>, rect: Rect);
}

pub trait UIModalComponent {
    fn new() -> Self where Self: Sized;

    fn get_title() -> String;

    fn get_items_and_state(&mut self) -> (Vec<ListItem>, &mut ListState);

    fn prev(&mut self) {
        let (items, state) = self.get_items_and_state();

        let i = match state.selected() {
            Some(i) => if i == 0 { items.len() - 1 } else { i - 1 },
            None => 0,
        };

        state.select(Some(i));
    }

    fn next(&mut self) {
        let (items, state) = self.get_items_and_state();

        let i = match state.selected() {
            Some(i) => if i >= items.len() - 1 { 0 } else { i + 1 },
            None => 0,
        };

        state.select(Some(i));
    }

    fn render<B: Backend>(&mut self, frame: &mut Frame<B>, rect: Rect) {
        let (items, mut state) = self.get_items_and_state();

        let block = Block::default()
            .title(Self::get_title())
            .title_alignment(Alignment::Center)
            .style(Style::default().bg(Color::Black))
            .borders(Borders::ALL)
            .border_type(BorderType::Plain);

        let list = List::new(items)
            .block(block)
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">> ");

        frame.render_widget(Clear, rect);
        frame.render_stateful_widget(list, rect, &mut state);
    }
}