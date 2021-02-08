// Dependencies go here
use ggez::{
    conf::WindowMode,
    event::EventsLoop,
    graphics::{self, Color, DrawMode, DrawParam, Mesh, Rect},
    Context, ContextBuilder,
};

pub mod snake;
pub mod types;

use types::{Cell, Grid};

// this function initializes the canvas
pub fn init(width: f32, height: f32) -> (Context, EventsLoop) {
    ContextBuilder::new("Snake", "<your name here>")
        .window_mode(WindowMode::default().dimensions(width, height))
        .build()
        .unwrap()
}

//creates a grid with ncells*ncells initialized with cell in a color
pub fn grid_init(nx_cells: u32, ny_cells: u32) -> Grid {
    let mut grid_vector = Vec::new();

    for row in 0..ny_cells {
        grid_vector.push(Vec::new());
        for _column in 0..nx_cells {
            grid_vector[row as usize].push(Cell {
                red: 35_u8,
                green: 15_u8,
                blue: 13_u8,
                // pixel_type: 0,
            });
        }
    }
    let grid = Grid { grid: grid_vector };

    grid
}

//converts row column values into xy pixels and draws rectangle in the specified color
pub fn display_cell(
    renderer: &mut Context,
    row: u32,
    col: u32,
    grid_data: &Grid,
    cell_width: &u32,
) {
    let cell_height = cell_width;

    let grid = &grid_data.grid;

    let x = cell_width * col;
    let y = cell_width * row;

    //For now, we want random colors, to see what happens.
    let cell_color = &grid[row as usize][col as usize];
    let drawing_color = Color::from_rgb(cell_color.red, cell_color.green, cell_color.blue);

    // let red: u8 = rand::random();
    // let green: u8 = rand::random();
    // let blue: u8 = rand::random();
    //
    // let drawing_color = Color::RGB(red, green, blue);

    let rect = Rect::new(x as f32, y as f32, *cell_width as f32, *cell_height as f32);
    let mesh = Mesh::new_rectangle(renderer, DrawMode::fill(), rect, drawing_color).unwrap();
    graphics::draw(renderer, &mesh, DrawParam::new()).unwrap();
}

//displays the whole grid by repeatedly calling display_cell on every cell
pub fn display_frame(
    renderer: &mut Context,
    grid: &Grid,
    nx_cells: &u32,
    ny_cells: &u32,
    cell_width: &u32,
) {
    graphics::clear(renderer, Color::from_rgb(0, 0, 0));

    for row in 0..*ny_cells {
        for column in 0..*nx_cells {
            display_cell(renderer, row, column, &grid, &cell_width);
        }
    }

    graphics::present(renderer).unwrap();
}

pub fn clear_grid(grid: &Grid) -> Grid {
    println!("clearing grid");

    let max_rows = &grid.grid.len();
    let max_columns = &grid.grid[0].len();

    let mut grid_vector = Vec::new();

    for row in 0..*max_rows as i32 {
        grid_vector.push(Vec::new());
        for _column in 0..*max_columns as i32 {
            grid_vector[row as usize].push(Cell {
                red: 35_u8,
                green: 15_u8,
                blue: 13_u8,
            });
        }
    }

    let grid = Grid { grid: grid_vector };

    grid
}
