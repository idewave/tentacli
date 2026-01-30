use serde::{Deserialize, Serialize};

use crate::plugins::tui::components::app::OutputItem;

#[derive(Serialize, Deserialize, Clone)]
pub struct SetItem(pub OutputItem);