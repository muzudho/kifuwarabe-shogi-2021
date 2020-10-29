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

        // 右側にラベルを追加する９行。
        for (i, row) in [2, 4, 6, 8, 10, 12, 14, 16, 18].iter().enumerate() {
            vec[*row] = format!("{} {: <4}", vec[*row], self.row_labels[i]);
        }
        // 右側にラベルを追加しない行。
        for row in &[0, 1, 3, 5, 7, 9, 11, 13, 15, 17, 19] {
            vec[*row] = format!("{}     ", vec[*row]);
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
