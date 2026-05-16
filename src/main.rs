mod body;
mod constants;
mod data;
mod settings;

extern crate approx;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use crate::constants::*;
use crate::data::initialise;
use crate::settings::Settings;
use approx::abs_diff_ne;
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

            for body in &self.bodies {
                body.render(&self.settings, &c, g, x0, y0);
            }
        })
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.bodies.iter_mut().for_each(|body| body.update(args));
    }

    fn control(&mut self, args: &ButtonArgs) {
        let mut factor = 1.0;

        if args.state == ButtonState::Press {
            if args.button == Button::Keyboard(Key::LeftBracket) {
                factor /= ZOOM_FACTOR;
            }
            if args.button == Button::Keyboard(Key::RightBracket) {
                factor *= ZOOM_FACTOR;
            }
            if args.button == Button::Keyboard(Key::O) {
                self.settings.show_orbits = !self.settings.show_orbits;
            }
        }

        if abs_diff_ne!(factor, 1.0) {
            self.settings.scale_factor *= factor;
            self.bodies.iter_mut().for_each(|body| body.zoom(factor));
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
