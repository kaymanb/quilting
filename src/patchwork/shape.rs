use colored::{ColoredString, Colorize};
use geo::{polygon, Polygon};
use rand::distributions::{Distribution, Standard};
use rand::Rng;

#[derive(Copy, Clone, Debug)]
pub enum Shape {
    Start,
    T,
    L,
    I,
    U,
    SpaceInvader,
    LongPlus,
    FatPlus,
    LongL,
    BlueL,
    Step,
    LongT,
    LongI,
    HalfCross,
    StripedStep,
}

impl Distribution<Shape> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Shape {
        match rng.gen_range(0..Shape::NUM) {
            0 => Shape::Start,
            1 => Shape::T,
            2 => Shape::L,
            3 => Shape::I,
            4 => Shape::U,
            5 => Shape::SpaceInvader,
            6 => Shape::LongPlus,
            7 => Shape::FatPlus,
            8 => Shape::LongL,
            9 => Shape::BlueL,
            10 => Shape::Step,
            11 => Shape::LongT,
            12 => Shape::LongI,
            13 => Shape::HalfCross,
            _ => Shape::StripedStep,
        }
    }
}

impl Shape {
    pub(crate) const NUM: u8 = 15;
    /// Returns the number of buttons on a shape.
    fn buttons(&self) -> usize {
        match self {
            Shape::Start => 0,
            Shape::T => 2,
            Shape::L => 1,
            Shape::I => 0,
            Shape::U => 0,
            Shape::SpaceInvader => 2,
            Shape::LongPlus => 1,
            Shape::FatPlus => 1,
            Shape::LongL => 2,
            Shape::BlueL => 2,
            Shape::Step => 1,
            Shape::LongT => 2,
            Shape::LongI => 1,
            Shape::HalfCross => 1,
            Shape::StripedStep => 3,
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
            Shape::T => polygon![
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
            Shape::L => polygon![
                (x: 0.0, y: 0.0),
                (x: 0.0, y: 3.0),
                (x: 1.0, y: 3.0),
                (x: 1.0, y: 1.0),
                (x: 2.0, y: 1.0),
                (x: 2.0, y: 0.0),
                (x: 0.0, y: 0.0),
            ],
            Shape::I => polygon![
                (x: 0.0, y: 0.0),
                (x: 0.0, y: 3.0),
                (x: 1.0, y: 3.0),
                (x: 1.0, y: 0.0),
                (x: 0.0, y: 0.0),
            ],
            Shape::U => polygon![
                (x: 0.0, y: 0.0),
                (x: 0.0, y: 2.0),
                (x: 1.0, y: 2.0),
                (x: 1.0, y: 1.0),
                (x: 2.0, y: 1.0),
                (x: 2.0, y: 2.0),
                (x: 3.0, y: 2.0),
                (x: 3.0, y: 0.0),
                (x: 0.0, y: 0.0),
            ],
            Shape::SpaceInvader => polygon![
                (x: 0.0, y: 0.0),
                (x: 0.0, y: 2.0),
                (x: 1.0, y: 2.0),
                (x: 1.0, y: 3.0),
                (x: 2.0, y: 3.0),
                (x: 2.0, y: 2.0),
                (x: 3.0, y: 2.0),
                (x: 3.0, y: 0.0),
                (x: 2.0, y: 0.0),
                (x: 2.0, y: 1.0),
                (x: 1.0, y: 1.0),
                (x: 1.0, y: 0.0),
                (x: 0.0, y: 0.0),
            ],
            Shape::LongPlus => polygon![
                (x: 0.0, y: 0.0),
                (x: 0.0, y: 2.0),
                (x: -1.0, y: 2.0),
                (x: -1.0, y: 3.0),
                (x: 0.0, y: 3.0),
                (x: 0.0, y: 5.0),
                (x: 1.0, y: 5.0),
                (x: 1.0, y: 3.0),
                (x: 2.0, y: 3.0),
                (x: 2.0, y: 2.0),
                (x: 1.0, y: 2.0),
                (x: 1.0, y: 0.0),
                (x: 0.0, y: 0.0),
            ],
            Shape::FatPlus => polygon![
                (x: 0.0, y: 0.0),
                (x: 0.0, y: 1.0),
                (x: -1.0, y: 1.0),
                (x: -1.0, y: 3.0),
                (x: 0.0, y: 3.0),
                (x: 0.0, y: 4.0),
                (x: 1.0, y: 4.0),
                (x: 1.0, y: 3.0),
                (x: 2.0, y: 3.0),
                (x: 2.0, y: 1.0),
                (x: 1.0, y: 1.0),
                (x: 1.0, y: 0.0),
                (x: 0.0, y: 0.0),
            ],
            Shape::LongL => polygon![
                (x: 0.0, y: 0.0),
                (x: 0.0, y: 4.0),
                (x: 1.0, y: 4.0),
                (x: 1.0, y: 1.0),
                (x: 2.0, y: 1.0),
                (x: 2.0, y: 0.0),
                (x: 0.0, y: 0.0),
            ],
            Shape::BlueL => polygon![
                (x: 0.0, y: 0.0),
                (x: 0.0, y: 3.0),
                (x: 1.0, y: 3.0),
                (x: 1.0, y: 1.0),
                (x: 2.0, y: 1.0),
                (x: 2.0, y: 0.0),
                (x: 0.0, y: 0.0),
            ],
            Shape::Step => polygon![
                (x: 0.0, y: 0.0),
                (x: 0.0, y: 2.0),
                (x: 1.0, y: 2.0),
                (x: 1.0, y: 3.0),
                (x: 2.0, y: 3.0),
                (x: 2.0, y: 1.0),
                (x: 1.0, y: 1.0),
                (x: 1.0, y: 0.0),
                (x: 0.0, y: 0.0),
            ],
            Shape::LongT => polygon![
                (x: 0.0, y: 0.0),
                (x: 0.0, y: 3.0),
                (x: -1.0, y: 3.0),
                (x: -1.0, y: 4.0),
                (x: 2.0, y: 4.0),
                (x: 2.0, y: 3.0),
                (x: 1.0, y: 3.0),
                (x: 1.0, y: 0.0),
                (x: 0.0, y: 0.0),
            ],
            Shape::LongI => polygon![
                (x: 0.0, y: 0.0),
                (x: 0.0, y: 4.0),
                (x: 1.0, y: 4.0),
                (x: 1.0, y: 0.0),
                (x: 0.0, y: 0.0),
            ],
            Shape::HalfCross => polygon![
                (x: 0.0, y: 0.0),
                (x: 0.0, y: 2.0),
                (x: -1.0, y: 2.0),
                (x: -1.0, y: 3.0),
                (x: 0.0, y: 3.0),
                (x: 0.0, y: 4.0),
                (x: 1.0, y: 4.0),
                (x: 1.0, y: 3.0),
                (x: 2.0, y: 3.0),
                (x: 2.0, y: 2.0),
                (x: 1.0, y: 2.0),
                (x: 1.0, y: 0.0),
                (x: 0.0, y: 0.0),
            ],
            Shape::StripedStep => polygon![
                (x: 0.0, y: 0.0),
                (x: 0.0, y: 2.0),
                (x: 1.0, y: 2.0),
                (x: 1.0, y: 3.0),
                (x: 2.0, y: 3.0),
                (x: 2.0, y: 1.0),
                (x: 1.0, y: 1.0),
                (x: 1.0, y: 0.0),
                (x: 0.0, y: 0.0),
            ],
        }
    }

    /// Returns the pattern of a shape.
    pub(crate) fn pattern(&self) -> ColoredString {
        match self {
            Shape::Start => "󱨎".bright_black().on_truecolor(170, 200, 160),
            Shape::T => "󰓦".white().on_truecolor(170, 185, 95),
            Shape::L => "󰴈".green().on_truecolor(195, 80, 80),
            Shape::I => "󰣐".red().on_truecolor(220, 220, 180),
            Shape::U => "󰣏".truecolor(55, 55, 35).on_truecolor(170, 200, 160),
            Shape::SpaceInvader => "󱨧".red().on_truecolor(190, 185, 145),
            Shape::LongPlus => " ".on_red(),
            Shape::FatPlus => "".truecolor(30, 60, 110).on_truecolor(95, 155, 220),
            Shape::LongL => "󱀝".bright_black().on_truecolor(190, 180, 175),
            Shape::BlueL => "━".truecolor(30, 60, 110).on_truecolor(95, 170, 210),
            Shape::Step => "".red().on_magenta(),
            Shape::LongT => "x".green().on_truecolor(105, 210, 205),
            Shape::LongI => "".bright_black().on_truecolor(35, 60, 50),
            Shape::HalfCross => "".bright_black().on_truecolor(128, 128, 0),
            Shape::StripedStep => "󰞱".truecolor(157, 0, 0).on_bright_white(),
        }
    }
}
