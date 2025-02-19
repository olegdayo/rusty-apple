use flager::Parser;

fn main() {
    let parser = Parser::new();
    let tp = parser.parse(&rusty_apple::TYPE).unwrap();
    let video_width = parser.parse(&rusty_apple::VIDEO_WIDTH).unwrap();
    let video_height: u32 = parser.parse(&rusty_apple::VIDEO_HEIGHT).unwrap();
    match tp.as_str() {
        "tty" => rusty_apple::tty::run(video_width, video_height),
        "tg" => rusty_apple::tg::run_tg(),
        _ => panic!("wrong type"),
    }
    ffmpeg_next::init().unwrap();
}
