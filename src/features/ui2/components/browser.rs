use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::Frame;
use ratatui::layout::{Margin, Rect};
use ratatui::prelude::{Color, Line, Modifier, Span, Style, Stylize};
use ratatui::symbols::border;
use ratatui::widgets::{Block, Paragraph};

use crate::features::ui2::components::list::StyledList;
use crate::features::ui2::components::modal::Modal;
use crate::features::ui2::layout::{split_horizontal, split_vertical, Values};
use crate::features::ui2::traits::{EventHandler, UIComponent};

pub const DONE_MSG: u8 = 0;
pub const FAIL_MSG: u8 = 1;
pub const DEBG_MSG: u8 = 2;
pub const RECV_MSG: u8 = 3;
pub const SENT_MSG: u8 = 4;

#[derive(Default)]
pub struct Browser<'a> {
    pub list: StyledList<'a>,
    pub filter_modal: Modal<'a>,
}

impl<'a> UIComponent for Browser<'a> {
    fn render(&mut self, frame: &mut Frame, rect: Rect) {
        if self.filter_modal.opened() {
            self.filter_modal.render(frame, rect);
        } else {
            let [left_column, right_column] = split_horizontal(
                rect,
                Values::Percentages([30, 70]),
            );

            let [left_top, left_bottom_info_panel, ..] = split_vertical(
                left_column,
                Values::Percentages([70, 6, 24]),
            );

            let instructions = Line::from(vec![
                Span::styled("To navigate, use ", Style::new().add_modifier(Modifier::BOLD)),
                Span::styled("<ArrowDn>", Style::new().fg(Color::LightBlue).add_modifier(Modifier::BOLD)),
                Span::styled(" and ", Style::new().add_modifier(Modifier::BOLD)),
                Span::styled("<ArrowUp>", Style::new().fg(Color::LightBlue).add_modifier(Modifier::BOLD)),
            ]);

            let title = Line::from(
                Span::raw("LOG OUTPUT")
                    .fg(Color::LightCyan).add_modifier(Modifier::BOLD)
            );

            let block = Block::bordered()
                .title(title.right_aligned())
                .title_bottom(instructions.centered().bg(Color::Black))
                .border_set(border::ROUNDED);

            frame.render_widget(block, left_top);

            let title = Line::from(
                Span::raw("STATS")
                    .fg(Color::LightCyan).add_modifier(Modifier::BOLD)
            );

            let block = Block::bordered()
                .title(title.right_aligned())
                .border_set(border::ROUNDED);

            let (done, fail, debg, recv, sent) = (
                format_count(*self.list.counter.get(&DONE_MSG).unwrap_or(&0)),
                format_count(*self.list.counter.get(&FAIL_MSG).unwrap_or(&0)),
                format_count(*self.list.counter.get(&DEBG_MSG).unwrap_or(&0)),
                format_count(*self.list.counter.get(&RECV_MSG).unwrap_or(&0)),
                format_count(*self.list.counter.get(&SENT_MSG).unwrap_or(&0)),
            );

            let paragraph = Paragraph::new(Line::from(vec![
                Span::styled(format!("[DONE]:{done} "), Style::new().fg(Color::Green)),
                Span::styled(format!("[FAIL]:{fail} "), Style::new().fg(Color::Red)),
                Span::styled(format!("[DEBG]:{debg} "), Style::new().fg(Color::Gray)),
                Span::styled(format!("[RECV]:{recv} "), Style::new().fg(Color::Magenta)),
                Span::styled(format!("[SENT]:{sent}"), Style::new().fg(Color::LightBlue)),
            ])).left_aligned().block(block);

            frame.render_widget(paragraph, left_bottom_info_panel);

            self.list.render(frame, left_top.inner(Margin { horizontal: 1, vertical: 1 }));
        }
    }
}

impl<'a> EventHandler for Browser<'a> {
    fn handle_event(&mut self, code: KeyCode, modifiers: KeyModifiers) -> anyhow::Result<()> {
        if code == KeyCode::Char('f') {
            self.filter_modal.toggle();
        }

        if self.filter_modal.opened() {
            self.filter_modal.handle_event(code, modifiers)?;
        } else {
            self.list.handle_event(code, modifiers)?;
        }

        Ok(())
    }
}

fn format_count(n: usize) -> String {
    match n {
        1_000_000_000.. => format!("{:.1}B", n as f64 / 1_000_000_000.0),
        1_000_000.. => format!("{:.1}M", n as f64 / 1_000_000.0),
        1_000.. => format!("{:.1}k", n as f64 / 1_000.0),
        _ => n.to_string(),
    }
}