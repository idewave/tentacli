use std::collections::BTreeMap;
use anyhow::{Result as AnyResult};
use tokio::task::JoinHandle;
use async_broadcast::{Receiver as BroadcastReceiver, Sender as BroadcastSender};

use crate::types::{HandlerOutput, ProcessorFunction, ProcessorResult};

pub trait Feature: Send {
    fn new() -> Self where Self: Sized;
    fn set_broadcast_channel(
        &mut self,
        sender: BroadcastSender<HandlerOutput>,
        receiver: BroadcastReceiver<HandlerOutput>,
    );
    fn get_tasks(&mut self) -> AnyResult<Vec<JoinHandle<()>>>;
    fn get_login_processors(&self) -> Vec<ProcessorFunction> {
        vec![]
    }

    fn get_realm_processors(&self) -> Vec<ProcessorFunction> {
        vec![]
    }
    fn get_one_time_handler_maps(&self) -> Vec<BTreeMap<u16, ProcessorResult>> {
        vec![]
    }
}