// Dependencies go here
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::EventPump;



// this function initializes the canvas
fn init<'a>(width: i32, height: i32) -> (Canvas<Window>, EventPump) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Rusty Snake", width as u32 + 1, height as u32 + 1)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let event_pump = sdl_context.event_pump().unwrap();
    (canvas, event_pump)
}

// this is main
fn main() {
    let canvas_width = 200;
    let canvas_height = 200;

    let (mut canvas, mut events) = init(canvas_width, canvas_height);

    loop {
        for event in events.poll_iter() {
            // handle user input here
        }
        // canvas.set_draw_color(Color::RGB(0, 0, 0));
        // canvas.clear();
        // canvas.present();
    }

}
