use crate::toy_box::Cell;
use crate::toy_box::Table;

impl Default for Table {
    fn default() -> Self {
        Table {
            width: 0,
            height: 0,
            cells: Vec::new(),
        }
    }
}

impl Table {
    pub fn print(&mut self) {
        self.print_row1();
        for i in 0..self.height {
            self.print_row2(i * self.width);
            self.print_row1();
        }
    }

    pub fn set_size(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        self.cells = Vec::new();
        for i in 0..self.width * self.height {
            self.cells.push(Cell::new(i));
        }
    }

    fn print_row1(&mut self) {
        print!("+");
        for _i in 0..self.width {
            print!("----+");
        }
        println!("");
    }

    fn print_row2(&mut self, first_index: usize) {
        print!("|");
        for i in 0..self.width {
            print!("{: >4}|", self.cells[first_index + i].get_index());
        }
        println!("");
    }
}
