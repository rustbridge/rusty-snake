// Dependencies go here
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::EventPump;
use rand;

pub mod types;
pub mod snake;

use types::{Grid, Cell};

// this function initializes the canvas
pub fn init(width: u32, height: u32) -> (Canvas<Window>, EventPump) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Canvas", width + 1, height + 1)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();

    let event_pump = sdl_context.event_pump().unwrap();
    (canvas, event_pump)
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
    renderer: &mut Canvas<Window>,
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
    let drawing_color = Color::RGB(cell_color.red, cell_color.green, cell_color.blue);

    // let red: u8 = rand::random();
    // let green: u8 = rand::random();
    // let blue: u8 = rand::random();
    //
    // let drawing_color = Color::RGB(red, green, blue);

    renderer.set_draw_color(drawing_color);
    let square = renderer.fill_rect(Rect::new(x as i32, y as i32, *cell_width, *cell_height));
    match square {
        Ok(()) => {}
        Err(error) => println!("{}", error),
    }
}

//displays the whole grid by repeatedly calling display_cell on every cell
pub fn display_frame(
    renderer: &mut Canvas<Window>,
    grid: &Grid,
    nx_cells: &u32,
    ny_cells: &u32,
    cell_width: &u32,
) {


    renderer.set_draw_color(Color::RGB(0, 0, 0));
    renderer.clear();

    for row in 0..*ny_cells {
        for column in 0..*nx_cells {
            display_cell(renderer, row, column, &grid, &cell_width)
        }
    }
    renderer.present();
}

pub fn clear_grid(grid: &Grid) -> Grid {
    println!("clearing grid");

    let max_rows = &grid.grid.len();
    let max_columns = &grid.grid[0].len();

    let mut grid_vector = Vec::new();

    for row in 0..*max_rows as i32 {
        grid_vector.push(Vec::new());
        for _column in 0..*max_columns as i32 {
            grid_vector[row as usize].push(Cell{
                red: 35_u8,
                green: 15_u8,
                blue: 13_u8,
            });
        }
    }

    let grid = Grid { grid: grid_vector };

    grid

}
