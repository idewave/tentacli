use std::sync::{Arc, Mutex as SyncMutex};
use crossterm::event::{KeyCode, KeyModifiers};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{ListItem, ListState};

use crate::primary::client::{Realm};
use crate::features::ui::traits::{UIModalComponent};
use crate::features::ui::types::{UIEventFlags};
use crate::primary::types::HandlerOutput;

#[derive(Clone)]
pub struct RealmModal {
    state: ListState,
    realms: Vec<Realm>,
}

impl RealmModal {
    pub fn set_items(&mut self, realms: Vec<Realm>) -> &mut Self {
        self.realms = realms;
        self
    }

    pub fn get_selected(&mut self) -> Option<Realm> {
        if self.state.selected().is_some() {
            let index = self.state.selected().unwrap();
            let realm = self.realms.remove(index);
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

        let is_modal_opened = { event_flags.lock().unwrap().contains(UIEventFlags::IS_REALM_MODAL_OPENED) };
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
                        output = Some(HandlerOutput::SelectRealm(selected));
                        event_flags.lock().unwrap().set(UIEventFlags::IS_REALM_MODAL_OPENED, false);
                    }
                }
            },
            _ => {},
        };

        output
    }
}

impl UIModalComponent for RealmModal {
    fn new() -> Self {
        Self {
            state: ListState::default(),
            realms: vec![],
        }
    }

    fn get_title() -> String {
        "SELECT REALM".to_string()
    }

    fn get_items_and_state(&mut self) -> (Vec<ListItem>, &mut ListState) {
        let items = self.realms
            .iter()
            .map(|realm| ListItem::new(vec![
                Spans::from(vec![
                    Span::raw("Connect to ["),
                    Span::styled(
                        realm.name.to_string(),
                        Style::default()
                            .fg(Color::LightGreen)
                            .add_modifier(Modifier::BOLD)
                    ),
                    Span::raw("]: "),
                    Span::styled(
                        realm.address.to_string(),
                        Style::default()
                            .fg(Color::LightYellow)
                    ),
                ])
            ]))
            .collect();

        (items, &mut self.state)
    }
}