use crossterm::event::{KeyCode, KeyModifiers};
use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Alignment, Rect};
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, BorderType, Paragraph, Wrap};
use crate::ipc::pipe::flag::FlagOutcome;

use crate::types::traits::UIComponent;
use crate::ui::types::{UIComponentOptions, UIModeFlags, UIStateFlags};

const PANEL_TITLE: &str = "MODES";

pub struct ModePanel {
    _flag_outcome: Option<FlagOutcome>,
    mode_flags: UIModeFlags,
}

impl ModePanel {
    pub fn toggle_flag(&mut self, number: u8) {
        let flag_outcome = self._flag_outcome.as_mut().unwrap();

        match number {
            1 => {
                self.mode_flags.toggle(UIModeFlags::DEBUG_MODE);
                flag_outcome.send_toggle_flag(UIModeFlags::DEBUG_MODE);
            },
            _ => {},
        }
    }

    pub fn handle_key_event(
        &mut self,
        key_modifiers: KeyModifiers,
        key_code: KeyCode,
        _: &mut UIStateFlags
    ) {
        match key_code {
            KeyCode::Char('1') => {
                if key_modifiers.contains(KeyModifiers::CONTROL) {
                    self.toggle_flag(1);
                }
            },
            _ => {},
        }
    }
}

impl UIComponent for ModePanel {
    fn new(options: UIComponentOptions) -> Self {
        Self {
            _flag_outcome: Some(options.output_options.flag_outcome),
            mode_flags: UIModeFlags::NONE,
        }
    }

    fn render<B: Backend>(&mut self, frame: &mut Frame<B>, rect: Rect) {
        let block = Block::default()
            .title(PANEL_TITLE)
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Double);

        let help_text = vec![
            Span::styled(
                "To toggle mode use CTRL + <NUMBER>",
                Style::default().fg(Color::LightBlue)
            )
        ];

        let mut spans = vec![];

        if self.mode_flags.contains(UIModeFlags::DEBUG_MODE) {
            spans.push(Span::styled("[1] DEBUG", Style::default().fg(Color::LightGreen)));
        } else {
            spans.push(Span::raw("[1] DEBUG"));
        }

        let paragraph = Paragraph::new(vec![Spans::from(help_text), Spans::from(spans)])
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true })
            .block(block);

        frame.render_widget(paragraph, rect);
    }
}