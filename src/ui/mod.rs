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
use tui::backend::{CrosstermBackend};
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::Terminal;
use tui::text::{Span, Spans};

use crate::message_pipe::types::{LoggerOutput, MessageType};
use crate::ui::debug_panel::DebugPanel;
use crate::ui::title::Title;
use crate::ui::types::UIOptions;

mod debug_panel;
pub mod types;
mod title;

pub const MARGIN: u16 = 1;

pub struct UI<'a> {
    _terminal: Terminal<CrosstermBackend<Stdout>>,
    spans: Vec<Spans<'a>>,
}

impl<'a> UI<'a> {
    pub fn new() -> Self {
        enable_raw_mode().unwrap();
        let mut stdout = std::io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
        let backend = CrosstermBackend::new(stdout);
        let mut _terminal = Terminal::new(backend).unwrap();
        _terminal.clear().unwrap();
        _terminal.hide_cursor().unwrap();

        Self {
            _terminal,
            spans: vec![],
        }
    }

    pub fn render(&mut self, options: UIOptions) {
        match options.message {
            MessageType::Message(output) => { self.build_debug_output(output); },
            MessageType::ChooseCharacter(_characters) => {},
            MessageType::ChooseRealm(_realms) => {},
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

            Title::render(frame, chunks[0]);
            DebugPanel::render(frame, output_panels[0], self.spans.to_vec());

        }).unwrap();
    }

    fn build_debug_output(&mut self, output: LoggerOutput) {
        let message = match output {
            LoggerOutput::Info(data) if !data.is_empty() => Span::styled(
                format!("[INFO]: {}\n", data), Style::default().fg(Color::Gray)
            ),
            LoggerOutput::Debug(data) if !data.is_empty() => Span::styled(
                format!("[DEBUG]: {}\n", data), Style::default().fg(Color::DarkGray)
            ),
            LoggerOutput::Error(data) if !data.is_empty() => Span::styled(
                format!("[ERROR]: {}\n", data), Style::default().fg(Color::Red)
            ),
            LoggerOutput::Success(data) if !data.is_empty() => Span::styled(
                format!("[SUCCESS]: {}\n", data), Style::default().fg(Color::LightGreen)
            ),
            LoggerOutput::Server(data) if !data.is_empty() => Span::styled(
                format!("[SERVER]: {}\n", data), Style::default().fg(Color::LightMagenta)
            ),
            LoggerOutput::Client(data) if !data.is_empty() => Span::styled(
                format!("[CLIENT]: {}\n", data), Style::default().fg(Color::LightBlue)
            ),
            _ => Span::raw(""),
        };

        if !message.content.is_empty() {
            self.spans.push(Spans::from(message));
        }
    }
}