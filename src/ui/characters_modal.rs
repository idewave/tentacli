use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Alignment, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, BorderType, Clear, List, ListItem, ListState};

use crate::client::Character;
use crate::types::traits::UIComponent;
use crate::ui::MARGIN;

const PANEL_TITLE: &str = "SELECT CHARACTER";

pub struct CharactersModal<'a> {
    items: Vec<ListItem<'a>>,
    state: ListState,
    characters: Vec<Character>,
}

impl<'a> CharactersModal<'a> {
    pub fn set_items(&mut self, characters: Vec<Character>) -> &mut Self {
        self.items = characters
            .iter()
            .map(|character| ListItem::new(vec![
                Spans::from(vec![
                    Span::raw("name: "),
                    Span::styled(
                        character.name.to_string(),
                        Style::default()
                            .fg(Color::LightGreen)
                            .add_modifier(Modifier::BOLD)
                    ),
                    Span::raw(format!(", guid: {:?}, ", character.guid)),
                    Span::styled(
                        format!("{} lvl", character.level),
                        Style::default()
                            .fg(Color::LightYellow)
                    ),
                ])
            ]))
            .collect();

        self.characters = characters;

        self
    }

    pub fn prev(&mut self) {
        let i = match self.state.selected() {
            Some(i) => if i == 0 { self.items.len() - 1 } else { i - 1 },
            None => 0,
        };

        self.state.select(Some(i));
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => if i >= self.items.len() - 1 { 0 } else { i + 1 },
            None => 0,
        };

        self.state.select(Some(i));
    }

    pub fn get_selected(&mut self) -> Character {
        let index = match self.state.selected() {
            Some(i) => i,
            None => 0,
        };

        self.characters.remove(index)
    }
}

impl<'a> UIComponent for CharactersModal<'a> {
    fn new() -> Self {
        Self {
            items: vec![],
            state: ListState::default(),
            characters: vec![],
        }
    }

    fn render<B: Backend>(&mut self, frame: &mut Frame<B>, rect: Rect) {
        let block = Block::default()
            .title(PANEL_TITLE)
            .title_alignment(Alignment::Center)
            .style(Style::default().bg(Color::Black))
            .borders(Borders::ALL)
            .border_type(BorderType::Plain);

        let list = List::new(self.items.to_vec())
            .block(block)
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">> ");

        frame.render_widget(Clear, rect);
        frame.render_stateful_widget(list, rect, &mut self.state);
    }
}