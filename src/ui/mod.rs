use std::process::exit;
use std::sync::{Arc, Mutex};
use crossterm::{
    event::{
        DisableMouseCapture,
        EnableMouseCapture,
        Event,
        KeyCode,
        KeyModifiers,
        read
    },
    execute,
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen
    }
};
use tui::backend::{Backend};
use tui::layout::{Constraint, Direction, Layout};
use tui::Terminal;

mod characters_modal;
mod debug_panel;
mod mode_panel;
mod realm_modal;
pub mod types;
mod title;

use crate::client::Player;
use crate::client::types::ClientFlags;
use crate::ipc::pipe::dialog::DialogOutcome;
use crate::ipc::pipe::flag::FlagOutcome;
use crate::ipc::pipe::event::EventIncome;
use crate::ipc::pipe::types::{IncomeMessageType, OutcomeMessageType};
use crate::ipc::session::Session;
use crate::types::traits::UIComponent;
use crate::ui::characters_modal::CharactersModal;
use crate::ui::debug_panel::DebugPanel;
use crate::ui::mode_panel::ModePanel;
use crate::ui::realm_modal::RealmModal;
use crate::ui::title::Title;
use crate::ui::types::{
    UIComponentOptions,
    UIModeFlags,
    UIOutputOptions,
    UIRenderOptions,
    UIStateFlags
};

pub const MARGIN: u16 = 1;

pub struct UI<'a, B: Backend> {
    state_flags: UIStateFlags,

    _dialog_outcome: DialogOutcome,
    _flag_outcome: FlagOutcome,
    _terminal: Terminal<B>,

    _characters_modal: CharactersModal<'a>,
    _debug_panel: DebugPanel<'a>,
    _mode_panel: ModePanel,
    _realm_modal: RealmModal<'a>,
    _title: Title,
}

impl<'a, B: Backend> UI<'a, B> {
    pub fn new(backend: B, output_options: UIOutputOptions) -> Self {
        enable_raw_mode().unwrap();
        execute!(std::io::stdout(), EnterAlternateScreen, EnableMouseCapture).unwrap();

        let mut _terminal = Terminal::new(backend).unwrap();
        _terminal.clear().unwrap();
        _terminal.hide_cursor().unwrap();

        let component_options = UIComponentOptions {
            output_options: output_options.clone(),
        };

        Self {
            state_flags: UIStateFlags::NONE,

            _dialog_outcome: output_options.dialog_outcome,
            _flag_outcome: output_options.flag_outcome,
            _terminal,

            // components
            _characters_modal: CharactersModal::new(component_options.clone()),
            _debug_panel: DebugPanel::new(component_options.clone()),
            _mode_panel: ModePanel::new(component_options.clone()),
            _realm_modal: RealmModal::new(component_options.clone()),
            _title: Title::new(component_options),
        }
    }

    pub fn render(&mut self, options: UIRenderOptions) {
        match options.message {
            IncomeMessageType::Message(output) => {
                self._debug_panel
                    .set_mode(options.client_flags.contains(ClientFlags::IN_DEBUG_MODE))
                    .add_item(output);
            },
            IncomeMessageType::ChooseCharacter(characters) => {
                self.state_flags.set(UIStateFlags::IS_CHARACTERS_MODAL_OPENED, true);
                self._characters_modal.set_items(characters);
            },
            IncomeMessageType::ChooseRealm(realms) => {
                self.state_flags.set(UIStateFlags::IS_REALM_MODAL_OPENED, true);
                self._realm_modal.set_items(realms);
            },
            IncomeMessageType::KeyEvent(modifiers, key_code) => {
                self.handle_key_event(modifiers, key_code);
            },
            IncomeMessageType::ResizeEvent => {
                self._terminal.autoresize().unwrap();
            }
        }

        self._terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(MARGIN)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Length(4),
                    Constraint::Percentage(76),
                    Constraint::Percentage(12),
                ])
                .split(frame.size());

            let output_panels = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(100),
                ])
                .split(chunks[2]);

            self._title.render(frame, chunks[0]);
            self._mode_panel.render(frame, chunks[1]);
            self._debug_panel.render(frame, output_panels[0]);

            if self.state_flags.contains(UIStateFlags::IS_CHARACTERS_MODAL_OPENED) {
                self._characters_modal.render(frame, chunks[2])
            }

            if self.state_flags.contains(UIStateFlags::IS_REALM_MODAL_OPENED) {
                self._realm_modal.render(frame, chunks[2])
            }

        }).unwrap();
    }

    fn handle_key_event(&mut self, modifiers: KeyModifiers, key_code: KeyCode) {
        self._characters_modal.handle_key_event(modifiers, key_code, &mut self.state_flags);
        self._mode_panel.handle_key_event(modifiers, key_code, &mut self.state_flags);
        self._realm_modal.handle_key_event(modifiers, key_code, &mut self.state_flags);

        if key_code == KeyCode::Char('c') {
            if modifiers.contains(KeyModifiers::CONTROL) {
                // TODO: probably need exit from app in different way
                disable_raw_mode().unwrap();
                execute!(std::io::stdout(), LeaveAlternateScreen, DisableMouseCapture).unwrap();
                exit(0);
            }
        }
    }
}

pub struct UIInput {
    _event_income: EventIncome,
}

impl UIInput {
    pub fn new(event_income: EventIncome) -> Self {
        Self {
            _event_income: event_income,
        }
    }

    pub fn handle(&mut self) {
        let event = read().unwrap();

        if let Event::Key(key) = event {
            self._event_income.send_key_event(key);
        }

        if let Event::Resize(_, _) = event {
            self._event_income.send_resize_event();
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
            OutcomeMessageType::SetUIMode(flag) => {
                if flag == UIModeFlags::DEBUG_MODE {
                    self.client_flags.toggle(ClientFlags::IN_DEBUG_MODE);
                }
            }
        };
    }
}