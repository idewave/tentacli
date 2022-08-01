use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Alignment, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, BorderType, List, ListItem};

use crate::client::Character;
use crate::types::traits::UIComponent;
use crate::ui::MARGIN;

const PANEL_TITLE: &str = "CHARACTERS LIST";

pub struct CharactersModal<'a> {
    items: Vec<ListItem<'a>>
}

impl<'a> CharactersModal<'a> {
    pub fn set_items(&mut self, characters: Vec<Character>) -> &mut Self {
        self.items = characters
            .iter()
            .map(|character| ListItem::new(character.name.to_string()))
            .collect();

        self
    }

    pub fn drop_items(&mut self) {
        self.items = vec![];
    }
}

impl<'a> UIComponent for CharactersModal<'a> {
    fn new() -> Self {
        Self {
            items: vec![],
        }
    }

    fn render<B: Backend>(&mut self, frame: &mut Frame<B>, rect: Rect) {
        let block = Block::default()
            .title(PANEL_TITLE)
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Plain);

        let list = List::new(self.items.to_vec())
            .block(block)
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">> ");

        frame.render_widget(list, rect);
    }
}