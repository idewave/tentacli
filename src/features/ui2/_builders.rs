pub fn log_list_item<'a>(msg: String, marker: u8) -> MarkedItem<'a> {
    let color = match marker {
        DONE_MSG => Color::Green,
        FAIL_MSG => Color::Red,
        DEBG_MSG => Color::Gray,
        RECV_MSG => Color::Magenta,
        SENT_MSG => Color::LightBlue,
        _ => Color::Gray
    };
    let local_time = Local::now().format("[%H:%M:%S]").to_string();
    let item = ListItem::new(Line::from(vec![
        Span::styled(
            format!("{local_time}"),
            Style::new().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            msg,
            Style::new().fg(color).add_modifier(Modifier::BOLD),
        ),
    ]));

    (marker, item)
}