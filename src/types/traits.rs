use tui::backend::Backend;
use tui::Frame;
use tui::layout::Rect;
use async_trait::async_trait;

use crate::types::{HandlerInput, HandlerResult, ProcessorResult};
use crate::ui::types::UIComponentOptions;

pub trait Processor {
    fn process_input(input: &mut HandlerInput) -> ProcessorResult;
}

pub trait UIComponent {
    fn new(options: UIComponentOptions) -> Self where Self: Sized;
    fn render<B: Backend>(&mut self, frame: &mut Frame<B>, rect: Rect);
}

#[async_trait]
pub trait PacketHandler {
    async fn handle(&mut self, input: &mut HandlerInput) -> HandlerResult;
}