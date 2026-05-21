use graphics::color::WHITE;
use graphics::types::FontSize;
use graphics::{Context, Text, Transformed};
use opengl_graphics::{GlGraphics, GlyphCache};
use piston::{Button, ButtonArgs, ButtonState, Key};

// ********** Defaults **************
const MIN_RADIUS: f64 = 2.0;
const SHOW_ORBITS: bool = true;
const ZOOM_FACTOR: f64 = 1.1;
const MIN_SCALE: f64 = 0.1;
const MAX_SCALE: f64 = 2000.0;
const MIN_SPEED: f64 = 0.1;
const MAX_SPEED: f64 = 2000.0;
const MIN_DRAW_RADIUS: f64 = 1.0;
const MAX_DRAW_RADIUS: f64 = 5.0;
const SHOW_CONTROLS: bool = false;
const FONT_SIZE: f64 = 24.0;
// **********************************

const TARGETS: [&str; 17] = [
    "Sun", "Mercury", "Venus", "Earth", "Mars", "Vesta", "Ceres", "Pallas", "Jupiter", "Saturn",
    "Uranus", "Neptune", "Pluto", "Haumea", "Makemake", "Gonggong", "Eris",
];

fn render_text(
    strings: &Vec<String>,
    position: &(f64, f64),
    c: &Context,
    g: &mut GlGraphics,
    glyphs: &mut GlyphCache,
) {
    let text = Text::new_color(WHITE, FONT_SIZE as FontSize).round();

    let x = position.0;
    let mut y = position.1;

    for string in strings {
        let transform = c.transform.trans(x, y);
        text.draw(string, glyphs, &c.draw_state, transform, g)
            .expect("Error drawing text");
        y += FONT_SIZE * 1.5;
    }
}

fn control_list() -> Vec<String> {
    let strings = [
        "Zoom in/out          [[/]]",
        "Speed -/+                 [,/.]",
        "Size -/+                     ['/\\]",
        "Toggle orbits          [o]",
        "Reset settings      [r]",
        "Toggle controls    [c]",
    ];

    strings.iter().map(|s| s.to_string()).collect()
}

fn render_control_list(c: &Context, g: &mut GlGraphics, glyphs: &mut GlyphCache) {
    let strings = control_list();
    let height = c.get_view_size()[1];
    let position = (FONT_SIZE, height - (FONT_SIZE * 1.5) * strings.len() as f64);

    render_text(&strings, &position, c, g, glyphs);
}

fn render_show_controls(c: &Context, g: &mut GlGraphics, glyphs: &mut GlyphCache) {
    let strings = vec!["Toggle controls    [c]".to_string()];
    let height = c.get_view_size()[1];
    let position = (FONT_SIZE, height - FONT_SIZE * 1.5);
    render_text(&strings, &position, c, g, glyphs);
}

fn not(number: Option<&str>, focus: &str, name: &str, id: &str) -> bool {
    match number {
        None => name != focus,
        Some(val) => !val.starts_with(id),
    }
}

pub(crate) struct Settings {
    initial_scale: f64,
    initial_speed: f64,
    scale_factor: f64,
    show_orbits: bool,
    min_radius: f64,
    v_factor: f64,
    show_controls: bool,
    changed_focus: bool,
    focus: usize,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            initial_scale: 1.0,
            initial_speed: 1.0,
            scale_factor: 1.0,
            show_orbits: SHOW_ORBITS,
            min_radius: MIN_RADIUS,
            v_factor: 1.0,
            show_controls: SHOW_CONTROLS,
            changed_focus: false,
            focus: 0,
        }
    }
}

impl Settings {
    pub(crate) fn get_radius(&self, radius: f64) -> f64 {
        let r = radius * self.scale_factor * self.initial_scale;

        r.max(self.min_radius)
    }

    pub(crate) fn get_focus(&self) -> String {
        TARGETS[self.focus].to_string()
    }

    fn zoom_out(&mut self) {
        if self.scale_factor > MIN_SCALE {
            self.scale_factor /= ZOOM_FACTOR;
        }
    }
    fn zoom_in(&mut self) {
        if self.scale_factor < MAX_SCALE {
            self.scale_factor *= ZOOM_FACTOR;
        }
    }

    fn slow_down(&mut self) {
        if self.v_factor > MIN_SPEED {
            self.v_factor /= ZOOM_FACTOR;
        }
    }

    fn speed_up(&mut self) {
        if self.v_factor < MAX_SPEED {
            self.v_factor *= ZOOM_FACTOR;
        }
    }

    fn decrease_planet_size(&mut self) {
        if self.min_radius > MIN_DRAW_RADIUS {
            self.min_radius /= ZOOM_FACTOR;
        }
    }

    fn increase_planet_size(&mut self) {
        if self.min_radius < MAX_DRAW_RADIUS {
            self.min_radius *= ZOOM_FACTOR;
        }
    }

    fn toggle_orbits(&mut self) {
        self.show_orbits = !self.show_orbits;
    }

    fn reset(&mut self) {
        self.scale_factor = 1.0;
        self.min_radius = MIN_RADIUS;
        self.v_factor = 1.0;
    }

    fn toggle_controls(&mut self) {
        self.show_controls = !self.show_controls;
    }

    fn previous_target(&mut self) {
        if self.focus == 0 {
            self.focus = TARGETS.len() - 1;
        } else {
            self.focus -= 1;
        }

        self.reset();
        self.changed_focus = true;
    }

    fn next_target(&mut self) {
        if self.focus + 1 == TARGETS.len() {
            self.focus = 0;
        } else {
            self.focus += 1;
        }

        self.reset();
        self.changed_focus = true;
    }

