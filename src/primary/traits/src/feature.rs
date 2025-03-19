use std::collections::BTreeMap;

use crate::types::{HandlerOutput, ProcessorFunction, ProcessorResult, Task};

pub trait Feature: Send {
    fn set_broadcast_channel(
        &mut self,
        _sender: async_broadcast::Sender<HandlerOutput>,
        _receiver: async_broadcast::Receiver<HandlerOutput>,
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