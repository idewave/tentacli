use std::collections::HashMap;
use std::sync::{Arc};
use async_broadcast::Receiver;
use tokio::sync::mpsc::Sender;
use tokio::sync::{RwLock};
use tokio_util::sync::CancellationToken;

mod components;
mod events;
mod layout;
mod traits;
mod theme;

// use crate::client::{CtxMap, Echo, OrderedOutput, ServerLabel, Task};
// use crate::client::plugin::CorePlugin;
use crate::client::prelude::*;
use crate::plugins::tui::components::app::App;

#[derive(Default)]
pub struct TUIPlugin;

impl CorePlugin for TUIPlugin {
    fn get_tasks(
        &self,
        broadcast_rx: Receiver<OrderedOutput>,
        echo_senders: Arc<HashMap<ServerLabel, Sender<Echo>>>,
        shutdown: CancellationToken,
        _: Arc<RwLock<CtxMap>>,
    ) -> anyhow::Result<Vec<Task>> {
        Ok(vec![
            App::handle_run(broadcast_rx, echo_senders.clone(), shutdown.clone())
        ])
    }
}