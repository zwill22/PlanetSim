mod body;
mod data;
mod settings;

extern crate approx;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use crate::data::initialise;
use crate::settings::Settings;
use body::Body;
use glutin_window::GlutinWindow as Window;
use graphics::clear;
use graphics::color::BLACK;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::window::WindowSettings;
use piston::{
    Button, ButtonArgs, ButtonEvent, ButtonState, Key, RenderArgs, RenderEvent, UpdateArgs,
    UpdateEvent,
};

// ******** Global constants ********
const OPENGL: OpenGL = OpenGL::V4_5;
const TITLE: &str = "Solar System";
const FULLSCREEN: bool = false;
const SAMPLES: u8 = 0;

const WIDTH: f64 = 3072.0;
const HEIGHT: f64 = 2048.0;
const EXIT_ON_ESCAPE: bool = true;

// **********************************

struct App {
    gl: GlGraphics,
    bodies: Vec<Body>,
    settings: Settings,
}

impl App {
    fn new(opengl: OpenGL) -> App {
        let settings = Settings::default();
        App {
            gl: GlGraphics::new(opengl),
            bodies: initialise(&settings),
            settings,
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        let x0 = args.window_size[0] / 2.0;
        let y0 = args.window_size[1] / 2.0;

        self.gl.draw(args.viewport(), |c, g| {
            clear(BLACK, g);
            
            self.bodies.iter().for_each(|body| body.render(&c, g, x0, y0))
        })
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.bodies.iter_mut().for_each(|body| body.update(&self.settings, args));
    }

    fn key_control(&mut self, key: &Key) {
        match key {
            Key::LeftBracket => self.settings.zoom_out(),           // `[` Zoom out
            Key::RightBracket => self.settings.zoom_in(),           // `]` Zoom in
            Key::Comma => self.settings.slow_down(),                // `,` Slow down
            Key::Period => self.settings.speed_up(),                // `.` Speed up
            Key::Quote => self.settings.decrease_planet_size(),     // `'` Decrease sizes
            Key::Backslash => self.settings.increase_planet_size(), // `\` Increase sizes
            Key::O => self.settings.toggle_orbits(),                // `o` Toggle orbits
            _ => {},
        }
    }

    fn control(&mut self, args: &ButtonArgs) {
        if args.state == ButtonState::Press {
            match args.button {
                Button::Keyboard(key) => self.key_control(&key),
                Button::Mouse(_) => {}
                Button::Controller(_) => {}
                Button::Hat(_) => {}
            }
        }
    }
}

fn main() {
    let mut window: Window = WindowSettings::new(TITLE, [WIDTH, HEIGHT])
        .graphics_api(OPENGL)
        .exit_on_esc(EXIT_ON_ESCAPE)
        .fullscreen(FULLSCREEN)
        .samples(SAMPLES)
        .build()
        .unwrap();

    let mut app = App::new(OPENGL);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        if let Some(args) = e.button_args() {
            app.control(&args);
        }
    }
}
