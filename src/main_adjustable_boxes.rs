extern crate sdl2;
extern crate rand;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use rand::prelude::*;

const GRID_ROWS: i32 = 50;
const GRID_COLS: i32 = 80;

fn main() {
    // Initialize SDL2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // Set the initial window size
    let initial_width = GRID_COLS * 10;
    let initial_height = GRID_ROWS * 10;

    // Create a window
    let window = video_subsystem
        .window("Dynamic Grid Size", initial_width as u32, initial_height as u32)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    // Create a canvas
    let mut canvas = window.into_canvas().build().unwrap();

    // Generate random colors for each square
    let colors: Vec<Color> = (0..GRID_ROWS * GRID_COLS)
        .map(|_| Color::RGB(random_color(), random_color(), random_color()))
        .collect();

    // Main loop
    'running: loop {
        for event in sdl_context.event_pump().unwrap().poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // Clear the screen
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        // Draw the dynamic grid of colored squares
        draw_dynamic_grid(&mut canvas, &colors);

        // Present the canvas
        canvas.present();

        // Cap the frame rate
        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn draw_dynamic_grid(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    colors: &[Color],
) {
    // Get the dimensions of the window
    let (window_width, window_height) = canvas.output_size().unwrap();

    // Calculate the size of each square based on the window dimensions
    let square_width = window_width as i32 / GRID_COLS;
    let square_height = window_height as i32 / GRID_ROWS;

    for row in 0..GRID_ROWS {
        for col in 0..GRID_COLS {
            // Get the color for the current square
            let color = colors[(row * GRID_COLS + col) as usize];

            // Calculate square position
            let x = col as i32 * square_width;
            let y = row as i32 * square_height;

            // Draw the square
            canvas.set_draw_color(color);
            canvas
                .fill_rect(Rect::new(x, y, square_width as u32, square_height as u32))
                .unwrap();
        }
    }
}

fn random_color() -> u8 {
    rand::thread_rng().gen_range(0..255)
}
