use geo::point;

use quilting::patchwork::{Board, Patch, Shape};

fn main() {
    let mut board = Board::default();

    let patch = Patch::from_shape(Shape::StripedStep);
    board.place(point! { x: 3, y: 3}, patch).unwrap();
    println!("{board}")
}
