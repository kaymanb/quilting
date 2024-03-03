use geo::point;

use quilting::patchwork::{Board, Patch, Shape};

fn main() {
    let mut board = Board::default();

    let start = Patch::from_shape(Shape::Start);
    board.place(point! { x: 0, y: 0}, start).unwrap();

    let tee = Patch::from_shape(Shape::Tee);
    board.place(point! { x: 2, y: 0}, tee).unwrap();

    let ell = Patch::from_shape(Shape::Ell);
    board.place(point! { x: 0, y: 1}, ell).unwrap();
    println!("{board}")
}
