use crate::constants::SCALE_FACTOR;
use crate::settings::Settings;
use graphics::{ellipse, Context, Ellipse, Transformed};
use opengl_graphics::GlGraphics;
use piston::UpdateArgs;

pub struct Body {
    name: String,
    colour: [f32; 4],
    radius: f64,
    orbital_velocity: f64,
    angular_velocity: f64,
    eccentricity: f64,
    el: f64,
    orbit: [f64; 4],
    position: (f64, f64),
}

impl Body {
    pub(crate) fn new(
        size: f64,
        periapsis: f64,
        apoapsis: f64,
        col: [f32; 4],
        velocity: f64,
        name: &str,
        settings: &Settings,
    ) -> Body {
        let r0 = (size * settings.scale_factor).max(settings.min_radius);

        let e = (apoapsis - periapsis) / (apoapsis + periapsis);
        let el = settings.scale_factor * periapsis * (1.0 + e) * 10.0_f64.powi(6);

        let a = el / (1.0 - e.powi(2));
        let b = a * (1.0 - e.powi(2)).sqrt();

        let v = settings.v_factor * velocity;

        let position = (periapsis * 10.0_f64.powi(6) * settings.scale_factor, 0.0);

        let w = v / position.0;

        let orbit = [
            -apoapsis * SCALE_FACTOR * 10.0_f64.powi(6),
            -b,
            2.0 * a,
            2.0 * b,
        ];

        Body {
            name: name.to_string(),
            colour: col,
            radius: r0,
            orbital_velocity: v,
            angular_velocity: w,
            eccentricity: e,
            el,
            orbit,
            position,
        }
    }

    fn render_body(&self, c: Context, g: &mut GlGraphics, xc: f64, yc: f64) {
        let circle = ellipse::circle(0.0, 0.0, self.radius);

        let transform = c
            .transform
            .trans(xc, yc) // Centre
            .rot_rad(self.position.1)
            .trans(self.position.0, 0.0);

        ellipse(self.colour, circle, transform, g)
    }

    fn render_orbit(&self, c: Context, g: &mut GlGraphics, xc: f64, yc: f64) {
        let ellipse = Ellipse::new_border(self.colour, 0.5);

        let transform = c.transform.trans(xc, yc);

        ellipse.draw(self.orbit, &c.draw_state, transform, g);
    }

    pub(crate) fn render(&self, c: Context, g: &mut GlGraphics, xc: f64, yc: f64) {
        self.render_body(c, g, xc, yc);
        self.render_orbit(c, g, xc, yc);
    }

    pub(crate) fn update(&mut self, args: &UpdateArgs) {
        if self.angular_velocity.abs() < 0.001 || self.name == "Sol" {
            return;
        }
        self.position.1 += self.angular_velocity * args.dt;

        let cos_theta = self.position.1.cos();

        self.position.0 = self.el / (1.0 + self.eccentricity * cos_theta);

        self.angular_velocity = self.orbital_velocity / self.position.0;
    }
}
