use crate::primary::types::{HandlerInput, ProcessorResult};

pub trait Processor {
    fn process_input(input: &mut HandlerInput) -> ProcessorResult;
}