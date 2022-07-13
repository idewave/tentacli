use crate::types::{HandlerFunction, HandlerInput, HandlerOutput, ProcessorResult};

pub trait Processor {
    fn process_input(input: HandlerInput) -> ProcessorResult;

    fn collect_responses(
        handlers: Vec<HandlerFunction>,
        mut input: HandlerInput
    ) -> ProcessorResult {
        let responses = handlers
            .into_iter()
            .filter_map(|mut func| func(&mut input).ok())
            .collect::<Vec<HandlerOutput>>();

        Ok(responses)
    }
}