use geo::point;

use quilting::patchwork::{Board, Patch};

fn main() {
    let mut board = Board::default();
    let patch = Patch::default();

    board.place(point! { x: 0, y: 0}, patch);
    println!("{board}")
}
