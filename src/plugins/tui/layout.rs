use ratatui::layout::{Constraint, Flex, Layout, Rect};

#[allow(dead_code)]
pub enum Values<const N: usize> {
    Lengths([u16; N]),
    Percentages([u16; N]),
    Ratios([(u32, u32); N]),
    Maxes([u16; N]),
    Mins([u16; N]),
    Fills([u16; N]),
}

fn generate_constraints<const N: usize>(values: Values<N>) -> Vec<Constraint> {
    match values {
        Values::Percentages(percentages) => Constraint::from_percentages(percentages),
        Values::Lengths(lengths) => Constraint::from_lengths(lengths),
        Values::Ratios(ratios) => Constraint::from_ratios(ratios),
        Values::Maxes(maxes) => Constraint::from_maxes(maxes),
        Values::Mins(mins) => Constraint::from_mins(mins),
        Values::Fills(fills) => Constraint::from_fills(fills),
    }
}

pub fn split_vertical<const N: usize>(area: Rect, values: Values<N>) -> [Rect; N] {
    let constraints = generate_constraints(values);
    Layout::vertical(constraints).areas(area)
}

pub fn split_horizontal<const N: usize>(area: Rect, values: Values<N>) -> [Rect; N] {
    let constraints = generate_constraints(values);
    Layout::horizontal(constraints).areas(area)
}

pub fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}
