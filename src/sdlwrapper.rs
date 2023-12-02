use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::libc::__c_anonymous_sockaddr_can_can_addr;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::ttf::{Font, Sdl2TtfContext};
use sdl2::video::Window;
use sdl2::{EventPump, Sdl, VideoSubsystem};

pub struct SDLContext {
    pub sdl_context: Sdl,
    pub ttf_context: Sdl2TtfContext,
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
    font_path: String,
}

impl SDLContext {
    pub fn new(font_path:&str) -> Self {
        let sdd_context = sdl2::init().unwrap();
        let video_subsystem = sdd_context.video().unwrap();
        let ttf_context = sdl2::ttf::init().unwrap();
        let EventPump = sdd_context.event_pump().unwrap();
        let window = video_subsystem
            .window("Title", 800, 600)
            .position_centered()
            .resizable()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();

        SDLContext {
            sdl_context: sdd_context,
            event_pump: EventPump,
            canvas: canvas,
            ttf_context:ttf_context,
            font_path: font_path.to_string(),
        }
    }

    pub fn render_text(&mut self, text: &str, x: i32, y: i32) {
        let font = self.ttf_context.load_font(&self.font_path, 36).unwrap();
        let surface = font.render(text).blended(Color::RGB(0, 0, 0)).unwrap();
        let texture_creator = self.canvas.texture_creator();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();
        let target_rect = Rect::new(x, y, surface.width(), surface.height());
        self.canvas.copy(&texture, None, target_rect).unwrap();
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.canvas.clear();
    }

    pub fn poll_events(&mut self) -> Vec<Event> {
        self.event_pump.poll_iter().collect()
    }

    pub fn draw_random_square(&mut self) {
        let x = rand::random::<i32>() % 800;
        let y = rand::random::<i32>() % 500;
        let size = rand::random::<u32>() % 50 + 10;
        let color = Color::RGB(10, 10, 10);

        self.canvas.set_draw_color(color);
        self.canvas.fill_rect(Rect::new(x, y, size, size)).unwrap();
    }
}
