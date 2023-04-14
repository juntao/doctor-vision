use cloud_vision_flows::text_detection;
use lambda_flows::{request_received, send_response};
use tg_flows::{listen_to_update, ChatId, InputFile, Telegram, UpdateKind};
use url::Url;

#[no_mangle]
pub fn run() {
    let telegram_token = std::env::var("telegram_token").unwrap();
    let tele = Telegram::new(telegram_token.clone());

    let crustaceans = "https://images.template.net/wp-content/uploads/2016/03/02064535/informal-lab-report-template.jpg";
    let url = Url::try_from(crustaceans).unwrap();
    //rebuild
    listen_to_update(telegram_token, |update| {
        _ = tele.send_photo(ChatId(6221995180), InputFile::url(url));

        let text = text_detection(String::from_utf8(crustaceans.as_bytes().to_vec()).unwrap());
        match text {
            Ok(r) => send_response(
                200,
                vec![(
                    String::from("content-type"),
                    String::from("text/plain; charset=UTF-8"),
                )],
                r.as_bytes().to_vec(),
            ),
            Err(e) => send_response(
                500,
                vec![(
                    String::from("content-type"),
                    String::from("text/plain; charset=UTF-8"),
                )],
                e.as_bytes().to_vec(),
            ),
        }

        if let UpdateKind::Message(msg) = update.kind {
            let mut text = msg.text().unwrap_or("");
            let chat_id = msg.chat.id;
        }
    });
}