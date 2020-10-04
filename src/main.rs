use rust_minesweeper::*;

fn main() {
    match Board::new(&10, &10, &10)
    {
        Ok(board) =>
        {
            board.display();
            board.touch_tile(&4, &4);
            board.display();
        }
        Err(message) =>
        {
            println!("{}", message);
        }
    }
}