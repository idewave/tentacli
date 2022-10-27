use tui::backend::Backend;
use tui::Frame;
use tui::layout::Rect;

use crate::ui::types::UIComponentOptions;

pub trait UIComponent {
    fn new(options: UIComponentOptions) -> Self where Self: Sized;
    fn render<B: Backend>(&mut self, frame: &mut Frame<B>, rect: Rect);
}