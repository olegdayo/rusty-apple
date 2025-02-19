use std::sync::mpsc;

use ffmpeg_next::util::frame::video::Video;
use teloxide::{prelude::*, utils::command::BotCommands};

pub async fn run_tg(video_width: u32, video_height: u32) {
    dotenv::dotenv().unwrap();
    let bot = Bot::from_env();

    Command::repl(
        bot,
        move |bot: Bot, message: Message, command: Command| async move {
            match command {
                Command::Start => {
                    bot.send_message(message.chat.id, Command::descriptions().to_string())
                        .await?;
                    return Ok(());
                }
                _ => {}
            }

            let sent_message = bot.send_message(message.chat.id, "🀀").await.unwrap();

            let (sender, receiver) = mpsc::channel();

            std::thread::spawn(move || {
                run_video(
                    "./data/bad-apple-small.mp4",
                    video_width,
                    video_height,
                    sender,
                );
            });

            for art in receiver {
                let _ = bot
                    .edit_message_text(sent_message.chat.id, sent_message.id, art)
                    .await;
            }

            Ok(())
        },
    )
    .await;
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "get commands list")]
    Start,
    #[command(description = "run bad apple animation")]
    BadApple,
}

fn run_video<P: AsRef<std::path::Path> + ?Sized>(
    path: &P,
    video_width: u32,
    video_height: u32,
    sender: mpsc::Sender<String>,
) {
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
            output_frames(
                &mut decoder,
                &mut scaler,
                video_width,
                video_height,
                &sender,
            )
            .unwrap();
        }
    }

    decoder.send_eof().unwrap();
    output_frames(
        &mut decoder,
        &mut scaler,
        video_width,
        video_height,
        &sender,
    )
    .unwrap();
}

fn output_frames(
    decoder: &mut ffmpeg_next::decoder::Video,
    scaler: &mut ffmpeg_next::software::scaling::Context,
    video_width: u32,
    video_height: u32,
    writer: &mpsc::Sender<String>,
) -> Result<(), ffmpeg_next::Error> {
    let mut decoded = Video::empty();
    while decoder.receive_frame(&mut decoded).is_ok() {
        let mut rgb_frame = Video::empty();
        scaler.run(&decoded, &mut rgb_frame)?;
        let art = crate::art::process_image(&rgb_frame, video_width, video_height);

        writer.send(art).unwrap();
    }
    Ok(())
}
