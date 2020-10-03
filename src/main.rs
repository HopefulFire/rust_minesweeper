use rust_minesweeper::*;

fn main() {
    let board = Board::new(&10, &10, &10).unwrap();
    board.display();
}
