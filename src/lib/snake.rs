use crate::lib::types::{Cell, SnakeHead, Grid};

pub fn snake_init() -> SnakeHead {
    let snake = SnakeHead {
                row: 6,
                column: 6,
                color: Cell {
                    red: 200_u8,
                    green: 0_u8,
                    blue: 100_u8,
                    }
                };
    snake
}


pub fn snake_moves(snake: &mut SnakeHead, direction: (i32, i32)) -> SnakeHead {
    snake.row = snake.row + direction.0 ;
    snake.column = snake.column + direction.1;


    let new_snake = SnakeHead {
        row: snake.row,
        column: snake.column,
        color: Cell {
            red: 200_u8,
            green: 0_u8,
            blue: 100_u8,
            }

    };

    new_snake
}

pub fn draw_grid_with_snake(mut grid: Grid, snake: &SnakeHead) -> Grid {

    let color = Cell {
        red: snake.color.red,
        green: snake.color.green,
        blue: snake.color.green,
    };


    grid.grid[snake.row as usize][snake.column as usize] = color;
    grid
}
