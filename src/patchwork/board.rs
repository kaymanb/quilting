use std::fmt;

use geo::{coord, Rect};

#[derive(Debug)]
pub struct Board {
    width: usize,
    height: usize,
    geometry: Rect,
}

impl Default for Board {
    fn default() -> Self {
        let (width, height) = (9, 9);
        Self {
            width,
            height,
            geometry: Rect::new(
                coord! { x: 0., y: 0.},
                coord! { x: width as f64, y: height as f64},
            ),
        }
    }
}
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let edge = "-".repeat(self.width);

        writeln!(f, "+{edge}+")?;
        let empty_spaces = " ".repeat(self.width);
        for _ in 0..(self.height + 1) {
            writeln!(f, "|{empty_spaces}|")?;
        }
        writeln!(f, "+{edge}+")
    }
}
