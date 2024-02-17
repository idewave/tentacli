pub mod types;

use crate::primary::traits::processor::Processor;
use crate::primary::types::{HandlerInput, ProcessorResult};

pub struct TradeProcessor;

impl Processor for TradeProcessor {
    fn get_handlers(_: &mut HandlerInput) -> ProcessorResult {
        let handlers: ProcessorResult = vec![];

        handlers
    }
}