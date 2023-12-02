use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::ttf::{Font, Sdl2TtfContext};

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

fn main() {
    // Initialize SDL2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // Initialize SDL2_ttf
    let ttf_context = sdl2::ttf::init().unwrap();

    // Create a window
    let window = video_subsystem
        .window("SDL2 Rust Text Rendering", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    // Create a canvas
    let mut canvas = window.into_canvas().build().unwrap();

    // Load a font
    let font_path = "./square.ttf";
    let font_size = 36;
    let font = ttf_context.load_font(font_path, font_size).unwrap();

    // Main event loop
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // Clear the canvas
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        // Render text
        render_text(
            &mut canvas,
            &font,
            "Hello, SDL2!",
            Point::new(100, 100),
            Color::RGB(0, 0, 0),
        );

        // Present the canvas
        canvas.present();
    }
}

// Function to render text on the canvas
fn render_text(canvas: &mut Canvas<Window>, font: &Font, text: &str, position: Point, color: Color) {
    let surface = font
        .render(text)
        .blended(color)
        .map_err(|e| e.to_string())
        .unwrap();

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.create_texture_from_surface(&surface).unwrap();

    let target_rect = sdl2::rect::Rect::new(position.x, position.y, surface.width(), surface.height());

    canvas.copy(&texture, None, target_rect).unwrap();
}
