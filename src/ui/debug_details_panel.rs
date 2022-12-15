use crossterm::event::{KeyCode, KeyModifiers};
use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Alignment, Rect};
use tui::style::{Color, Style};
use tui::text::{Text};
use tui::widgets::{Block, Borders, BorderType, Paragraph, Wrap};

use crate::traits::ui_component::UIComponent;
use crate::ui::MARGIN;
use crate::ui::types::{UIComponentOptions, UIStateFlags};

const PANEL_TITLE: &str = "DEBUG DETAILS";

pub struct DebugDetailsPanel<'a> {
    text: Text<'a>,
    scroll_offset: u16,
    panel_height: u16,
}

impl<'a> DebugDetailsPanel<'a> {
    pub fn set_output(&mut self, output: String) -> &mut Self {
        self.scroll_offset = 0;
        self.text = Text::styled(output, Style::default());
        self
    }

    pub fn handle_key_event(
        &mut self,
        key_modifiers: KeyModifiers,
        key_code: KeyCode,
        _: &mut UIStateFlags
    ) {
        match key_code {
            KeyCode::Down if key_modifiers.contains(KeyModifiers::CONTROL) => {
                if self.text.height() > (self.panel_height as usize) {
                    if (self.scroll_offset as usize) <
                        self.text.height() - (self.panel_height as usize)
                    {
                        self.scroll_offset += 1;
                    }
                }
            },
            KeyCode::Up if key_modifiers.contains(KeyModifiers::CONTROL) => {
                if self.scroll_offset > 0 {
                    self.scroll_offset -= 1;
                }
            },
            _ => {},
        }
    }
}

impl<'a> UIComponent for DebugDetailsPanel<'a> {
    fn new(_: UIComponentOptions) -> Self {
        Self {
            text: Text::default(),
            scroll_offset: 0,
            panel_height: 0,
        }
    }

    fn render<B: Backend>(&mut self, frame: &mut Frame<B>, rect: Rect) {
        self.panel_height = rect.height - MARGIN * 2;

        let block = Block::default()
            .title(PANEL_TITLE)
            .borders(Borders::ALL)
            .border_type(BorderType::Plain);

        let paragraph = Paragraph::new(self.text.clone())
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: false })
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .scroll((self.scroll_offset, 0))
            .block(block);

        frame.render_widget(paragraph, rect);
    }
}