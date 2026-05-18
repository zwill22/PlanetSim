use crate::settings::Settings;
use graphics::{Context, Ellipse, Transformed, ellipse};
use opengl_graphics::GlGraphics;
use piston::UpdateArgs;

pub(crate) struct Body {
    colour: [f32; 4],
    radius: f64,
    orbital_velocity: f64,
    eccentricity: f64,
    el: f64,
    orbital_parameters: [f64; 4],
    coordinates: (f64, f64),
    orbit: Option<[f64; 4]>,
    draw_radius: f64,
}

impl Body {
    pub(crate) fn new(
        size: f64,
        periapsis: f64,
        apoapsis: f64,
        col: [f32; 4],
        velocity: f64,
        settings: &Settings,
    ) -> Body {
        let e = (apoapsis - periapsis) / (apoapsis + periapsis);
        let semi_latus_rectum = periapsis * (1.0 + e) * 10.0_f64.powi(6);

        let a = semi_latus_rectum / (1.0 - e.powi(2));
        let b = a * (1.0 - e.powi(2)).sqrt();

        let orbit_parameters = [-apoapsis * 10.0_f64.powi(6), -b, 2.0 * a, 2.0 * b];

        let initial_coordinates = (settings.scale(periapsis), 0.0);
        let initial_orbit = settings.get_orbit(&orbit_parameters);
        let initial_radius = settings.get_radius(size);

        Body {
            colour: col,
            radius: size,
            orbital_velocity: velocity,
            eccentricity: e,
            el: semi_latus_rectum,
            orbital_parameters: orbit_parameters,
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

    pub(crate) fn update(&mut self, settings: &Settings, args: &UpdateArgs) {
        self.draw_radius = settings.get_radius(self.radius);

        let w = settings.get_angular_velocity(&self.coordinates, self.orbital_velocity);

        self.coordinates.1 += w * args.dt;

        let cos_theta = self.coordinates.1.cos();

        self.coordinates.0 = settings.scale(self.el / (1.0 + self.eccentricity * cos_theta));

        self.orbit = settings.get_orbit(&self.orbital_parameters);
    }
}
