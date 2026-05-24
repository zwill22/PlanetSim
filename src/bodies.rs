use crate::body::Body;
use crate::settings::Settings;
use graphics::Context;
use opengl_graphics::{GlGraphics, GlyphCache};
use piston::UpdateArgs;

pub(crate) struct Bodies {
    bodies: Vec<Body>,
}

impl Bodies {
    pub(crate) fn new(bodies: Vec<Body>) -> Self {
        Bodies { bodies }
    }

    pub(crate) fn render(
        &self,
        c: &Context,
        g: &mut GlGraphics,
        glyphs: &mut GlyphCache,
        x0: f64,
        y0: f64,
    ) {
        self.bodies
            .iter()
            .for_each(|body| body.render(c, g, glyphs, x0, y0))
    }

    pub(crate) fn update(&mut self, settings: &Settings, args: &UpdateArgs) {
        self.bodies
            .iter_mut()
            .for_each(|body| body.update(settings, args));
    }

    fn get_length_scales(&self) -> Vec<f64> {
        self.bodies
            .iter()
            .map(|b| b.get_scale())
            .collect::<Vec<_>>()
    }
    fn get_length_scale(&self) -> Option<f64> {
        let lengths_scales = self.get_length_scales();
        let scales = lengths_scales.iter().max_by(|a, b| a.total_cmp(b));

        scales.map(|v| 500.0 / v)
    }

    fn get_time_periods(&self) -> Vec<f64> {
        self.bodies
            .iter()
            .filter_map(|b| b.get_time_scale())
            .collect::<Vec<_>>()
    }

    fn get_time_scale(&self) -> Option<f64> {
        let time_periods = self.get_time_periods();
        let time_scales = time_periods.iter().max_by(|a, b| a.total_cmp(b));

        time_scales.map(|v| 100000.0 * v)
    }

    pub(crate) fn get_scales(&self) -> (Option<f64>, Option<f64>) {
        let scale = self.get_length_scale();
        let time_scale = self.get_time_scale();

        (scale, time_scale)
    }
}
