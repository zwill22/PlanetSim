use graphics::color::WHITE;
use graphics::{Context, Text, Transformed};
use opengl_graphics::{GlGraphics, GlyphCache};


// ********** Defaults **************
const SCALE_FACTOR: f64 = 0.00000015;
const MIN_RADIUS: f64 = 2.0;
const V_FACTOR: f64 = 1000000000.0;
const SHOW_ORBITS: bool = true;
const ZOOM_FACTOR: f64 = 1.1;
const MIN_SCALE: f64 = 0.1;
const MAX_SCALE: f64 = 2000.0;
const MIN_SPEED: f64 = 0.1;
const MAX_SPEED: f64 = 2000.0;
const MIN_DRAW_RADIUS: f64 = 1.0;
const MAX_DRAW_RADIUS: f64 = 5.0;
// **********************************

pub(crate) struct Settings {
    scale_factor: f64,
    show_orbits: bool,
    min_radius: f64,
    v_factor: f64,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            scale_factor: SCALE_FACTOR,
            show_orbits: SHOW_ORBITS,
            min_radius: MIN_RADIUS,
            v_factor: V_FACTOR,
        }
    }
}

impl Settings {
    pub(crate) fn get_radius(&self, radius: f64) -> f64 {
        let r = radius * self.scale_factor;

        r.max(self.min_radius)
    }

    pub(crate) fn zoom_out(&mut self) {
        if self.scale_factor > SCALE_FACTOR * MIN_SCALE {
            self.scale_factor /= ZOOM_FACTOR;
        }
    }
    pub(crate) fn zoom_in(&mut self) {
        if self.scale_factor < SCALE_FACTOR * MAX_SCALE {
            self.scale_factor *= ZOOM_FACTOR;
        }
    }

    pub(crate) fn slow_down(&mut self) {
        if self.v_factor > V_FACTOR * MIN_SPEED {
            self.v_factor /= ZOOM_FACTOR;
        }
    }

    pub(crate) fn speed_up(&mut self) {
        if self.v_factor < V_FACTOR * MAX_SPEED {
            self.v_factor *= ZOOM_FACTOR;
        }
    }

    pub(crate) fn decrease_planet_size(&mut self) {
        if self.min_radius > MIN_DRAW_RADIUS {
            self.min_radius /= ZOOM_FACTOR;
        }
    }

    pub(crate) fn increase_planet_size(&mut self) {
        if self.min_radius < MAX_DRAW_RADIUS {
            self.min_radius *= ZOOM_FACTOR;
        }
    }

    pub(crate) fn toggle_orbits(&mut self) {
        self.show_orbits = !self.show_orbits;
    }

    pub(crate) fn scale(&self, coordinate: f64) -> f64 {
        coordinate * self.scale_factor
    }

    pub(crate) fn get_orbit(&self, orbit: &[f64; 4]) -> Option<[f64; 4]> {
        if self.show_orbits {
            let out = orbit.map(|x| x * self.scale_factor);
            return Some(out);
        }

        None
    }

    pub(crate) fn angular_velocity(
        &self,
        coordinates: &(f64, f64),
        orbital_coefficient: f64,
    ) -> f64 {
        let r0 = coordinates.0 / self.scale_factor; // Unscaled radius

        // \omega = c / r^2
        self.v_factor * orbital_coefficient / (r0 * r0)
    }

    pub(crate) fn render(&mut self, c: &Context, g: &mut GlGraphics, glyphs: &mut GlyphCache) {
        let font_size = 24;
        let text = Text::new_color(WHITE, font_size).round();

        let orbits = if self.show_orbits {
            "Orbits:              ON"
        } else {
            "Orbits:             OFF"
        };

        let strings = vec![
            "Solar System View".to_string(),
            format!(
                "Zoom:  {:>12}",
                format!("x{:.2}", (self.scale_factor / SCALE_FACTOR).clamp(MIN_SCALE, MAX_SCALE))
            ),
            format!(
                "Speed: {:>12}",
                format!("x{:.2}", (self.v_factor / V_FACTOR).clamp(MIN_SPEED, MAX_SPEED))
            ),
            format!(
                "Sizes:     {:>12.2}",
                self.min_radius.clamp(MIN_DRAW_RADIUS, MAX_DRAW_RADIUS)
            ),
            orbits.to_string(),
        ];

        let mut height = font_size as f64 * 1.5;

        for string in strings {
            let transform = c.transform.trans(font_size as f64, height);
            text.draw(&string, glyphs, &c.draw_state, transform, g)
                .expect("Error drawing text");
            height += font_size as f64 * 1.5;
        }
    }
}
