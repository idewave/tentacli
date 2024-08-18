use std::collections::BTreeMap;
use crate::types::{HandlerInput, ProcessorResult};

pub trait Processor {
    fn get_handlers(input: &mut HandlerInput) -> ProcessorResult;
    fn get_one_time_handler_map() -> BTreeMap<u16, ProcessorResult> {
        BTreeMap::default()
    }
}