    pub(crate) fn new_focus(&mut self) -> bool {
        self.changed_focus
    }

    pub(crate) fn refocus(&mut self, scales: (Option<f64>, Option<f64>)) {
        if let Some(length_scale) = scales.0 {
            self.initial_scale = length_scale;
        }

        if let Some(time_scale) = scales.1 {
            self.initial_speed = time_scale;
        }
        self.changed_focus = false;
    }

    pub(crate) fn is_focus(&self, name: Option<&str>) -> bool {
        match name {
            None => false,
            Some(val) => val == self.get_focus(),
        }
    }

    pub(crate) fn out_of_focus(
        &self,
        name: Option<&str>,
        satellite: bool,
        number: Option<&str>,
    ) -> bool {
        let object = match name {
            None => {
                return true;
            }
            Some(value) => value,
        };

        match self.get_focus().as_str() {
            "Sun" => satellite,
            "Earth" => object != "Earth" && object != "Moon",
            "Mars" => not(number, "Mars", object, "M"),
            "Jupiter" => not(number, "Jupiter", object, "J"),
            "Saturn" => not(number, "Saturn", object, "S"),
            "Uranus" => not(number, "Uranus", object, "U"),
            "Neptune" => not(number, "Neptune", object, "N"),
            "Pluto" => not(number, "Pluto", object, "134340"),
            "Haumea" => not(number, "Haumea", object, "136108"),
            "Gonggong" => not(number, "Gonggong", object, "225088"),
            "Eris" => not(number, "Eris", object, "136199"),
            planet => object != planet,
        }
    }

    fn key_control(&mut self, key: &Key) {
        match key {
            Key::LeftBracket => self.zoom_out(),           // `[` Zoom out
            Key::RightBracket => self.zoom_in(),           // `]` Zoom in
            Key::Comma => self.slow_down(),                // `,` Slow down
            Key::Period => self.speed_up(),                // `.` Speed up
            Key::Quote => self.decrease_planet_size(),     // `'` Decrease sizes
            Key::Backslash => self.increase_planet_size(), // `\` Increase sizes
            Key::O => self.toggle_orbits(),                // `o` Toggle orbits
            Key::R => self.reset(),                        // `r` Reset settings to default
            Key::C => self.toggle_controls(),              // `c` Toggle list of controls
            Key::Left => self.previous_target(),           // `<-` Focus on previous target
            Key::Right => self.next_target(),              // '->` Focus on next target
            _ => {}
        }
    }

    pub(crate) fn control(&mut self, args: &ButtonArgs) {
        if args.state == ButtonState::Press {
            match args.button {
                Button::Keyboard(key) => self.key_control(&key),
                Button::Mouse(_) => {}
                Button::Controller(_) => {}
                Button::Hat(_) => {}
            }
        }
    }

    pub(crate) fn scroll(&mut self, args: &[f64; 2]) {
        let _horizontal_scroll = args[0];
        let vertical_scroll = args[1];

        if vertical_scroll > 0.0 {
            self.zoom_out();
        }

        if vertical_scroll < 0.0 {
            self.zoom_in();
        }
    }

    pub(crate) fn scale(&self, coordinate: f64) -> f64 {
        coordinate * self.scale_factor * self.initial_scale
    }

    pub(crate) fn get_orbit(&self, orbit: &[f64; 4]) -> Option<[f64; 4]> {
        if self.show_orbits {
            let out = orbit.map(|x| x * self.scale_factor * self.initial_scale);
            return Some(out);
        }

        None
    }

    pub(crate) fn angular_velocity(
        &self,
        coordinates: &(f64, f64),
        orbital_coefficient: f64,
    ) -> f64 {
        let r0 = coordinates.0 / (self.scale_factor * self.initial_scale); // Unscaled radius

        // \omega = c / r^2
        self.v_factor * self.initial_speed * orbital_coefficient / (r0 * r0)
    }

    fn get_settings_strings(&self) -> Vec<String> {
        let mut output = Vec::new();

        output.push("Solar System View".to_string());

        let focus = format!("Focus: {:>12}", TARGETS[self.focus]);
        output.push(focus);

        let zoom = self.scale_factor.clamp(MIN_SCALE, MAX_SCALE);
        let zoom_string = format!("Zoom:  {:>12}", format!("x{:.2}", zoom));
        output.push(zoom_string);

        let speed = self.v_factor.clamp(MIN_SPEED, MAX_SPEED);
        let speed_string = format!("Speed: {:>12}", format!("x{:.2}", speed));

        output.push(speed_string);

        let size = self.min_radius.clamp(MIN_DRAW_RADIUS, MAX_DRAW_RADIUS);
        let size_string = format!("Sizes:     {:>12.2}", size);
        output.push(size_string);

        let orbits = if self.show_orbits {
            "Orbits:              ON"
        } else {
            "Orbits:             OFF"
        };
        output.push(orbits.to_string());

        output
    }
    fn render_settings(&mut self, c: &Context, g: &mut GlGraphics, glyphs: &mut GlyphCache) {
        let strings = self.get_settings_strings();
        let position = (FONT_SIZE, FONT_SIZE * 1.5);

        render_text(&strings, &position, c, g, glyphs);
    }

    fn render_controls(&mut self, c: &Context, g: &mut GlGraphics, glyphs: &mut GlyphCache) {
        if self.show_controls {
            render_control_list(c, g, glyphs);
        } else {
            render_show_controls(c, g, glyphs);
        }
    }

    pub(crate) fn render(&mut self, c: &Context, g: &mut GlGraphics, glyphs: &mut GlyphCache) {
        self.render_settings(c, g, glyphs);
        self.render_controls(c, g, glyphs);
    }
}
