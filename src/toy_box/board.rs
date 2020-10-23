use crate::toy_box::Board;

impl Default for Board {
    fn default() -> Self {
        Board {
            width: 0,
            height: 0,
        }
    }
}

impl Board {
    pub fn print(&mut self) {
        self.print_row1();
        for _i in 0..self.height {
            self.print_row2();
            self.print_row1();
        }
    }

    fn print_row1(&mut self) {
        print!("+");
        for _i in 0..self.width {
            print!("----+");
        }
        println!("");
    }

    fn print_row2(&mut self) {
        print!("|");
        for _i in 0..self.width {
            print!("    |");
        }
        println!("");
    }
}
