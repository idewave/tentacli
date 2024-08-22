use std::collections::BTreeMap;
use crate::types::{ProcessorResult};

pub trait Processor {
    fn get_handlers(_opcode: u16) -> ProcessorResult {
        vec![]
    }

    fn get_one_time_handler_map() -> BTreeMap<u16, ProcessorResult> {
        BTreeMap::default()
    }

    fn get_initial_handlers(_opcode: u16) -> ProcessorResult {
        vec![]
    }
}