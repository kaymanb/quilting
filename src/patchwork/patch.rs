use colored::ColoredString;
use geo::{coord, Convert, CoordsIter, LineString, Point, Polygon};

use crate::patchwork::Shape;

#[derive(Debug)]
pub struct Patch {
    pub(crate) shape: Shape,
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
