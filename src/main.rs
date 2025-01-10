use std::fmt::format;

use ffmpeg_next::util::frame::video::Video;
use image::GenericImageView;

fn main() {
    ffmpeg_next::init().unwrap();
    let mut ictx = ffmpeg_next::format::input("./data/bad-apple.mp4").unwrap();

    let input = ictx
        .streams()
        .best(ffmpeg_next::media::Type::Video)
        .unwrap();

    let video_stream_index = input.index();

    let context_decoder =
        ffmpeg_next::codec::context::Context::from_parameters(input.parameters()).unwrap();
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

    let mut frame_index = 0;

    let mut receive_and_process_decoded_frames =
        |decoder: &mut ffmpeg_next::decoder::Video| -> Result<(), ffmpeg_next::Error> {
            let mut decoded = Video::empty();
            while decoder.receive_frame(&mut decoded).is_ok() {
                let mut rgb_frame = Video::empty();
                scaler.run(&decoded, &mut rgb_frame)?;
                process_image(&rgb_frame, frame_index);
                frame_index += 1;
            }
            Ok(())
        };

    for (stream, packet) in ictx.packets() {
        if stream.index() == video_stream_index {
            decoder.send_packet(&packet).unwrap();
            receive_and_process_decoded_frames(&mut decoder).unwrap();
        }
        // break;
    }

    decoder.send_eof().unwrap();
    receive_and_process_decoded_frames(&mut decoder).unwrap();
}

fn process_image(frame: &Video, index: usize) {
    let data = [
        format!("P6\n{} {}\n255\n", frame.width(), frame.height()).as_bytes(),
        frame.data(0),
    ]
    .concat();
    let img = image::load_from_memory(&data).unwrap().resize(
        100,
        100,
        image::imageops::FilterType::Gaussian,
    );

    // img.save(format!("./tmp/{}.png", index)).unwrap();

    let mut ascii_art = String::new();
    for i in 0..img.height() {
        for j in 0..img.width() {
            let pixel = img.get_pixel(j, i);
            // println!("{:?}", pixel);
            ascii_art += &rusty_apple::color::Color::from_pixel(pixel)
                .to_ascii_art()
                .to_string();
        }
        ascii_art += "\n";
    }
    println!("{}", ascii_art);
}
