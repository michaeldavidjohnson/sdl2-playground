use sdl2::keyboard;

mod sdlwrapper;

struct GameLogic {}

impl GameLogic {
    fn new() -> Self {
        Self {}
    }

    fn handle_event(&mut self, _event: &sdl2::event::Event) {}

    fn update(&mut self, sdl_context: &mut sdlwrapper::SDLContext) {
        for i in 0..110 {
            sdl_context.draw_random_square();
            sdl_context.present();
        }
        
    }

    fn render(&self, sdl_context: &mut sdlwrapper::SDLContext) {
        sdl_context.clear();
        sdl_context.render_text("hmmm", 100, 50);
        sdl_context.draw_random_square();
        sdl_context.present();
    }
}

fn crazy_main() {
    let mut sdl_context = sdlwrapper::SDLContext::new("./square.ttf");
    let mut game = GameLogic::new();

    'running: loop {
        for event in sdl_context.poll_events() {
            match event {
                sdl2::event::Event::Quit { .. }
                | sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => break 'running,
             _ => game.handle_event(&event),
        }
    }
    
    game.update(&mut sdl_context);
    game.render(&mut sdl_context);
    std::thread::sleep(std::time::Duration::from_millis(16));
    }
}
