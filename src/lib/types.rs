//Data types
pub struct Cell {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    // later :pixel_type: i32,
}

pub struct Grid {
    pub grid: Vec<Vec<Cell>>,
}

pub struct Snake {
    pub tail: Vec<SnakeHead>,
}

pub struct SnakeHead {
    pub row: i32,
    pub column: i32,
    pub color: Cell,
}
