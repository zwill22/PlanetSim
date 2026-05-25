use crate::settings::Settings;
use crate::text::render_text;
use graphics::color::{GRAY, hex};
use graphics::types::FontSize;
use graphics::{CharacterCache, Context, Ellipse, Transformed, ellipse};
use opengl_graphics::{GlGraphics, GlyphCache};
use piston::UpdateArgs;
use rand::{RngExt, rng};
use std::collections::HashMap;

const COLOURS: [(&str, &str); 51] = [
    // Sun
    ("Sun", "ffffff"),
    // Mercury
    ("Mercury", "8d8988"),
    // Venus
    ("Venus", "cf8932"),
    // Earth
    ("Earth", "526173"),
    ("Moon", "655f5e"),
    // Mars
    ("Mars", "d42f15"),
    ("Phobos", "8f7a6d"),
    ("Deimos", "7d7068"),
    // Asteroid belt
    ("Vesta", "89877f"),
    ("Ceres", "7a6e6d"),
    ("Pallas", "b3b3b3"),
    // Jupiter
    ("Jupiter", "d48e58"),
    ("Io", "c9c056"),
    ("Europa", "72685c"),
    ("Ganymede", "9d9183"),
    ("Callisto", "5d5643"),
    // Saturn
    ("Saturn", "e7c57c"),
    ("Mimas", "515151"),
    ("Enceladus", "e8e8e8"),
    ("Tethys", "c6c6c6"),
    ("Calypso", "c6c6c6"),
    ("Dione", "b1b0b1"),
    ("Helene", "b1b0b1"),
    ("Rhea", "c2c2c2"),
    ("Titan", "be9a52"),
    ("Iapetus", "928e8b"),
    // Uranus
    ("Uranus", "00cad2"),
    ("Miranda", "9f9fa0"),
    ("Ariel", "565656"),
    ("Umbriel", "383838"),
    ("Titania", "7e7d7f"),
    ("Oberon", "7e746a"),
    // Neptune
    ("Neptune", "3e65fa"),
    ("Triton", "b5b5b5"),
    ("Nereid", "6c6653"),
    ("Naiad", "a8a7a5"),
    ("Thalassa", "a8a28e"),
    // TNOs
    // Pluto
    ("Pluto", "A52A2A"),
    ("Charon", "95928f"),
    ("Styx", "cccccc"),
    ("Nix", "a9a398"),
    ("Hydra", "a5a1a2"),
    ("Kerberos", "92acd3"),
    // Haumea
    ("Haumea", "96857d"),
    ("Hi'iaka", "8c8074"),
    ("Namaka", "8d8d8d"),
    // Makemake
    ("Makemake", "744c45"),
    // Gonggong
    ("Gonggong", "8c6e6a"),
    ("Xiangliu", "5e9ee0"),
    // Eris
    ("Eris", "babac6"),
    ("Dysnomia", "5b5a56"),
];

fn colour(body: Option<&str>) -> [f32; 4] {
    let Some(name) = body else { return GRAY };
    let map: HashMap<_, _> = COLOURS.into_iter().collect();

    match map.get(name) {
        Some(result) => hex(result),
        None => hex("202020"),
    }
}

struct OrbitalParameters {
    a: f64,
    b: f64,
    e: f64,
    el: f64,
    apoapsis: f64,
    periapsis: f64,
    period: f64,
    prograde: bool,
}

impl OrbitalParameters {
    fn new(a: f64, e: f64, period: f64, prograde: bool) -> OrbitalParameters {
        let el = a * (1.0 - e.powi(2));

        let apoapsis = el / (1.0 - e);
        let periapsis = el / (1.0 + e);

        let b = (a * el).sqrt();

        OrbitalParameters {
            a,
            b,
            e,
            el,
            apoapsis,
            periapsis,
            period,
            prograde,
        }
    }

    fn get_orbit(&self) -> [f64; 4] {
        [-self.apoapsis, -self.b, 2.0 * self.a, 2.0 * self.b]
    }

    fn get_orbital_coefficient(&self) -> f64 {
        const SECONDS_PER_MINUTE: f64 = 60.0;
        const SECONDS_PER_HOUR: f64 = SECONDS_PER_MINUTE * 60.0;
        const SECONDS_PER_DAY: f64 = SECONDS_PER_HOUR * 24.0;
        const SECONDS_PER_YEAR: f64 = SECONDS_PER_DAY * 365.25;

        let sign = if self.prograde { -1.0 } else { 1.0 };

        2.0 * std::f64::consts::PI * sign * self.a * self.b / (self.period * SECONDS_PER_YEAR)
    }

    fn get_r(&self, cos_theta: f64) -> f64 {
        self.el / (1.0 + self.e * cos_theta)
    }
}

fn orbital_parameters(
    a: Option<f64>,
    e: Option<f64>,
    period: Option<f64>,
    prograde: bool,
) -> Option<OrbitalParameters> {
    let semi_major_axis = a?;
    let eccentricity = e?;
    let t = period?;

    let parameters = OrbitalParameters::new(semi_major_axis, eccentricity, t, prograde);

    Some(parameters)
}

pub(crate) struct Body {
    name: Option<String>,
    colour: [f32; 4],
    radius: f64,
    orbital_parameters: Option<OrbitalParameters>,
    coordinates: (f64, f64),
    orbit: Option<[f64; 4]>,
    draw_radius: f64,
    show_name: bool,
}

