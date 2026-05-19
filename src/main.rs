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
use glutin_window::GlutinWindow;
use graphics::clear;
use graphics::color::BLACK;
use opengl_graphics::{GlGraphics, GlyphCache, OpenGL, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::window::WindowSettings;
use piston::{
    Button, ButtonArgs, ButtonEvent, ButtonState, Key, MouseScrollEvent, RenderArgs, RenderEvent,
    UpdateArgs, UpdateEvent,
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
    bodies: Vec<Body>,
    settings: Settings,
    glyphs: GlyphCache<'a>,
}

impl App<'_> {

    fn new<'a>(opengl: OpenGL) -> App<'a> {
        let settings = Settings::default();
        let glyphs = setup_glyphs();
        App {
            gl: GlGraphics::new(opengl),
            bodies: initialise(&settings),
            settings,
            glyphs,
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        let x0 = args.window_size[0] / 2.0;
        let y0 = args.window_size[1] / 2.0;

        self.gl.draw(args.viewport(), |c, g| {
            clear(BLACK, g);

            self.settings.render(&c, g, &mut self.glyphs);

            self.bodies
                .iter()
                .for_each(|body| body.render(&c, g, x0, y0))
        })
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.bodies
            .iter_mut()
            .for_each(|body| body.update(&self.settings, args));
    }

    fn key_control(&mut self, key: &Key) {
        match key {
            Key::LeftBracket => self.settings.zoom_out(), // `[` Zoom out
            Key::RightBracket => self.settings.zoom_in(), // `]` Zoom in
            Key::Comma => self.settings.slow_down(),      // `,` Slow down
            Key::Period => self.settings.speed_up(),      // `.` Speed up
            Key::Quote => self.settings.decrease_planet_size(), // `'` Decrease sizes
            Key::Backslash => self.settings.increase_planet_size(), // `\` Increase sizes
            Key::O => self.settings.toggle_orbits(),      // `o` Toggle orbits
            _ => {}
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

    fn scroll(&mut self, args: &[f64; 2]) {
        let _horizontal_scroll = args[0] / WIDTH;
        let vertical_scroll = args[1] / HEIGHT;

        if vertical_scroll > 0.0 {
            self.settings.zoom_out();
        }

        if vertical_scroll < 0.0 {
            self.settings.zoom_in();
        }
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

        if let Some(args) = e.mouse_scroll_args() {
            app.scroll(&args);
        }
    }
}
