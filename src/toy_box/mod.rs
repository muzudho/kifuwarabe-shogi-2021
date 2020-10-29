mod cell;
mod position;
mod table;
mod table_render;

/// Position.  
/// 局面。  
pub struct Position {
    pub table: Table,
}

/// Table.  
/// 表。  
pub struct Table {
    width: usize,
    height: usize,
    pub column_labels: Vec<String>,
    row_labels: Vec<String>,
    pub cells: Vec<Cell>,
}

/// Cell.  
/// セル。  
pub struct Cell {
    index: usize,
}
