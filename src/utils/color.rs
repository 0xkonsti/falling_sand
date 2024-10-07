use bevy::color::Color;

pub fn hex_to_color(hex: &str) -> Option<Color> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 && hex.len() != 8 || !hex.chars().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }

    let r = u8::from_str_radix(&hex[0..2], 16).ok()? as f32 / 255.0;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()? as f32 / 255.0;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()? as f32 / 255.0;

    if hex.len() == 8 {
        let a = u8::from_str_radix(&hex[6..8], 16).ok()? as f32 / 255.0;
        return Some(Color::srgba(r, g, b, a));
    }

    Some(Color::srgb(r, g, b))
}