impl Body {
    pub(crate) fn new(
        name: Option<&str>,
        size: f64,
        a: Option<f64>,
        e: Option<f64>,
        prograde: bool,
        period: Option<f64>,
        settings: &Settings,
    ) -> Body {
        let col = colour(name);

        let initial_radius = settings.get_radius(size);

        if settings.is_focus(name) {
            return Body {
                name: name.map(String::from),
                colour: col,
                radius: size,
                orbital_parameters: None,
                coordinates: (0.0, 0.0),
                orbit: None,
                draw_radius: initial_radius,
                show_name: settings.show_name(),
            };
        }

        let orbit = orbital_parameters(a, e, period, prograde);

        let theta = rng().random::<f64>() * std::f64::consts::PI * 2.0;

        let initial_coordinates = match &orbit {
            Some(parameters) => (settings.scale(parameters.get_r(theta.cos())), theta),
            None => (0.0, 0.0),
        };

        let initial_orbit = match &orbit {
            Some(parameters) => settings.get_orbit(&parameters.get_orbit()),
            None => None,
        };

        Body {
            name: name.map(String::from),
            colour: col,
            radius: size,
            orbital_parameters: orbit,
            coordinates: initial_coordinates,
            orbit: initial_orbit,
            draw_radius: initial_radius,
            show_name: settings.show_name(),
        }
    }

    fn render_body(&self, c: &Context, g: &mut GlGraphics, xc: f64, yc: f64) {
        let rect = ellipse::circle(0.0, 0.0, self.draw_radius);
        let circle = Ellipse::new(self.colour);

        let transform = c
            .transform
            .trans(xc, yc) // Centre
            .rot_rad(self.coordinates.1)
            .trans(self.coordinates.0, 0.0);

        circle.draw(rect, &c.draw_state, transform, g);
    }

    fn render_name(
        &self,
        c: &Context,
        g: &mut GlGraphics,
        glyphs: &mut GlyphCache,
        xc: f64,
        yc: f64,
    ) {
        if !self.show_name {
            return;
        }
        let name = match &self.name {
            None => {
                return;
            }
            Some(v) => vec![v.clone()],
        };

        let r = self.coordinates.0;
        let theta = self.coordinates.1;

        let orbit = match &self.orbital_parameters {
            None => {
                return;
            }
            Some(v) => v,
        };

        if (1.0 + orbit.e * theta.cos()) * r / (1.0 - orbit.e) < 100.0 {
            return;
        }

        let font_size = 10.0;

        let width = glyphs.width(font_size as FontSize, &name[0]).unwrap();
        let height = glyphs
            .character(font_size as FontSize, 'A')
            .unwrap()
            .advance_height();

        let x0 = (r + self.draw_radius + width) * theta.cos() + xc - width / 2.0;
        let y0 = (r + self.draw_radius + width / 2.0) * theta.sin() + yc - height / 2.0;

        let position = (x0, y0);

        render_text(&name, &position, c, g, glyphs, self.colour, font_size);
    }

    fn render_orbit(&self, c: &Context, g: &mut GlGraphics, xc: f64, yc: f64) {
        let Some(orbit) = self.orbit else {
            return;
        };

        let ellipse = Ellipse::new_border(self.colour, 0.5);

        let transform = c.transform.trans(xc, yc);

        ellipse.draw(orbit, &c.draw_state, transform, g);
    }

    pub(crate) fn render(
        &self,
        c: &Context,
        g: &mut GlGraphics,
        glyphs: &mut GlyphCache,
        xc: f64,
        yc: f64,
    ) {
        self.render_body(c, g, xc, yc);
        self.render_name(c, g, glyphs, xc, yc);
        self.render_orbit(c, g, xc, yc);
    }

    fn r(&self, settings: &Settings) -> f64 {
        let cos_theta = self.coordinates.1.cos();
        let r0 = self.orbital_parameters.as_ref().unwrap().get_r(cos_theta);

        settings.scale(r0)
    }

    fn angular_velocity(&self, settings: &Settings) -> f64 {
        let c = self
            .orbital_parameters
            .as_ref()
            .unwrap()
            .get_orbital_coefficient();

        settings.angular_velocity(&self.coordinates, c)
    }

    fn orbit(&self, settings: &Settings) -> Option<[f64; 4]> {
        let orbit = self.orbital_parameters.as_ref()?.get_orbit();

        settings.get_orbit(&orbit)
    }

    pub(crate) fn get_scale(&self) -> f64 {
        match &self.orbital_parameters {
            None => self.radius,
            Some(orbit) => orbit.periapsis,
        }
    }

    pub(crate) fn get_time_scale(&self) -> Option<f64> {
        self.orbital_parameters.as_ref().map(|orbit| orbit.period)
    }

    pub(crate) fn update(&mut self, settings: &Settings, args: &UpdateArgs) {
        self.draw_radius = settings.get_radius(self.radius);

        if self.orbital_parameters.is_none() {
            return;
        };

        self.coordinates.1 += self.angular_velocity(settings) * args.dt;
        self.coordinates.0 = self.r(settings);

        self.orbit = self.orbit(settings);

        self.show_name = settings.show_name()
    }
}
