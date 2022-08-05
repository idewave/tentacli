use tui::backend::Backend;
use tui::Frame;
use tui::layout::Rect;

use crate::types::{HandlerInput, ProcessorResult};

pub trait Processor {
    fn process_input(input: &mut HandlerInput) -> ProcessorResult;
}

pub trait UIComponent {
    fn new() -> Self where Self: Sized;
    fn render<B: Backend>(&mut self, frame: &mut Frame<B>, rect: Rect);
}