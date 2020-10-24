use crate::toy_box::Cell;

impl Cell {
    pub fn new(index: usize) -> Self {
        Cell { index: index }
    }

    pub fn get_index(&self) -> usize {
        self.index
    }
}
