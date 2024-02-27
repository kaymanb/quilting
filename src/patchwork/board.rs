use std::collections::HashMap;
use std::fmt;

use geo::{coord, point, Contains, Point, Rect};

use crate::patchwork::Patch;

#[derive(Debug)]
pub struct Board {
    width: usize,
    height: usize,
    geometry: Rect<usize>,
    placements: HashMap<Point<usize>, Patch>,
}

impl Board {
    pub fn place(&mut self, point: Point<usize>, patch: Patch) {
        self.placements.insert(point, patch);
    }
}

impl Default for Board {
    fn default() -> Self {
        let (width, height) = (9, 9);
        Self {
            width,
            height,
            geometry: Rect::new(coord! { x: 0, y: 0}, coord! { x: width, y: height}),
            placements: HashMap::new(),
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let edge = "-".repeat(self.width);
        writeln!(f, "+{edge}+")?;

        for y in (0..self.height).rev() {
            let mut row = String::new();
            for x in 0..self.width {
                // Sample is offset by 0.5 to avoid boundary issues
                let mut found = false;
                let sample = point! { x: (x as f64 + 0.5), y: (y as f64+ 0.5) };
                for patch in self.placements.values() {
                    if patch.geometry.contains(&sample) {
                        found = true;
                        row.push('*');
                        break;
                    }
                }
                if !found {
                    row.push(' ');
                }
            }
            writeln!(f, "|{row}|")?;
        }

        writeln!(f, "+{edge}+")
    }
}
