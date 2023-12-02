extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::ttf::Font;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
const LOG_WIDTH: u32 = SCREEN_WIDTH / 3;
const FONT_SIZE: u16 = 18;
const MAX_LINES: usize = 26 as usize;

struct MessageLog<'a> {
    lines: Vec<String>,
    font: Font<'a, 'static>,
}



impl<'a> MessageLog<'a> {
    fn new(font: Font<'a, 'static>) -> MessageLog<'a> {
        MessageLog {
            lines: Vec::new(),
            font,
        }
    }

    fn add_message(&mut self, message: String) {
        println!("{:?}", self.lines);
        let mut splitting_point = message.len();
        for (index, _) in message.chars().enumerate() {
            if 18 * (index as u32) > SCREEN_WIDTH / 3 {
                splitting_point = index - 1
            }
        }

        if splitting_point == message.len() {
            self.lines.push(message);

        } else {
            let string_1 = message[..=splitting_point].to_string();
            self.lines.push(string_1);
            let string_2 = message[splitting_point..message.len()].to_string();
            self.add_message(string_2);

        }
        
        if self.lines.len() > MAX_LINES {
            self.lines.remove(0);
        }
    }

    fn render(
        &self,
        canvas: &mut Canvas<sdl2::video::Window>,
        texture_creator: &'a TextureCreator<sdl2::video::WindowContext>,
    ) {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas
            .fill_rect(Rect::new(
                (2 * SCREEN_WIDTH / 3) as i32,
                0,
                LOG_WIDTH,
                SCREEN_HEIGHT as u32,
            ))
            .unwrap();

        let mut y = 0;
        let mut line_iter = self.lines.iter().rev();

        for wrapped_line in self.lines.iter() {
            let mut texture = texture_creator
                .create_texture_from_surface(
                    self.font
                        .render(wrapped_line)
                        .blended(Color::WHITE)
                        .unwrap(),
                )
                .map_err(|e| e.to_string())
                .unwrap();

            let query = texture.query();
            let dest_rect = Rect::new((2 * SCREEN_WIDTH / 3) as i32, y, query.width, query.height);
            canvas.copy(&texture, None, dest_rect).unwrap();
            y += query.height as i32 + 5;
        }
    }

    // fn wrap_text(&self, text: &str) -> (String, &str) {
    //     let mut wrapped_line = String::new();
    //     let mut current_width = 0;

    //     for word in text.split_whitespace() {

    //         let Texture { query, .. } = self.font.render(word).blended(Color::WHITE).unwrap().query();
    //         current_width += query.width;

    //         if current_width <= LOG_WIDTH as u32 {
    //             wrapped_line.push_str(word);
    //             wrapped_line.push(' ');
    //         } else {
    //             return (wrapped_line, text);
    //         }
    //     }

    //     (wrapped_line, "")
    // }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();

    let window = video_subsystem
        .window("Message Log", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let font = ttf_context.load_font("./square.ttf", FONT_SIZE).unwrap();

    let mut message_log = MessageLog::new(font);

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Return),
                    ..
                } => {
                    message_log.add_message("This is a sample message!".to_owned());
                }
                _ => {}
            }
        }
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        // Render other components of your game/application here

        message_log.render(&mut canvas, &texture_creator);

        canvas.present();
    }
}
