use piston_window::*;
use piston_window::math::Matrix2d;

pub struct TextRenderer {

}

impl TextRenderer {
    pub fn render(text : String,  text_color : &[f32; 4], transform: &Matrix2d, graphics: &mut G2d, glyphs : &mut Glyphs, draw_state : &DrawState) {
        text::Text::new_color(*text_color, 16).draw(
            text.as_str(),
            glyphs,
            draw_state,
            *transform, graphics
        ).unwrap();
    } 
}