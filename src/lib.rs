use rand::prelude::*;

#[derive(Debug, Clone)]
pub enum Tile
{
    Near(usize),
    HiddenMine(bool),
    Flagged(Box<Tile>),
    Mined
}
impl Tile
{
    pub fn display(&self)
    {
        match self
        {
            Tile::Near(n) =>
            {
                print!("[{}]", n)
            },
            Tile::HiddenMine(_) =>
            {
                print!("[-]")
            },
            Tile::Flagged(_) =>
            {
                print!("[P]")
            },
            Tile::Mined =>
            {
                print!("-X-")
            }
        }
    }
}

pub struct Board
{
    board:Vec<Vec<Tile>>
}
impl Board
{
    pub fn new(x_size:&usize, y_size:&usize, mines:&usize)
    -> Result<Board, &'static str>
    {
        let mut board = Board{ board:vec!() };
        let successful = board.generate(x_size, y_size, mines);
        if successful
        {
            return Ok(board)
        }
        else
        {
            return Err("Bad arguments")
        }
    }
    pub fn display(&self)
    {
        for x in 0..self.board.len()
        {
            for y in 0..self.board[x].len()
            {
                self.board[x][y].display();
            }
            println!("");
        }
    }
    fn generate(&mut self, x_size:&usize, y_size:&usize, mines:&usize)
    -> bool
    {
        let tiles_size = x_size * y_size;
        let mut tiles = vec!();
        for _mine in 0..*mines
        {
            tiles.push(Tile::HiddenMine(true))
        }
        if tiles.len() > tiles_size
        {
            return false;
        }
        while tiles.len() < tiles_size
        {
            tiles.push(Tile::HiddenMine(false))
        }
        let mut rng = rand::thread_rng();
        tiles.shuffle(&mut rng);
        for _x in 0..*x_size
        {
            let mut column = vec!();
            for _y in 0..*y_size
            {
                column.push(tiles.pop().unwrap());
            }
            self.board.push(column);
        }
        return true;
    }
    fn touch_tile(&mut self, x:&usize, y:&usize)
    -> bool
    {
        match &self.board[*x][*y]
        {
            Tile::Near(_) =>
            {
                return false;
            },
            Tile::HiddenMine(value) =>
            {
                match value
                {
                    true =>
                    {
                        self.board[*x][*y] = Tile::Mined;
                        return true;
                    },
                    false =>
                    {
                        self.board[*x][*y] = Tile::Near(self.find_nearby(&x, &y));
                        return false;
                    },
                }
            },
            Tile::Flagged(_) =>
            {
                return false;
            },
            Tile::Mined =>
            {
                return true;
            }
        }
    }
    fn find_nearby(&self, origin_x:&usize, origin_y:&usize)
    -> usize
    {
        let mut mines:usize = 0;
        for dx in -1..=1
        {
            let x = (*origin_x as isize + dx) as usize;
            for dy in -1..=1
            {
                let y = (*origin_y as isize + dy) as usize;
                let is_mine = match &self.board[x][y]
                {
                    Tile::Mined =>
                    {
                        true
                    },
                    Tile::HiddenMine(true) =>
                    {
                        true
                    },
                    Tile::Flagged(boxed) =>
                    {
                        match **boxed
                        {
                            Tile::Mined =>
                            {
                                true
                            },
                            Tile::HiddenMine(true) =>
                            {
                                true
                            },
                            _ =>
                            {
                                false
                            }
                        }
                    },
                    _ =>
                    {
                        false
                    },
                };
                if is_mine
                {
                    mines += 1;
                }
            }
        }
        return mines;
    }
}