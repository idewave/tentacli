use ratatui::Frame;
use ratatui::layout::{Margin, Rect};
use ratatui::prelude::{Color, Line, Modifier, Span, Style};
use ratatui::style::Stylize;
use ratatui::symbols::border;
use ratatui::widgets::{Block, Paragraph};
use tentacli_traits::types::Outputs;

use crate::features::ui2::builders::list::log_list_item;
use crate::features::ui2::components::ComponentEvent;
use crate::features::ui2::components::list::StyledList;
use crate::features::ui2::layout::{split_vertical, Values};
use crate::features::ui2::traits::{EventHandler, UIComponent};

pub const DONE_MSG: u8 = 0;
pub const FAIL_MSG: u8 = 1;
pub const DEBG_MSG: u8 = 2;
pub const RECV_MSG: u8 = 3;
pub const SENT_MSG: u8 = 4;

#[derive(Default)]
pub struct LogViewer<'a> {
    list: StyledList<'a>,
}

impl<'a> LogViewer<'a> {
    pub fn new() -> Self {
        let mut instance = Self::default();
        instance.list.autoscroll(true);

        instance
    }
    fn format_count(n: usize) -> String {
        match n {
            1_000_000_000.. => format!("{:.1}B", n as f64 / 1_000_000_000.0),
            1_000_000.. => format!("{:.1}M", n as f64 / 1_000_000.0),
            1_000.. => format!("{:.1}k", n as f64 / 1_000.0),
            _ => n.to_string(),
        }
    }
}

impl<'a> UIComponent for LogViewer<'a> {
    fn render(&mut self, frame: &mut Frame, rect: Rect) {
        let [top, bottom] = split_vertical(
            rect,
            Values::Percentages([90, 10]),
        );

        let title = Line::from(
            Span::raw("LOG OUTPUT").fg(Color::LightCyan).add_modifier(Modifier::BOLD)
        );

        let instructions = Line::from(vec![
            Span::styled(
                "To navigate, use ",
                Style::new().add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "<ArrowDn>",
                Style::new().fg(Color::LightBlue).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                " and ",
                Style::new().add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "<ArrowUp>",
                Style::new().fg(Color::LightBlue).add_modifier(Modifier::BOLD),
            ),
        ]);

        let block = Block::bordered()
            .title(title.right_aligned())
            .title_bottom(instructions.centered().bg(Color::Black))
            .border_set(border::ROUNDED);

        frame.render_widget(block, top);

        self.list.render(frame, top.inner(Margin { horizontal: 1, vertical: 1 }));

        let title = Line::from(
            Span::raw("STATS").fg(Color::LightCyan).add_modifier(Modifier::BOLD)
        );

        let block = Block::bordered()
            .title(title.right_aligned())
            .border_set(border::ROUNDED);

        let (done, fail, debg, recv, sent) = (
            Self::format_count(*self.list.counter.get(&DONE_MSG).unwrap_or(&0)),
            Self::format_count(*self.list.counter.get(&FAIL_MSG).unwrap_or(&0)),
            Self::format_count(*self.list.counter.get(&DEBG_MSG).unwrap_or(&0)),
            Self::format_count(*self.list.counter.get(&RECV_MSG).unwrap_or(&0)),
            Self::format_count(*self.list.counter.get(&SENT_MSG).unwrap_or(&0)),
        );

        let paragraph = Paragraph::new(Line::from(vec![
            Span::styled(format!("[DONE]:{done} "), Style::new().fg(Color::Green)),
            Span::styled(format!("[FAIL]:{fail} "), Style::new().fg(Color::Red)),
            Span::styled(format!("[DEBG]:{debg} "), Style::new().fg(Color::Gray)),
            Span::styled(format!("[RECV]:{recv} "), Style::new().fg(Color::Magenta)),
            Span::styled(format!("[SENT]:{sent}"), Style::new().fg(Color::LightBlue)),
        ])).left_aligned().block(block);

        frame.render_widget(paragraph, bottom);
    }
}

impl<'a> EventHandler for LogViewer<'a> {
    type Output = Option<Outputs>;

    fn handle_event(&mut self, event: &ComponentEvent) -> anyhow::Result<Self::Output> {
        match event {
            ComponentEvent::LogItem(marker, text) => {
                let text = match *marker {
                    DONE_MSG => format!("[DONE]: {text}"),
                    FAIL_MSG => format!("[FAIL]: {text}"),
                    DEBG_MSG => format!("[DEBG]: {text}"),
                    RECV_MSG => format!("[RECV]: {text}"),
                    SENT_MSG => format!("[SENT]: {text}"),
                    _ => String::new()
                };

                self.list.add_item(*marker, log_list_item(*marker, text));
            }
            _ => {
                self.list.handle_event(&event)?;
            }
        }

        Ok(None)
    }
}