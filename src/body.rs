use crate::settings::Settings;
use graphics::color::{BLUE, CYAN, GRAY, RED, TEAL, WHITE, YELLOW, hex};
use graphics::{Context, Ellipse, Transformed, ellipse};
use opengl_graphics::GlGraphics;
use piston::UpdateArgs;

fn colour(body: Option<&str>) -> [f32; 4] {
    let Some(name) = body else { return GRAY };
    match name {
        "Sun" => WHITE,
        "Mercury" => GRAY,
        "Venus" => hex("FF8C00"),
        "Earth" => TEAL,
        "Mars" => RED,
        "Ceres" => GRAY,
        "Jupiter" => hex("FFA500"),
        "Saturn" => YELLOW,
        "Uranus" => CYAN,
        "Neptune" => BLUE,
        "Pluto" => hex("A52A2A"),
        "Eris" => GRAY,
        &_ => GRAY,
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
    colour: [f32; 4],
    radius: f64,
    orbital_parameters: Option<OrbitalParameters>,
    coordinates: (f64, f64),
    orbit: Option<[f64; 4]>,
    draw_radius: f64,
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
                colour: col,
                radius: size,
                orbital_parameters: None,
                coordinates: (0.0, 0.0),
                orbit: None,
                draw_radius: initial_radius,
            };
        }

        let orbit = orbital_parameters(a, e, period, prograde);

        let initial_coordinates = match &orbit {
            Some(parameters) => (settings.scale(parameters.periapsis), 0.0),
            None => (0.0, 0.0),
        };

        let initial_orbit = match &orbit {
            Some(parameters) => settings.get_orbit(&parameters.get_orbit()),
            None => None,
        };

        Body {
            colour: col,
            radius: size,
            orbital_parameters: orbit,
            coordinates: initial_coordinates,
            orbit: initial_orbit,
            draw_radius: initial_radius,
        }
    }

    fn render_body(&self, c: &Context, g: &mut GlGraphics, xc: f64, yc: f64) {
        let circle = ellipse::circle(0.0, 0.0, self.draw_radius);

        let transform = c
            .transform
            .trans(xc, yc) // Centre
            .rot_rad(self.coordinates.1)
            .trans(self.coordinates.0, 0.0);

        ellipse(self.colour, circle, transform, g)
    }

    fn render_orbit(&self, c: &Context, g: &mut GlGraphics, xc: f64, yc: f64) {
        let Some(orbit) = self.orbit else {
            return;
        };

        let ellipse = Ellipse::new_border(self.colour, 0.5);

        let transform = c.transform.trans(xc, yc);

        ellipse.draw(orbit, &c.draw_state, transform, g);
    }

    pub(crate) fn render(&self, c: &Context, g: &mut GlGraphics, xc: f64, yc: f64) {
        self.render_body(c, g, xc, yc);
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

    pub(crate) fn update(&mut self, settings: &Settings, args: &UpdateArgs) {
        self.draw_radius = settings.get_radius(self.radius);

        if self.orbital_parameters.is_none() {
            return;
        };

        self.coordinates.1 += self.angular_velocity(settings) * args.dt;
        self.coordinates.0 = self.r(settings);

        self.orbit = self.orbit(settings);
    }
}
