use rand::{prelude::*, seq::IteratorRandom};

#[derive(Clone, Debug)]
struct MineCell {
    hidden: bool,
    flagged: bool,
    mine: bool,
}

impl MineCell {
    pub fn new() -> MineCell {
        MineCell {
            hidden: false,
            flagged: false,
            mine: false,
        }
    }
}

#[derive(Debug)]
pub struct Minesweeper {
    width: usize,
    height: usize,
    board: Vec<MineCell>,
}

impl Minesweeper {
    pub fn new(width: usize, height: usize, mine_count: usize) -> Minesweeper {
        let mut rng = thread_rng();

        Minesweeper {
            width,
            height,
            board: {
                let mut board = vec![MineCell::new(); width * height];

                let sample = (0..width * height).choose_multiple(&mut rng, mine_count);

                for cell in sample {
                    board[cell].mine = true;
                }

                board
            },
        }
    }
}
