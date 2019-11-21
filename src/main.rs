// Dependencies go here
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::{thread, time};

use crate::lib::snake;

pub mod lib;


// this is main
fn main() {
    let canvas_width = 720_u32;
    let canvas_height = 720_u32;

    let columns = 12_u32;
    let rows = 12_u32;

    let cell_width = canvas_width / columns;

    let (mut canvas, mut events) = lib::init(canvas_width, canvas_height);
    let mut grid = lib::grid_init(columns, rows);
    let mut snake = snake::snake_init();
    let mut direction = (0, 1);

    thread::spawn(move || {
        // some work here
        });

    'game: loop {
        for event in events.poll_iter() {

            match event {
                Event::KeyDown {
                   keycode: Some(Keycode::Up),
                   ..
               } => {
                   direction.0 = -1;
                   direction.1 = 0;
                   }

               Event::KeyDown {
                   keycode: Some(Keycode::Down),
                   ..
               } => {
                   direction.0 = 1;
                   direction.1 = 0;
                   }

               Event::KeyDown {
                   keycode: Some(Keycode::Left),
                   ..
               } => {
                   direction.1 = -1;
                   direction.0 = 0;
                   }

               Event::KeyDown {
                   keycode: Some(Keycode::Right),
                   ..
               } => {
                   direction.1 = 1;
                   direction.0 = 0;
                   }
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'game,

                _ => continue 'game,
            }
        }
        let snake = snake::snake_moves(&mut snake, direction);
        grid = snake::draw_grid_with_snake(grid, &snake);
        lib::display_frame(&mut canvas, &grid, &columns, &rows, &cell_width);
        //grid = lib::clear_grid(&grid);
        thread::sleep(time::Duration::from_millis(800));

    }
}
