// Global constants

use glutin_window::OpenGL;

pub(crate) const OPENGL: OpenGL = OpenGL::V4_5;
pub(crate) const TITLE: &str = "Solar System";
pub(crate) const FULLSCREEN: bool = false;
pub(crate) const SAMPLES: u8 = 0;

pub(crate) const WIDTH: f64 = 3072.0;
pub(crate) const HEIGHT: f64 = 2048.0;
pub(crate) const EXIT_ON_ESCAPE: bool = true;
pub(crate) const SCALE_FACTOR: f64 = 0.00000015;
pub(crate) const MIN_RADIUS: f64 = 4.0;
pub(crate) const V_FACTOR: f64 = 10.0;
pub(crate) const ZOOM_FACTOR: f64 = 1.2;
pub(crate) const SHOW_ORBITS: bool = true;
// End Constants
