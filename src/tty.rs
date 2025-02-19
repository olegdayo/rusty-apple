use std::io::Write;

use ffmpeg_next::util::frame::video::Video;
use image::GenericImageView;

pub fn run(video_width: u32, video_height: u32) {
    loop {
        run_video("./data/bad-apple.mp4", video_width, video_height);
    }
}

fn run_video<P: AsRef<std::path::Path> + ?Sized>(path: &P, video_width: u32, video_height: u32) {
    let mut input = ffmpeg_next::format::input(path).unwrap();
    let stream = input
        .streams()
        .best(ffmpeg_next::media::Type::Video)
        .unwrap();

    let context_decoder =
        ffmpeg_next::codec::context::Context::from_parameters(stream.parameters()).unwrap();
    let mut decoder = context_decoder.decoder().video().unwrap();

    let mut scaler = ffmpeg_next::software::scaling::context::Context::get(
        decoder.format(),
        decoder.width(),
        decoder.height(),
        ffmpeg_next::format::Pixel::RGB24,
        decoder.width(),
        decoder.height(),
        ffmpeg_next::software::scaling::flag::Flags::BILINEAR,
    )
    .unwrap();

    let index = stream.index();
    for (stream, packet) in input.packets() {
        if stream.index() == index {
            decoder.send_packet(&packet).unwrap();
            output_frames(&mut decoder, &mut scaler, video_width, video_height).unwrap();
        }
    }

    decoder.send_eof().unwrap();
    output_frames(&mut decoder, &mut scaler, video_width, video_height).unwrap();
}

fn output_frames(
    decoder: &mut ffmpeg_next::decoder::Video,
    scaler: &mut ffmpeg_next::software::scaling::Context,
    video_width: u32,
    video_height: u32,
) -> Result<(), ffmpeg_next::Error> {
    let mut decoded = Video::empty();
    while decoder.receive_frame(&mut decoded).is_ok() {
        let mut rgb_frame = Video::empty();
        scaler.run(&decoded, &mut rgb_frame)?;
        let art = process_image(&rgb_frame, video_width, video_height);

        print!("\x1B[2J\x1B[1;1H");
        std::io::stdout().write_all(art.as_bytes()).unwrap();
        std::io::stdout().flush().unwrap();
        std::thread::sleep(core::time::Duration::from_millis(15));
    }
    Ok(())
}

fn process_image(frame: &Video, video_width: u32, video_height: u32) -> String {
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
        art += &crate::color::Color::from_pixel(pixel)
            .to_emoji_art()
            .to_string();
        for j in 1..img.width() {
            let left_pixel = crate::color::Color::from_pixel(img.get_pixel(j - 1, i));
            let right_pixel = crate::color::Color::from_pixel(img.get_pixel(j, i));
            let pixel = match left_pixel {
                crate::color::Color::Black => match right_pixel {
                    crate::color::Color::Black => "🌑",
                    crate::color::Color::White => "🌓",
                },
                crate::color::Color::White => match right_pixel {
                    crate::color::Color::Black => "🌗",
                    crate::color::Color::White => "🌕",
                },
            };

            art += pixel;
        }
        let pixel = img.get_pixel(img.width() - 1, i);
        art += &crate::color::Color::from_pixel(pixel)
            .to_emoji_art()
            .to_string();
        art += "\n";
    }
    art
}
