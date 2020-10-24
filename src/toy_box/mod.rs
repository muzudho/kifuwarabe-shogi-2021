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
    pub width: usize,
    pub height: usize,
    pub cell: Vec<Cell>,
}

/// Cell.  
/// セル。  
pub struct Cell {}
