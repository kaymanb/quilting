use colored::ColoredString;
use geo::{coord, Convert, CoordsIter, LineString, Point, Polygon};
use rand::Rng;

use crate::patchwork::Shape;

#[derive(Clone, Debug)]
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

#[derive(Debug)]
pub struct PatchCircle {
    spool: usize,
    patches: Vec<Patch>,
}

impl PatchCircle {
    pub(crate) fn next(&self, k: usize) -> Vec<(usize, &Patch)> {
        let mut next = vec![];
        for i in 0..k {
            let index = self.spool + i;
            next.push((index, &self.patches[index]));
        }
        next
    }
}

impl Default for PatchCircle {
    fn default() -> Self {
        let shapes: Vec<Shape> = (0..Shape::NUM).map(|_| rand::random()).collect();
        Self {
            spool: 0,
            patches: shapes.into_iter().map(Patch::from_shape).collect(),
        }
    }
}
