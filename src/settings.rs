// ********** Defaults **************
const SCALE_FACTOR: f64 = 0.00000015;
const MIN_RADIUS: f64 = 4.0;
const V_FACTOR: f64 = 1000000000.0;
const SHOW_ORBITS: bool = true;
const ZOOM_FACTOR: f64 = 1.1;
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
        self.scale_factor /= ZOOM_FACTOR;
    }
    pub(crate) fn zoom_in(&mut self) {
        self.scale_factor *= ZOOM_FACTOR;
    }

    pub(crate) fn slow_down(&mut self) {
        self.v_factor /= ZOOM_FACTOR;
    }

    pub(crate) fn speed_up(&mut self) {
        self.v_factor *= ZOOM_FACTOR;
    }

    pub(crate) fn decrease_planet_size(&mut self) {
        self.min_radius /= ZOOM_FACTOR;
    }

    pub(crate) fn increase_planet_size(&mut self) {
        self.min_radius *= ZOOM_FACTOR;
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

    pub(crate) fn angular_velocity(&self, coordinates: &(f64, f64), orbital_coefficient: f64) -> f64 {
        let r0 = coordinates.0 / self.scale_factor; // Unscaled radius

        // \omega = c / r^2
        self.v_factor * orbital_coefficient / (r0 * r0)
    }
}
