use crate::types::{HandlerInput, ProcessorResult};

pub trait Processor {
    fn get_handlers(input: &mut HandlerInput) -> ProcessorResult;
}