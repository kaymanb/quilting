use colored::{ColoredString, Colorize};
use geo::{polygon, Polygon};

#[derive(Copy, Clone, Debug)]
pub enum Shape {
    Start,
    Tee,
    Ell,
}

impl Shape {
    /// Returns the number of buttons on a shape.
    fn buttons(&self) -> usize {
        match self {
            Shape::Start => 0,
            Shape::Tee => 2,
            Shape::Ell => 1,
        }
    }

    /// Returns the default geometry for a shape.
    ///
    /// The origin (0, 0) is always at the bottom left corner of a geometry.
    pub(crate) fn geometry(&self) -> Polygon {
        match self {
            Shape::Start => polygon![
                (x: 0.0, y: 0.0),
                (x: 0.0, y: 1.0),
                (x: 2.0, y: 1.0),
                (x: 2.0, y: 0.0),
                (x: 0.0, y: 0.0)
            ],
            Shape::Tee => polygon![
                (x: 0.0, y: 0.0),
                (x: 0.0, y: 2.0),
                (x: -1.0, y: 2.0),
                (x: -1.0, y: 3.0),
                (x: 2.0, y: 3.0),
                (x: 2.0, y: 2.0),
                (x: 1.0, y: 2.0),
                (x: 1.0, y: 0.0),
                (x: 0.0, y: 0.0),
            ],
            Shape::Ell => polygon![
                (x: 0.0, y: 0.0),
                (x: 0.0, y: 3.0),
                (x: 1.0, y: 3.0),
                (x: 1.0, y: 1.0),
                (x: 2.0, y: 1.0),
                (x: 2.0, y: 0.0),
                (x: 0.0, y: 0.0),
            ],
        }
    }

    /// Returns the pattern of a shape.
    pub(crate) fn pattern(&self) -> ColoredString {
        match self {
            Shape::Start => "󱨎".bright_black().on_truecolor(170, 200, 160),
            Shape::Tee => "󰓦".white().on_truecolor(170, 185, 95),
            Shape::Ell => "󰴈".green().on_truecolor(195, 80, 80),
        }
    }
}
