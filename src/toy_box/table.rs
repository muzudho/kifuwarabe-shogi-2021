use crate::toy_box::Cell;
use crate::toy_box::Table;

impl Default for Table {
    fn default() -> Self {
        Table {
            width: 0,
            height: 0,
            column_labels: Vec::new(),
            row_labels: Vec::new(),
            cells: Vec::new(),
        }
    }
}

impl Table {
    pub fn set_size(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        self.cells = Vec::new();
        for i in 0..self.width * self.height {
            self.cells.push(Cell::new(i));
        }
    }
}
