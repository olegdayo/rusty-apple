use std::io::Write;

use ffmpeg_next::util::frame::video::Video;

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
        let art = crate::art::process_image(&rgb_frame, video_width, video_height);

        print!("\x1B[2J\x1B[1;1H");
        std::io::stdout().write_all(art.as_bytes()).unwrap();
        std::io::stdout().flush().unwrap();
        std::thread::sleep(core::time::Duration::from_millis(15));
    }
    Ok(())
}
