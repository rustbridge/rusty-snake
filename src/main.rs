// Dependencies go here
use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::{event::EventHandler, Context, GameResult};
use std::{thread, time};

use crate::lib::{snake, types};

pub mod lib;

struct Game {
    grid: types::Grid,
    snake: types::SnakeHead,
    direction: (i32, i32),
    columns: u32,
    rows: u32,
    cell_width: u32,
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        snake::snake_moves(&mut self.snake, self.direction);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        snake::draw_grid_with_snake(&mut self.grid, &self.snake);
        lib::display_frame(ctx, &self.grid, &self.columns, &self.rows, &self.cell_width);
        thread::sleep(time::Duration::from_millis(500));
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Escape => ggez::event::quit(ctx),
            KeyCode::Up => {
                self.direction = (-1, 0);
            }
            KeyCode::Down => {
                self.direction = (1, 0);
            }
            KeyCode::Left => {
                self.direction = (0, -1);
            }
            KeyCode::Right => {
                self.direction = (0, 1);
            }
            _ => {}
        }
    }
}

// this is main
fn main() {
    let canvas_width = 500;
    let canvas_height = 500;

    let columns = 12;
    let rows = 12;

    let cell_width = canvas_width / columns;

    let (mut canvas, mut events) = lib::init(canvas_width as f32, canvas_height as f32);
    let grid = lib::grid_init(columns, rows);
    let snake = snake::snake_init();

    let mut game = Game {
        grid,
        snake,
        direction: (0, 1),
        columns,
        rows,
        cell_width,
    };

    thread::spawn(move || {
        // some work here
    });

    ggez::event::run(&mut canvas, &mut events, &mut game).unwrap();
}
