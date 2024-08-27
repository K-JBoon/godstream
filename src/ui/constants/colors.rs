use crate::*;

#[allow(dead_code)]
pub trait ColorPalette {
    const BLACK: Color;
    const WHITE: Color;
}

impl ColorPalette for Color {
    const BLACK: Color = Color::srgba(0.0, 0.0, 0.0, 0.9);
    const WHITE: Color = Color::srgba(255.0, 255.0, 255.0, 0.9);
}
