use serde::{Deserialize, Serialize};

use crate::client::ChoiceItems;
use crate::plugins::tui::components::app::OutputItem;

#[derive(Clone, Serialize, Deserialize)]
pub struct OutputEvent {
    pub items: Vec<OutputItem>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ChoicesEvent(pub ChoiceItems);
