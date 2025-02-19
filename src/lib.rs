pub mod bot;
pub mod color;
pub mod tg;
pub mod tty;

use flager::{new_flag, Flag, Parser};

pub const VIDEO_WIDTH: Flag<u32> = new_flag!("-w", "--width").mandatory().help("video width");
pub const VIDEO_HEIGHT: Flag<u32> = new_flag!("-h", "--height").mandatory().help("video height");
pub const TYPE: Flag<String> = new_flag!("-t", "--type")
    .mandatory()
    .help("type of bad apple to launch: tty/tg");
