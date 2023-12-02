extern crate sdl2;
extern crate rand;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::ttf;
use sdl2::image::LoadTexture;

use rand::prelude::*;

const GRID_ROWS: i32 = 50;
const GRID_COLS: i32 = 80;
const FONT_SIZE: u16 = 10;

fn main() {
    // Initialize SDL2
    let sdl_context = sdl2::init().unwrap();
    let ttf_context = ttf::init().map_err(|e| e.to_string()).unwrap();

    // Set the initial window size
    let initial_width = GRID_COLS * FONT_SIZE as i32;
    let initial_height = GRID_ROWS * FONT_SIZE as i32;

    // Create a window
    let window = sdl_context
        .video()
        .unwrap()
        .window("Dynamic Grid Size", initial_width as u32, initial_height as u32)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    // Create a canvas
    let mut canvas = window.into_canvas().software().build().unwrap();

    let texture_creator = canvas.texture_creator();
    //let image_bytes = 
    let path = std::path::Path::new("./Buddy.png");
    let mut loaded_texture = texture_creator.load_texture(path).unwrap();
    loaded_texture.set_color_mod(255, 0, 0);

    let sprite_width = 10;
    let sprite_heignt = 10;

    let source_rect = Rect::new(0,0,sprite_width,sprite_heignt);

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();

    canvas.copy(&loaded_texture,  None, None).unwrap();
    canvas.present();
    
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

        // Cap the frame rate
        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}
