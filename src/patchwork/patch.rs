use geo::{polygon, Polygon};

#[derive(Debug)]
pub struct Patch {
    geometry: Polygon,
}

impl Default for Patch {
    fn default() -> Self {
        Self {
            geometry: polygon![
                (x: 0.0, y: 0.0),
                (x: 0.0, y: 1.0),
                (x: 2.0, y: 1.0),
                (x: 2.0, y: 0.0),
                (x: 0.0, y: 0.0)
            ],
        }
    }
}
