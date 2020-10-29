use crate::toy_box::Table;

impl Table {
    pub fn table(&mut self) -> Vec<String> {
        let mut vec = Vec::<String>::new();
        vec.push(format!("{}", self.header()));
        vec.push(format!("{}", self.row1()));
        for i in 0..self.height {
            vec.push(format!("{}", self.row2(i * self.width)));
            vec.push(format!("{}", self.row1()));
        }
        vec
    }

    fn row1(&mut self) -> String {
        let mut s = String::new();
        s.push_str("+");
        for _i in 0..self.width {
            s.push_str("----+");
        }
        s.push_str("");
        s
    }

    fn row2(&mut self, first_index: usize) -> String {
        let mut s = String::new();
        s.push_str("|");
        for i in 0..self.width {
            s.push_str(&format!("{: >4}|", self.cells[first_index + i].get_index()));
        }
        s.push_str("");
        s
    }

    fn header(&mut self) -> String {
        let mut s = String::new();
        s.push_str(" ");
        for i in 0..self.width {
            s.push_str(&format!("{: >4} ", self.column_labels[i]));
        }
        s.push_str("");
        s
    }
}
