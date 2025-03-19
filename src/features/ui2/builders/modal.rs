use ratatui::prelude::{Color, Line, Modifier, Span, Style};
use ratatui::widgets::ListItem;

use crate::features::ui2::components::modal::{Modal, ModalCallback};

pub struct CharacterItem {
    pub name: String,
    pub guid: String,
}

pub struct RealmItem {
    pub name: String,
    pub address: String,
    pub server_id: String,
}

pub fn build_characters_modal<'a>(
    characters: Vec<CharacterItem>, callback: ModalCallback,
) -> Modal<'a> {
    Modal::default()
        .set_title("SELECT CHARACTER")
        .set_callback(callback)
        .set_items(characters
            .into_iter()
            .map(|CharacterItem { name, guid }| {
                let padding = " ".repeat(20 - name.len());
                ListItem::new(Line::from(vec![
                    Span::styled(
                        "NAME: [",
                        Style::new().fg(Color::Magenta).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        name,
                        Style::new().fg(Color::LightGreen).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!("],{padding}GUID: "),
                        Style::new().fg(Color::Magenta).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!("{guid}"),
                        Style::new().fg(Color::Gray).add_modifier(Modifier::BOLD),
                    ),
                ]))
            })
            .collect())
}

pub fn build_realm_modal<'a>(realms: Vec<RealmItem>, callback: ModalCallback) -> Modal<'a> {
    Modal::default()
        .set_title("SELECT REALM")
        .set_callback(callback)
        .set_items(realms
            .into_iter()
            .map(|RealmItem { name, address, server_id }| {
                ListItem::new(Line::from(vec![
                    Span::styled(
                        "NAME: [",
                        Style::new().fg(Color::Magenta).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        name,
                        Style::new().fg(Color::Gray).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        "], ADDRESS: [",
                        Style::new().fg(Color::Magenta).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        address,
                        Style::new().fg(Color::Green).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        "], SERVER_ID: ",
                        Style::new().fg(Color::Magenta).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!("{server_id}"),
                        Style::new().fg(Color::Gray).add_modifier(Modifier::BOLD),
                    ),
                ]))
            }).collect())
}