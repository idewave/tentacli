use std::collections::BTreeMap;

use async_broadcast::{Receiver as BroadcastReceiver, Sender as BroadcastSender};

use crate::types::{HandlerOutput, ProcessorFunction, ProcessorResult, Task};

pub trait Feature: Send {
    fn set_broadcast_channel(
        &mut self,
        _sender: BroadcastSender<HandlerOutput>,
        _receiver: BroadcastReceiver<HandlerOutput>,
    ) {}

    fn get_tasks(&mut self) -> anyhow::Result<Vec<Task>> {
        Ok(vec![])
    }

    fn get_login_processors(&self) -> Vec<ProcessorFunction> {
        vec![]
    }

    fn get_realm_processors(&self) -> Vec<ProcessorFunction> {
        vec![]
    }

    fn get_one_time_handler_maps(&self) -> Vec<BTreeMap<u16, ProcessorResult>> {
        vec![]
    }

    fn get_initial_processors(&self) -> Vec<ProcessorFunction> {
        vec![]
    }
}