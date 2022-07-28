use crate::types::{HandlerFunction, HandlerInput, HandlerResult, ProcessorResult};

pub trait Processor {
    fn process_input(input: HandlerInput) -> ProcessorResult;

    fn collect_responses(
        handlers: Vec<HandlerFunction>,
        mut input: HandlerInput
    ) -> ProcessorResult {
        handlers.into_iter()
            .map(|mut func| func(&mut input))
            .collect::<Vec<HandlerResult>>()
    }
}