mod data;
mod body;
mod constants;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use graphics::color::{BLACK};
use graphics::{clear};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::window::WindowSettings;
use piston::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use body::Body;
use crate::constants::*;

fn initialise() -> Vec<Body> {
    data::initialise()
}

struct App {
    gl: GlGraphics,
    bodies: Vec<Body>,
}

impl App {
    fn new(opengl: OpenGL) -> App {
        App {
            gl: GlGraphics::new(opengl),
            bodies: initialise(),
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        let x0 = args.window_size[0] / 2.0;
        let y0 = args.window_size[1] / 2.0;

        self.gl.draw(args.viewport(), |c, g| {
            clear(BLACK, g);

            for body in &self.bodies {
                body.render(c, g, x0, y0);
            }
        })
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.bodies.iter_mut().for_each(|body| body.update(args));
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
    }
}
