use ffmpeg_next::util::frame::video::Video;
use image::GenericImageView;

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

    pub fn to_emoji_art(self: &Color) -> char {
        match self {
            Color::Black => '🌑',
            Color::White => '🌕',
        }
    }
}

pub fn process_image(frame: &Video, video_width: u32, video_height: u32) -> String {
    let data = [
        format!("P6\n{} {}\n255\n", frame.width(), frame.height()).as_bytes(),
        frame.data(0),
    ]
    .concat();
    let img = image::load_from_memory(&data).unwrap().resize(
        video_width,
        video_height,
        image::imageops::FilterType::Gaussian,
    );

    let mut art = String::new();
    for i in 1..img.height() - 1 {
        let pixel = img.get_pixel(0, i);
        art += &Color::from_pixel(pixel).to_emoji_art().to_string();
        for j in 1..img.width() {
            let left_pixel = Color::from_pixel(img.get_pixel(j - 1, i));
            let right_pixel = Color::from_pixel(img.get_pixel(j, i));
            let pixel = match left_pixel {
                Color::Black => match right_pixel {
                    Color::Black => "🌑",
                    Color::White => "🌓",
                },
                Color::White => match right_pixel {
                    Color::Black => "🌗",
                    Color::White => "🌕",
                },
            };

            art += pixel;
        }
        let pixel = img.get_pixel(img.width() - 1, i);
        art += &Color::from_pixel(pixel).to_emoji_art().to_string();
        art += "\n";
    }
    art
}
