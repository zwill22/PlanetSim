use crate::constants::{SCALE_FACTOR, SHOW_ORBITS};

pub(crate) struct Settings {
    pub(crate) scale_factor: f64,
    pub(crate) show_orbits: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            scale_factor: SCALE_FACTOR,
            show_orbits: SHOW_ORBITS,
        }
    }
}
