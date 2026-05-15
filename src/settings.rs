use crate::constants::{MIN_RADIUS, SCALE_FACTOR, V_FACTOR};

pub(crate) struct Settings {
    pub(crate) scale_factor: f64,
    pub(crate) min_radius: f64,
    pub(crate) v_factor: f64,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            scale_factor: SCALE_FACTOR,
            min_radius: MIN_RADIUS,
            v_factor: V_FACTOR,
        }
    }
}
