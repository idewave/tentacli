use chrono::Local;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::ListItem;

use crate::features::ui2::{DEBG_MSG, DONE_MSG, FAIL_MSG, RECV_MSG, SENT_MSG};
use crate::features::ui2::components::Marker;

pub fn log_list_item<'a>(marker: Marker, msg: String) -> ListItem<'a> {
    let local_time = Local::now().format("[%H:%M:%S]").to_string();
    let item = ListItem::new(Line::from(vec![
        Span::styled(
            format!("{local_time}"),
            Style::new().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            msg,
            Style::new()
                .fg(match marker {
                    DONE_MSG => Color::Green,
                    FAIL_MSG => Color::Red,
                    DEBG_MSG => Color::Gray,
                    RECV_MSG => Color::Magenta,
                    SENT_MSG => Color::LightBlue,
                    _ => Color::Gray
                })
                .add_modifier(Modifier::BOLD),
        ),
    ]));

    item
}