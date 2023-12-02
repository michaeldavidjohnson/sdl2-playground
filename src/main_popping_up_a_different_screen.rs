extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::WindowContext;
use sdl2::ttf::Font;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

struct InventoryScreen<'a> {
    font: Font<'a, 'static>,
}

impl<'a> InventoryScreen<'a> {
    fn new(ttf_context: &'a sdl2::ttf::Sdl2TtfContext) -> Self {
        let font = ttf_context
            .load_font("./square.ttf", 36)
            .expect("Failed to load font");

        Self { font }
    }

    fn render(
        &self,
        canvas: &mut Canvas<sdl2::video::Window>,
        x: i32,
        y: i32,
        content: &str,
    ) {
        // Set draw color to light gray
        canvas.set_draw_color(Color::RGB(200, 200, 200));
        // Draw the background rectangle
        canvas.fill_rect(Rect::new(x, y, SCREEN_WIDTH, SCREEN_HEIGHT)).unwrap();

        // Set draw color to black for text
        canvas.set_draw_color(Color::RGB(0, 0, 0));

        // Render the text onto the canvas
        let surface = self.font.render(content).blended(Color::RGB(0, 0, 0)).unwrap();
        let texture_cerator = canvas.texture_creator();
        let texture = texture_cerator.create_texture_from_surface(&surface).unwrap();

        let texture_query = texture.query();
        let target = Point::new(
            x + (SCREEN_WIDTH as i32 - texture_query.width as i32)  / 2,
            y + (SCREEN_HEIGHT as i32 - texture_query.height as i32) / 2,
        );

        let dest_rect = Rect::new(target.x, target.y, texture_query.width, texture_query.height);
        canvas.copy(&texture, None, dest_rect).unwrap();
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();

    let window = video_subsystem
        .window("Inventory Screen", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let inventory_screen = InventoryScreen::new(&ttf_context);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                // Add an event to open the inventory screen (e.g., press 'I' key)
                Event::KeyDown {
                    keycode: Some(Keycode::I),
                    ..
                } => {
                    // Open the inventory screen with custom content
                    inventory_screen.render(
                        &mut canvas,
                        0,
                        0,
                        "This is the inventory\nPress ESC or I to close",
                    );
                    canvas.present();
                }
                _ => {}
            }
        }

        // Clear the screen
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        // Render other game elements here

        // Present the canvas
        canvas.present();
        
    }
}
