// ********** Defaults **************
const SCALE_FACTOR: f64 = 0.00000015;
const MIN_RADIUS: f64 = 4.0;
const V_FACTOR: f64 = 10.0;
const SHOW_ORBITS: bool = true;

// **********************************

pub(crate) struct Settings {
    pub(crate) scale_factor: f64,
    pub(crate) show_orbits: bool,
    pub(crate) min_radius: f64,
    pub(crate) v_factor: f64,
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
