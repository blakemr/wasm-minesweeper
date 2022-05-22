use std::fmt::Display;
use std::fmt::Write;

use rand::{prelude::*, seq::IteratorRandom};

// Not unicode (cmd is being annoying)
const CELL_HIDDEN: char = 'O';
const CELL_MINE: char = 'X';
const CELL_VALUE: [char; 9] = [
    '_', // 0 - empty box
    '1', // 1
    '2', // 2
    '3', // 3
    '4', // 4
    '5', // 5
    '6', // 6
    '7', // 7
    '8', // 8
];

// Unicode
// const CELL_HIDDEN: char = '\u{1568}';
// const CELL_MINE: char = '\u{1500}';
// const CELL_VALUE: [char; 9] = [
//     '\u{1564}', // 0 - empty box
//     '\u{1502}', // 1
//     '\u{1503}', // 2
//     '\u{1504}', // 3
//     '\u{1505}', // 4
//     '\u{1506}', // 5
//     '\u{1507}', // 6
//     '\u{1508}', // 7
//     '\u{1509}', // 8
// ];

pub enum CellValue {
    Mine,
    Value(u8),
}

#[derive(Clone, Debug)]
struct MineCell {
    hidden: bool,
    flagged: bool,
    mine: bool,
}

impl MineCell {
    pub fn new() -> MineCell {
        MineCell {
            hidden: true,
            flagged: false,
            mine: false,
        }
    }
}

#[derive(Debug)]
pub struct Minesweeper {
    width: usize,
    board: Vec<MineCell>,
}

impl Minesweeper {
    pub fn new(width: usize, height: usize, mine_count: usize) -> Minesweeper {
        let mut rng = thread_rng();

        Minesweeper {
            width,
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
    pub fn unhide(&mut self, x: usize, y: usize) -> CellValue {
        let cell = &mut self.board[(y * self.width) + x];

        cell.hidden = false;

        if cell.mine {
            CellValue::Mine
        } else {
            CellValue::Value(self.adjacent_mines(x, y))
        }
    }

    fn cell(&self, x: usize, y: usize) -> &MineCell {
        &self.board[(y * self.width) + x]
    }

    fn adjacent_mines(&self, x: usize, y: usize) -> u8 {
        {
            self.cell(x - 1, y - 1).mine as u8
                + self.cell(x + 0, y - 1).mine as u8
                + self.cell(x + 1, y - 1).mine as u8
                + self.cell(x + 0, y - 1).mine as u8
                + self.cell(x + 0, y + 1).mine as u8
                + self.cell(x - 1, y + 1).mine as u8
                + self.cell(x + 0, y + 1).mine as u8
                + self.cell(x + 1, y + 1).mine as u8
        }
    }
}

impl Display for Minesweeper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, cell) in self.board.iter().enumerate() {
            if i % self.width == 0 {
                f.write_char('\n')?;
            }

            if cell.hidden {
                f.write_char(CELL_HIDDEN)?;
            } else if cell.mine {
                f.write_char(CELL_MINE)?;
            } else {
                f.write_char(
                    CELL_VALUE[self.adjacent_mines(i % self.width, i / self.width) as usize],
                )?;
            }
        }

        f.write_char('\n')?;
        Ok(())
    }
}
