use geo::point;

use quilting::patchwork::{Board, Patch, Shape};

fn main() {
    let mut board = Board::default();

    let start = Patch::from_shape(Shape::Start);
    board.place(point! { x: 0, y: 0}, start);

    let tee = Patch::from_shape(Shape::Tee);
    board.place(point! { x: 2, y: 0}, tee);

    let ell = Patch::from_shape(Shape::Ell);
    board.place(point! { x: 0, y: 1}, ell);
    println!("{board}")
}
