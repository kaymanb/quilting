use std::fmt;
use std::{collections::HashMap, error::Error};

use colored::{ColoredString, Colorize};
use geo::dimensions::Dimensions;
use geo::{coord, point, BooleanOps, Contains, HasDimensions, Point, Rect};

use crate::patchwork::{Patch, Shape};

#[derive(Debug)]
pub struct PlacementError {
    point: Point<u8>,
    shape: Shape,
}

impl Error for PlacementError {}

impl fmt::Display for PlacementError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let point = self.point;
        let shape = self.shape;
        write!(f, "Unable to place patch {shape:?} at point {point:?}")
    }
}

#[derive(Clone, Debug)]
pub struct Board {
    height: usize,
    width: usize,
    geometry: Rect,
    placements: HashMap<Point<u8>, Patch>,
}

impl Board {
    pub(crate) fn place(&mut self, point: Point<u8>, patch: Patch) -> Result<(), PlacementError> {
        let geometry = patch.relative_geometry(&point);
        let error = PlacementError {
            point,
            shape: patch.shape,
        };

        // Make sure that patch fits on the board.
        if !self.geometry.contains(&geometry) {
            return Err(error);
        }

        // Make sure that patch doesn't overlap with existing placements.
        for (epoint, epatch) in &self.placements {
            let existing_geometry = epatch.relative_geometry(epoint);
            let intersection = existing_geometry.intersection(&geometry);
            if intersection.dimensions() == Dimensions::TwoDimensional {
                return Err(error);
            }
        }
        self.placements.insert(point, patch);
        Ok(())
    }

    /// Returns a point and rotation for which a patch can be placed on the
    /// board. Returns `None` if no valid placement is available.
    pub(crate) fn fit(&self, patch: &Patch) -> Option<(Point, f64)> {
        TODO: Here
    }
}

impl Default for Board {
    fn default() -> Self {
        let (width, height) = (9, 9);
        Self {
            width,
            height,
            geometry: Rect::new(coord! {x: 0., y: 0.}, coord! {x: 9., y: 9.}),
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
