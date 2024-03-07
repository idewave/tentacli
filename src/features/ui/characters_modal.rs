use std::sync::{Arc, Mutex as SyncMutex};
use crossterm::event::{KeyCode, KeyModifiers};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{ListItem, ListState};

use crate::primary::client::{Player};
use crate::features::ui::traits::{UIModalComponent};
use crate::features::ui::types::{UIEventFlags};
use crate::primary::types::HandlerOutput;

pub struct CharactersModal {
    state: ListState,
    characters: Vec<Player>,
}

impl CharactersModal {
    pub fn set_items(&mut self, characters: Vec<Player>) -> &mut Self {
        self.characters = characters;
        self
    }

    pub fn get_selected(&mut self) -> Option<Player> {
        if self.state.selected().is_some() {
            let index = self.state.selected().unwrap();
            let realm = self.characters.remove(index);
            Some(realm)
        } else {
            None
        }
    }

    pub fn handle_key_event(
        &mut self,
        _: KeyModifiers,
        key_code: KeyCode,
        event_flags: Arc<SyncMutex<UIEventFlags>>
    ) -> Option<HandlerOutput> {
        let mut output = None;

        let is_modal_opened = { event_flags.lock().unwrap().contains(UIEventFlags::IS_CHARACTERS_MODAL_OPENED) };
        match key_code {
            KeyCode::Up => {
                if is_modal_opened {
                    self.prev();
                    event_flags.lock().unwrap().set(UIEventFlags::IS_EVENT_HANDLED, true);
                }
            },
            KeyCode::Down => {
                if is_modal_opened {
                    self.next();
                    event_flags.lock().unwrap().set(UIEventFlags::IS_EVENT_HANDLED, true);
                }
            },
            KeyCode::Enter => {
                if is_modal_opened {
                    if let Some(selected) = self.get_selected() {
                        output = Some(HandlerOutput::SelectCharacter(selected));
                        event_flags.lock().unwrap().set(UIEventFlags::IS_CHARACTERS_MODAL_OPENED, false);
                    }
                }
            },
            _ => {},
        };

        output
    }
}

impl UIModalComponent for CharactersModal {
    fn new() -> Self {
        Self {
            state: ListState::default(),
            characters: vec![],
        }
    }

    fn get_title() -> String {
        "SELECT CHARACTER".to_string()
    }

    fn get_items_and_state(&mut self) -> (Vec<ListItem>, &mut ListState) {
        let items = self.characters
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

        (items, &mut self.state)
    }
}