use std::io::Stdout;
use crossterm::{
    event::{
        self,
        DisableMouseCapture,
        EnableMouseCapture,
        Event,
        KeyCode
    },
    execute,
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen
    }
};
use tui::backend::{Backend, CrosstermBackend};
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::Terminal;
use tui::text::{Span, Spans};

use crate::ipc::duplex::types::{LoggerOutput, IncomeMessageType};
use crate::types::traits::UIComponent;
use crate::ui::characters_modal::CharactersModal;
use crate::ui::debug_panel::DebugPanel;
use crate::ui::title::Title;
use crate::ui::types::{UIOptions};

mod characters_modal;
mod debug_panel;
pub mod types;
mod title;

pub const MARGIN: u16 = 1;

pub struct UI<'a, B: Backend> {
    _terminal: Terminal<B>,
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

            // components
            _debug_panel: DebugPanel::new(),
            _title: Title::new(),
            _characters_modal: CharactersModal::new(),
        }
    }

    pub fn render(&mut self, options: UIOptions) {
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

            match options.message {
                IncomeMessageType::Message(output) => {
                    self._debug_panel.add_item(output);
                },
                IncomeMessageType::ChooseCharacter(characters) => {
                    self._characters_modal.set_items(characters);
                },
                IncomeMessageType::ChooseRealm(_realms) => {
                    // ...
                },
            }

            self._title.render(frame, chunks[0]);
            self._debug_panel.render(frame, output_panels[0]);

            if self._characters_modal.has_items() {
                self._characters_modal.render(frame, chunks[1])
            }

        }).unwrap();
    }
}

pub struct UIInput;

impl UIInput {
    pub fn new() -> Self {
        Self
    }

    pub fn handle(&mut self) {
        if let event::Event::Key(key) = event::read().unwrap() {
            // ...
        }
    }
}