pub enum Color {
    Black,
    White,
}

const THRESHOLD_BLACK: u8 = 15;
const THRESHOLD_WHITE: u8 = 240;

impl Color {
    pub fn from_pixel(pixel: image::Rgba<u8>) -> Color {
        if pixel[0] < THRESHOLD_BLACK && pixel[1] < THRESHOLD_BLACK && pixel[2] < THRESHOLD_BLACK {
            return Color::Black;
        }

        if pixel[0] > THRESHOLD_WHITE && pixel[1] > THRESHOLD_WHITE && pixel[2] > THRESHOLD_WHITE {
            return Color::White;
        }

        Color::White
    }

    pub fn to_ascii_art(self: &Color) -> char {
        match self {
            Color::Black => ' ',
            Color::White => '*',
        }
    }
}
