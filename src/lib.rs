mod minesweeper;
use minesweeper::Minesweeper;

#[test]
pub fn test() {
    let mut mines = Minesweeper::new(10, 10, 10);

    mines.unhide(2, 3);

    print!("{}", mines);
}
