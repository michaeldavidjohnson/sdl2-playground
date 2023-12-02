extern crate rand;
extern crate sdl2;

use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::ttf;

use rand::prelude::*;

const GRID_ROWS: i32 = 80;
const GRID_COLS: i32 = 80;
const FONT_SIZE: u16 = 10;

fn draw_dynamic_grid(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    loaded_texture: &mut Texture,
) {
    let sprite_width = 10;
    let sprite_height = 10;
    let square_width = canvas.output_size().unwrap().0 as i32 / GRID_COLS;
    let square_height = canvas.output_size().unwrap().1  as i32 / GRID_ROWS;

    for i in 0..(canvas.output_size().unwrap().0/square_width as u32) as i32{
        for j in 0..(canvas.output_size().unwrap().0/square_height as u32) as i32{

            let source_rect = Rect::new(rand::thread_rng().gen_range(0..16) * 10 % 160, rand::thread_rng().gen_range(0..16) * 10 % 160, sprite_width, sprite_height);
            let dest_rect = Rect::new(
                i * square_width,
                j * square_height,
                square_width as u32,
                square_height as u32,
            );
            canvas.set_draw_color(Color::RGB(
                rand::thread_rng().gen_range(0..255),
                rand::thread_rng().gen_range(0..255),
                rand::thread_rng().gen_range(0..255),
            ));
            canvas.fill_rect(dest_rect).unwrap();
            loaded_texture.set_color_mod(
                rand::thread_rng().gen_range(0..255),
                rand::thread_rng().gen_range(0..255),
                rand::thread_rng().gen_range(0..255),
            );
            canvas
                .copy(&loaded_texture, source_rect, dest_rect)
                .unwrap();
        }
    }
}

fn main() {
    // Initialize SDL2
    let sdl_context = sdl2::init().unwrap();
    let ttf_context = ttf::init().map_err(|e| e.to_string()).unwrap();

    // Set the initial window size
    let initial_width = 800;
    let initial_height = 400;

    // Create a window
    let window = sdl_context
        .video()
        .unwrap()
        .window(
            "Dynamic Grid Size",
            initial_width as u32,
            initial_height as u32,
        )
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    // Create a canvas
    let mut canvas = window.into_canvas().build().unwrap();

    let texture_creator = canvas.texture_creator();
    //let image_bytes =
    let path = std::path::Path::new("./Taffer_w_trans.png");
    let mut loaded_texture = texture_creator.load_texture(path).unwrap();

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

        canvas.clear();

        draw_dynamic_grid(
            &mut canvas,
            &mut loaded_texture,
        );

        canvas.present();

        //Cap the frame rate
        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}
