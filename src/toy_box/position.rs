use crate::toy_box::{Position, Table};

impl Default for Position {
    fn default() -> Self {
        Position {
            table: Table::default(),
        }
    }
}

impl Position {
    pub fn print(&mut self) {
        println!(
            "{}",
            self.table.table().join(
                "
"
            )
        );
    }
}
