use ratatui::Frame;
use ratatui::layout::Rect;

pub trait UIComponent {
    fn render(&mut self, frame: &mut Frame, rect: Rect);
}

pub trait Paginator {
    fn prev(&mut self);

    fn next(&mut self);

    fn first(&mut self);

    fn last(&mut self);
}

pub trait Focusable {
    fn holds_focus(&mut self) -> bool {
        true
    }

    fn on_focus(&mut self) {}
}