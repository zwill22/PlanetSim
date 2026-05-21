mod bodies;
mod body;
mod data;
mod settings;

extern crate approx;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use crate::bodies::Bodies;
use crate::data::Data;
use crate::settings::Settings;
use glutin_window::GlutinWindow;
use graphics::clear;
use graphics::color::BLACK;
use opengl_graphics::{GlGraphics, GlyphCache, OpenGL, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::window::WindowSettings;
use piston::{
    ButtonArgs, ButtonEvent, MouseScrollEvent, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent,
};
use rusttype::Font;

// ******** Global constants ********
const OPENGL: OpenGL = OpenGL::V4_5;
const TITLE: &str = "Solar System";
const FULLSCREEN: bool = false;
const SAMPLES: u8 = 0;
const WIDTH: f64 = 3072.0;
const HEIGHT: f64 = 2048.0;
const EXIT_ON_ESCAPE: bool = true;

// **********************************

fn setup_glyphs<'a>() -> GlyphCache<'a> {
    let font_data: &[u8] = include_bytes!("../data/Michroma-Regular.ttf");

    let font: Font<'static> = Font::try_from_bytes(font_data).unwrap();
    GlyphCache::from_font(font, (), TextureSettings::new())
}

struct App<'a> {
    gl: GlGraphics,
    settings: Settings,
    glyphs: GlyphCache<'a>,
    data: Data,
    bodies: Bodies,
}

impl App<'_> {
    fn new<'a>() -> App<'a> {
        let gl = GlGraphics::new(OPENGL);
        let mut settings = Settings::default();
        let data = Data::new();
        let bodies = data.get_bodies(&settings);
        let glyphs = setup_glyphs();

        let scales = bodies.get_scales();
        settings.refocus(scales);

        App {
            gl,
            settings,
            glyphs,
            data,
            bodies,
        }
    }

    fn get_centre(&self, args: &RenderArgs) -> (f64, f64) {
        (args.window_size[0] / 2.0, args.window_size[1] / 2.0)
    }

    fn render(&mut self, args: &RenderArgs) {
        let (x0, y0) = self.get_centre(args);

        self.gl.draw(args.viewport(), |c, g| {
            clear(BLACK, g);

            self.settings.render(&c, g, &mut self.glyphs);

            self.bodies.render(&c, g, x0, y0)
        })
    }

    fn update_bodies(&mut self) {
        if self.settings.new_focus() {
            self.bodies = self.data.get_bodies(&self.settings);
            let scales = self.bodies.get_scales();
            self.settings.refocus(scales);
        }
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.update_bodies();

        self.bodies.update(&self.settings, args);
    }

    fn control(&mut self, args: &ButtonArgs) {
        self.settings.control(args)
    }

    fn scroll(&mut self, args: &[f64; 2]) {
        self.settings.scroll(args);
    }
}

fn main() {
    let mut window: GlutinWindow = WindowSettings::new(TITLE, [WIDTH, HEIGHT])
        .graphics_api(OPENGL)
        .exit_on_esc(EXIT_ON_ESCAPE)
        .fullscreen(FULLSCREEN)
        .samples(SAMPLES)
        .build()
        .unwrap();

    let mut app = App::new();

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

        if let Some(args) = e.mouse_scroll_args() {
            app.scroll(&args);
        }
    }
}
