use std::fmt;

use colored::{ColoredString, Colorize};
use geo::{coord, polygon, Convert, CoordsIter, LineString, Point, Polygon};

#[derive(Debug)]
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
    fn geometry(&self) -> Polygon {
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
    fn pattern(&self) -> ColoredString {
        match self {
            Shape::Start => "󱨎".bright_black().on_green(),
            Shape::Tee => "".white().on_green(),
            Shape::Ell => "󱢆".green().on_red(),
        }
    }
}

#[derive(Debug)]
pub struct Patch {
    shape: Shape,
    geometry: Polygon,
}

impl Patch {
    /// Creates a new patch for a specific shape.
    pub fn from_shape(shape: Shape) -> Self {
        Self {
            geometry: shape.geometry(),
            shape,
        }
    }

    /// Returns the pattern of a patch.
    pub(crate) fn pattern(&self) -> ColoredString {
        self.shape.pattern()
    }

    /// Returns the geometry of a patch relative to a specific point.
    pub fn relative_geometry(&self, point: &Point<u8>) -> Polygon {
        let point: Point<f64> = point.convert();
        let exterior: LineString = self
            .geometry
            .coords_iter()
            .map(|c| coord! {x: point.x() + c.x, y: point.y() + c.y})
            .collect();
        Polygon::new(exterior, vec![])
    }
}
