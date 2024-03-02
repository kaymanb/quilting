use std::collections::HashMap;
use std::fmt;

use colored::{ColoredString, Colorize};
use geo::{coord, point, Contains, Point, Rect};

use crate::patchwork::Patch;

#[derive(Debug)]
pub struct Board {
    height: usize,
    width: usize,
    geometry: Rect<usize>,
    placements: HashMap<Point<u8>, Patch>,
}

impl Board {
    pub fn place(&mut self, point: Point<u8>, patch: Patch) {
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
        let mut rows = Vec::new();
        for y in (0..self.height).rev() {
            let mut row = Vec::new();
            for x in 0..self.width {
                // Sample is offset by 0.5 to avoid boundary issues
                let mut found = false;
                let sample = point! { x: (x as f64 + 0.5), y: (y as f64+ 0.5) };
                for (point, patch) in &self.placements {
                    if patch.relative_geometry(point).contains(&sample) {
                        found = true;
                        row.push(patch.pattern());
                        break;
                    }
                }
                if !found {
                    row.push(" ".into());
                }
            }
            rows.push(row);
        }

        let mut viewer = Viewer::new(f);
        viewer.top(self.width)?;
        for row in rows.into_iter() {
            viewer.row(row)?;
        }
        viewer.bottom(self.width)
    }
}

struct Viewer<'a, 'b> {
    out: &'a mut fmt::Formatter<'b>,
}

impl<'a, 'b> Viewer<'a, 'b> {
    // Due to character dimensions, a board will render more like
    // a square if we "double" the width.
    const WIDTH_MULTIPLE: usize = 2;

    const VERTICAL: &'static str = "│";
    const HORIZONTAL: &'static str = "─";

    const TOP_LEFT: &'static str = "╭";
    const TOP_RIGHT: &'static str = "╮";
    const BOTTOM_LEFT: &'static str = "╰";
    const BOTTOM_RIGHT: &'static str = "╯";

    fn new(out: &'a mut fmt::Formatter<'b>) -> Self {
        Self { out }
    }

    fn top(&mut self, width: usize) -> Result<(), fmt::Error> {
        let left = Viewer::TOP_LEFT;
        let right = Viewer::TOP_RIGHT;
        let middle = Viewer::HORIZONTAL.repeat(width * Viewer::WIDTH_MULTIPLE);
        let top = Viewer::colorize(format!("{left}{middle}{right}"));
        writeln!(self.out, "{top}")
    }

    fn bottom(&mut self, width: usize) -> Result<(), fmt::Error> {
        let left = Viewer::BOTTOM_LEFT;
        let right = Viewer::BOTTOM_RIGHT;
        let middle = Viewer::HORIZONTAL.repeat(width * Viewer::WIDTH_MULTIPLE);
        let bottom = Viewer::colorize(format!("{left}{middle}{right}"));
        writeln!(self.out, "{bottom}")
    }

    fn row(&mut self, tiles: Vec<ColoredString>) -> Result<(), fmt::Error> {
        let edge = Viewer::colorize(Viewer::VERTICAL.to_owned());
        write!(self.out, "{edge}")?;
        for tile in tiles.iter() {
            for _ in 0..Viewer::WIDTH_MULTIPLE {
                write!(self.out, "{tile}")?;
            }
        }
        writeln!(self.out, "{edge}")
    }

    fn colorize(string: String) -> ColoredString {
        string.bright_black()
    }
}
