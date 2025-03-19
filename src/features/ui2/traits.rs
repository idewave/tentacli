use ratatui::Frame;
use ratatui::layout::Rect;

use crate::features::ui2::components::ComponentEvent;

pub trait EventHandler {
    type Output;

    fn handle_event(&mut self, event: &ComponentEvent) -> anyhow::Result<Self::Output>;
}

pub trait UIComponent {
    fn render(&mut self, frame: &mut Frame, rect: Rect);
}

pub trait Paginator {
    fn prev(&mut self);

    fn next(&mut self);

    fn first(&mut self);

    fn last(&mut self);
}