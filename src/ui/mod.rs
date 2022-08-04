use std::io::Stdout;
use std::process::exit;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use crossterm::{
    event::{
        DisableMouseCapture,
        EnableMouseCapture,
        Event,
        EventStream,
        KeyCode,
    },
    execute,
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen
    }
};
use crossterm::event::KeyModifiers;
use futures::{FutureExt, StreamExt};
use tui::backend::{Backend, CrosstermBackend};
use tui::layout::{Constraint, Direction, Layout};
use tui::Terminal;

mod characters_modal;
mod debug_panel;
mod realm_modal;
pub mod types;
mod title;

use crate::client::Player;
use crate::client::types::ClientFlags;
use crate::ipc::pipe::dialog::DialogOutcome;
use crate::ipc::pipe::key_event::KeyEventIncome;
use crate::ipc::pipe::types::{IncomeMessageType, OutcomeMessageType};
use crate::ipc::session::Session;
use crate::types::traits::UIComponent;
use crate::ui::characters_modal::CharactersModal;
use crate::ui::debug_panel::DebugPanel;
use crate::ui::realm_modal::RealmModal;
use crate::ui::title::Title;
use crate::ui::types::{UIOutputOptions, UIRenderOptions, UIStateFlags};

pub const MARGIN: u16 = 1;
const UI_INPUT_TICK_RATE: u64 = 500;

pub struct UI<'a, B: Backend> {
    _terminal: Terminal<B>,
    _state_flags: UIStateFlags,

    _debug_panel: DebugPanel<'a>,
    _title: Title,
    _characters_modal: CharactersModal<'a>,
    _dialog_outcome: DialogOutcome,
    _realm_modal: RealmModal<'a>,
}

impl<'a, B: Backend> UI<'a, B> {
    pub fn new(backend: B, output_options: UIOutputOptions) -> Self {
        enable_raw_mode().unwrap();
        execute!(std::io::stdout(), EnterAlternateScreen, EnableMouseCapture).unwrap();

        let mut _terminal = Terminal::new(backend).unwrap();
        _terminal.clear().unwrap();
        _terminal.hide_cursor().unwrap();

        Self {
            _terminal,
            _state_flags: UIStateFlags::NONE,
            _dialog_outcome: output_options.dialog_outcome,

            // components
            _debug_panel: DebugPanel::new(),
            _title: Title::new(),
            _characters_modal: CharactersModal::new(),
            _realm_modal: RealmModal::new(),
        }
    }

    pub fn render(&mut self, options: UIRenderOptions) {
        match options.message {
            IncomeMessageType::Message(output) => {
                self._debug_panel.add_item(output);
            },
            IncomeMessageType::ChooseCharacter(characters) => {
                self._state_flags.set(UIStateFlags::IS_CHARACTERS_MODAL_OPENED, true);
                self._characters_modal.set_items(characters);
            },
            IncomeMessageType::ChooseRealm(realms) => {
                self._state_flags.set(UIStateFlags::IS_REALM_MODAL_OPENED, true);
                self._realm_modal.set_items(realms);
            },
            IncomeMessageType::KeyEvent(modifiers, key_code) => {
                self.handle_key_event(modifiers, key_code);
            },
        }

        self._terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(MARGIN)
                .constraints([
                    Constraint::Percentage(12),
                    Constraint::Percentage(76),
                    Constraint::Percentage(12),
                ])
                .split(frame.size());

            let output_panels = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(100),
                ])
                .split(chunks[1]);

            self._title.render(frame, chunks[0]);
            self._debug_panel.render(frame, output_panels[0]);

            if self._state_flags.contains(UIStateFlags::IS_CHARACTERS_MODAL_OPENED) {
                self._characters_modal.render(frame, chunks[1])
            }

            if self._state_flags.contains(UIStateFlags::IS_REALM_MODAL_OPENED) {
                self._realm_modal.render(frame, chunks[1])
            }

        }).unwrap();
    }

    fn handle_key_event(&mut self, modifiers: KeyModifiers, key_code: KeyCode) {
        match key_code {
            KeyCode::Up => {
                if self._state_flags.contains(UIStateFlags::IS_CHARACTERS_MODAL_OPENED) {
                    self._characters_modal.prev();
                }

                if self._state_flags.contains(UIStateFlags::IS_REALM_MODAL_OPENED) {
                    self._realm_modal.prev();
                }
            },
            KeyCode::Down => {
                if self._state_flags.contains(UIStateFlags::IS_CHARACTERS_MODAL_OPENED) {
                    self._characters_modal.next();
                }

                if self._state_flags.contains(UIStateFlags::IS_REALM_MODAL_OPENED) {
                    self._realm_modal.next();
                }
            },
            KeyCode::Enter => {
                if self._state_flags.contains(UIStateFlags::IS_CHARACTERS_MODAL_OPENED) {
                    self._state_flags.set(UIStateFlags::IS_CHARACTERS_MODAL_OPENED, false);
                    self._dialog_outcome.send_selected_character(
                        self._characters_modal.get_selected()
                    );
                }
                if self._state_flags.contains(UIStateFlags::IS_REALM_MODAL_OPENED) {
                    self._state_flags.set(UIStateFlags::IS_REALM_MODAL_OPENED, false);
                    self._dialog_outcome.send_selected_realm(self._realm_modal.get_selected());
                }
            },
            KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => {
                // TODO: probably need exit from app in different way
                exit(0);
            },
            _ => {},
        }
    }
}

pub struct UIInput {
    _key_event_income: KeyEventIncome,
    _event_stream: EventStream,
}

impl UIInput {
    pub fn new(key_event_income: KeyEventIncome) -> Self {
        let event_stream = EventStream::new();

        Self {
            _key_event_income: key_event_income,
            _event_stream: event_stream,
        }
    }

    pub async fn handle(&mut self) {
        match self._event_stream.next().fuse().await {
            Some(Ok(event)) => {
                if let Event::Key(key) = event {
                    self._key_event_income.send_key_event(key);
                }
            },
            _ => {},
        }
    }
}

pub struct UIOutput<'a> {
    session: Arc<Mutex<Session>>,
    client_flags: &'a mut ClientFlags,
}

impl<'a> UIOutput<'a> {
    pub fn new(session: Arc<Mutex<Session>>, client_flags: &'a mut ClientFlags) -> Self {
        Self {
            session,
            client_flags,
        }
    }

    pub fn handle(&mut self, message: OutcomeMessageType) {
        match message {
            OutcomeMessageType::CharacterSelected(character) => {
                self.session.lock().unwrap().me = Some(Player::from(character));
                self.client_flags.set(ClientFlags::IN_FROZEN_MODE, false);
            },
            OutcomeMessageType::RealmSelected(realm) => {
                self.session.lock().unwrap().selected_realm = Some(realm);
                self.client_flags.set(ClientFlags::IN_FROZEN_MODE, false);
            },
        };
    }
}