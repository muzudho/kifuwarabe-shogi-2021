use crate::toy_box::{Board, Position};

impl Default for Position {
    fn default() -> Self {
        Position {
            board: Board::default(),
        }
    }
}

impl Position {
    pub fn print(&mut self) {
        println!("Hey!");
        self.board.print();
    }
}
