mod minesweeper;
use minesweeper::Minesweeper;

#[test]
pub fn test() {
    let mines = Minesweeper::new(10, 10, 10);

    print!("{:?}", mines);
}
