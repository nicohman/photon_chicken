use graphics::types::Color;
use graphics::color::hex;
pub fn get_color (id: i32) -> Color {
    match (id) {
        1 => hex("430082"),
        2 => hex("008080"),
        _ => hex("008080")
    }
}
