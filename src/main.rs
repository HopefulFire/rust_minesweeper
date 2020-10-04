use rust_minesweeper::*;

fn main() {
    match Board::new(&10, &10, &10)
    {
        Ok(board) =>
        {
            board.display()
        }
        Err(message) =>
        {
            println!("{}", message);
        }
    }
}