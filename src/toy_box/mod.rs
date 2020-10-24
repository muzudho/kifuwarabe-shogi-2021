mod cell;
mod position;
mod table;

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
    pub cells: Vec<Cell>,
}

/// Cell.  
/// セル。  
pub struct Cell {
    index: usize,
}
