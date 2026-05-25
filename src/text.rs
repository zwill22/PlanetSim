use graphics::types::FontSize;
use graphics::{Context, Text, Transformed};
use opengl_graphics::{GlGraphics, GlyphCache};

pub(crate) fn render_text(
    strings: &Vec<String>,
    position: &(f64, f64),
    c: &Context,
    g: &mut GlGraphics,
    glyphs: &mut GlyphCache,
    colour: [f32; 4],
    font_size: f64,
) {
    let text = Text::new_color(colour, font_size as FontSize).round();

    let x = position.0;
    let mut y = position.1;

    for string in strings {
        let transform = c.transform.trans(x, y);
        text.draw(string, glyphs, &c.draw_state, transform, g)
            .expect("Error drawing text");
        y += font_size * 1.5;
    }
}
