use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::Frame;
use ratatui::layout::{Constraint, Rect};
use tentacli_traits::types::{HandlerOutput, Outputs};

pub use list::Marker;
pub use log_viewer::{DEBG_MSG, DONE_MSG, FAIL_MSG, RECV_MSG, SENT_MSG};
pub use modal::ModalResponse;

use crate::features::ui2::builders::modal::{
    build_characters_modal, build_realm_modal, CharacterItem, RealmItem,
};
use crate::features::ui2::components::log_viewer::LogViewer;
use crate::features::ui2::components::modal::{Modal, ModalCallback};
use crate::features::ui2::layout::{center, split_horizontal, split_vertical, Values};
use crate::features::ui2::traits::{EventHandler, UIComponent};

mod list;
pub(crate) mod modal;
mod log_viewer;

pub enum ComponentEvent {
    KeyEvent(KeyCode, KeyModifiers),
    CharactersList(Vec<CharacterItem>, ModalCallback),
    RealmList(Vec<RealmItem>, ModalCallback),
    LogItem(Marker, String),
}

#[derive(Default)]
pub struct App<'a> {
    modal: Modal<'a>,
    log_viewer: LogViewer<'a>,
    output: Vec<HandlerOutput>,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        Self {
            log_viewer: LogViewer::new(),
            ..Default::default()
        }
    }
    pub fn handle_event(&mut self, event: ComponentEvent) -> anyhow::Result<Option<Outputs>> {
        let mut outputs = None;

        match event {
            ComponentEvent::CharactersList(characters, callback) => {
                self.modal = build_characters_modal(characters, callback);
                self.modal.open();
            }
            ComponentEvent::RealmList(realms, callback) => {
                self.modal = build_realm_modal(realms, callback);
                self.modal.open();
            }
            ComponentEvent::LogItem(_, _) => {
                self.log_viewer.handle_event(&event)?;
            }
            ComponentEvent::KeyEvent(_, _) if self.modal.opened() => {
                if let ModalResponse::External(o) = self.modal.handle_event(&event)? {
                    outputs = Some(o);
                    self.modal.close();
                }
            }
            ComponentEvent::KeyEvent(_, _) => {
                self.log_viewer.handle_event(&event)?;
            }
        }

        Ok(outputs)
    }
}

impl<'a> UIComponent for App<'a> {
    fn render(&mut self, frame: &mut Frame, rect: Rect) {
        // --------LOG OUTPUT------------------------------------------
        // |                  |                                       |
        // |                  |                                       |
        // |                  |                                       |
        // |                  |                                       |
        // |                  |                                       |
        // |------------STATS-|                                       |
        // |                  |                                       |
        // |                  |                                       |
        // |                  |                                       |
        // ------------------------------------------------------------

        let [left_column, right_column] = split_horizontal(
            rect,
            Values::Percentages([30, 70]),
        );

        let [left_top, left_bottom] = split_vertical(
            left_column,
            Values::Percentages([80, 20]),
        );

        let modal_layout = center(rect, Constraint::Percentage(70), Constraint::Percentage(70));

        self.log_viewer.render(frame, left_top);

        // modal on last to override rest
        self.modal.render(frame, modal_layout);
    }
}