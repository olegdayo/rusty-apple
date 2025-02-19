use teloxide::prelude::*;

async fn smth() {
    let bot = teloxide::Bot::from_env();
    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        if !msg.text().unwrap().contains("bad apple") {
            return Ok(());
        }

        let sent_message = bot
            .send_message(msg.chat.id, "ща буит бэд аппл")
            .await
            .unwrap();
        bot.edit_message_text(sent_message.chat.id, sent_message.id, "bruh")
            .await
            .unwrap();
        Ok(())
    })
    .await
}
