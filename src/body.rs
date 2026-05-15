use graphics::{ellipse, rectangle, Context, Transformed};
use opengl_graphics::GlGraphics;
use piston::UpdateArgs;
use crate::constants::{SCALE_FACTOR, MIN_RADIUS, V_FACTOR};
pub struct Body {
    name: String,
    colour: [f32; 4],
    radius: f64,
    velocity: f64,
    eccentricity: f64,
    el: f64,
    position: (f64, f64),
}

impl Body {
    pub(crate) fn new(size: f64, periapse: f64, apoapse: f64, col: [f32; 4], velocity: f64, name: &str) -> Body {
        let radius = (size * SCALE_FACTOR).max(MIN_RADIUS);

        let e = (apoapse - periapse) / (apoapse + periapse);
        let el = SCALE_FACTOR * periapse * ( 1.0 + e) * 10.0_f64.powi(6);

        Body {
            name: name.to_string(),
            colour: col,
            radius,
            velocity: velocity * V_FACTOR,
            eccentricity: e,
            el,
            position: (periapse * 10.0_f64.powi(6) * SCALE_FACTOR, 0.0),
        }
    }

    pub(crate) fn render(&self, c: Context, g: &mut GlGraphics, xc: f64, yc: f64) {
        let circle= ellipse::circle(0.0, 0.0, self.radius);

        let transform = c
            .transform
            .trans(xc, yc)// Centre
            .rot_rad(self.position.1)
            .trans(self.position.0, 0.0);

        ellipse(self.colour, circle, transform, g);
    }

    pub(crate) fn update(&mut self, args: &UpdateArgs) {
        if self.velocity.abs() < 0.001 {
            return;
        }
        self.position.1 += self.velocity * args.dt;

        let cos_theta = self.position.1.cos();

        self.position.0 = self.el / ( 1.0 + self.eccentricity * cos_theta );
    }
}