use std::io::Stdout;
use std::sync::mpsc::Sender;
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
use futures::{FutureExt, select, StreamExt, TryStreamExt};
use tui::backend::{Backend, CrosstermBackend};
use tui::layout::{Constraint, Direction, Layout};
use tui::Terminal;

use crate::ipc::duplex::key_event::KeyEventIncome;
use crate::ipc::duplex::types::{IncomeMessageType};
use crate::types::traits::UIComponent;
use crate::ui::characters_modal::CharactersModal;
use crate::ui::debug_panel::DebugPanel;
use crate::ui::title::Title;
use crate::ui::types::{UIOptions, UIStateFlags};

mod characters_modal;
mod debug_panel;
pub mod types;
mod title;

pub const MARGIN: u16 = 1;
const UI_INPUT_TICK_RATE: u64 = 500;

pub struct UI<'a, B: Backend> {
    _terminal: Terminal<B>,
    _state_flags: UIStateFlags,
    _debug_panel: DebugPanel<'a>,
    _title: Title,
    _characters_modal: CharactersModal<'a>,
}

impl<'a, B: Backend> UI<'a, B> {
    pub fn new(backend: B) -> Self {
        enable_raw_mode().unwrap();
        execute!(std::io::stdout(), EnterAlternateScreen, EnableMouseCapture).unwrap();

        let mut _terminal = Terminal::new(backend).unwrap();
        _terminal.clear().unwrap();
        _terminal.hide_cursor().unwrap();

        Self {
            _terminal,
            _state_flags: UIStateFlags::NONE,

            // components
            _debug_panel: DebugPanel::new(),
            _title: Title::new(),
            _characters_modal: CharactersModal::new(),
        }
    }

    pub fn render(&mut self, options: UIOptions) {
        match options.message {
            IncomeMessageType::Message(output) => {
                self._debug_panel.add_item(output);
            },
            IncomeMessageType::ChooseCharacter(characters) => {
                self._state_flags.set(UIStateFlags::IS_CHARACTERS_MODAL_OPENED, true);
                self._characters_modal.set_items(characters);
            },
            IncomeMessageType::ChooseRealm(_realms) => {
                self._state_flags.set(UIStateFlags::IS_REALM_MODAL_OPENED, true);
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

        }).unwrap();
    }

    fn handle_key_event(&mut self, modifiers: KeyModifiers, key_code: KeyCode) {
        let is_modal_opened = self._state_flags.contains(
            UIStateFlags::IS_CHARACTERS_MODAL_OPENED & UIStateFlags::IS_REALM_MODAL_OPENED
        );

        if is_modal_opened {
            if key_code == KeyCode::Esc {
                self._state_flags.set(UIStateFlags::IS_CHARACTERS_MODAL_OPENED, false);
                self._state_flags.set(UIStateFlags::IS_REALM_MODAL_OPENED, false);
            }
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