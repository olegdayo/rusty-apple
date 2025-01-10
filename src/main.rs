use ffmpeg_next::util::frame::video::Video;
use image::GenericImageView;

const VIDEO_WIDTH: u32 = 120;
const VIDEO_HEIGHT: u32 = 90;

fn main() {
    ffmpeg_next::init().unwrap();
    loop {
        run_video("./data/bad-apple-small.mp4");
    }
}

fn run_video<P: AsRef<std::path::Path> + ?Sized>(path: &P) {
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

    let index = stream.index();
    for (stream, packet) in input.packets() {
        if stream.index() == index {
            decoder.send_packet(&packet).unwrap();
            receive_and_process_decoded_frames(&mut decoder).unwrap();
        }
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
        VIDEO_WIDTH,
        VIDEO_HEIGHT,
        image::imageops::FilterType::Gaussian,
    );

    // Not needed for now
    // img.save(format!("./tmp/{}.png", index)).unwrap();

    let mut ascii_art = String::new();
    for i in 0..img.height() {
        for j in 0..img.width() {
            let pixel = img.get_pixel(j, i);
            ascii_art += &rusty_apple::color::Color::from_pixel(pixel)
                .to_ascii_art()
                .to_string();
        }
        ascii_art += "\n";
    }
    println!("{}", ascii_art);
}
