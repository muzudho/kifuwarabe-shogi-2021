mod board;
mod position;

/// Position.  
/// 局面。  
pub struct Position {
    pub board: Board,
}

/// Board.  
/// 盤。  
pub struct Board {
    pub width: usize,
    pub height: usize,
}
