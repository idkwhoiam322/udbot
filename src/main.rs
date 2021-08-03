mod file_handling;
mod helper;
mod formatter;
use helper::get_data_from_api;
use std::error::Error;
use teloxide::{
    prelude::*,
    requests::ResponseResult,
    types:: {
        ParseMode, Me,
        MessageKind, MediaKind,
    }
};
use tokio_stream::wrappers::UnboundedReceiverStream;
use chrono::prelude::*;

const START_POST: &str = "Hi! I am Urban Dictionary Bot - made using Rust!
I post word definitions from Urban Dictionary.\n
Usage: @rsurbandictionarybot &lt;word&gt;
Example: @rsurbandictionarybot hello\n
Owner: @idkwhoiam322
Source code: https://github.com/idkwhoiam322/udbot";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    run().await.unwrap();
    Ok(())
}

async fn run() -> Result<(), Box<dyn Error>> {
    let tg_bot = Bot::from_env();
    teloxide::enable_logging!();

    let chat_id:i64 = -1001527066155; // test chat

    let Me { user: bot_user, .. } = tg_bot.get_me().send().await.unwrap();
    let bot_name = bot_user.username.expect("Bots must have usernames");
    let utc_time: DateTime<Utc> = chrono::Utc::now();
    let startpost_text = format!("Starting @{} at <code>{}-{}-{} {}:{}:{} UTC</code>.",
                            bot_name,
                            utc_time.year(), utc_time.month(), utc_time.day(),
                            utc_time.hour(), utc_time.minute(), utc_time.second());

    tg_bot.send_message(chat_id, startpost_text)
        .parse_mode(ParseMode::Html)
        .send()
        .await
        .expect("Message could not be sent");

    Dispatcher::new(tg_bot)
        .messages_handler(|rx: DispatcherHandlerRx<Bot, Message>| {
            UnboundedReceiverStream::new(rx).for_each_concurrent(None, |query| async move {
                match &query.update.chat.kind {
                    teloxide::types::ChatKind::Public(_) => {
                        // Intentionally left empty
                        // Don't post welcome message to groups
                    }
                    teloxide::types::ChatKind::Private(_) => {
                        handle_message(query)
                        .await
                        .log_on_error()
                        .await
                    }
                }
            })
        })
        .inline_queries_handler(move |rx: DispatcherHandlerRx<Bot, InlineQuery>| {
            UnboundedReceiverStream::new(rx).for_each_concurrent(None, move |query| {
                async move {
                    handle_inline_query(query)
                        .await
                        .log_on_error()
                        .await
                }
            })
        })
        .dispatch()
        .await;
    Ok(())
}

async fn handle_message(
    query: UpdateWithCx<Bot, Message>
) -> ResponseResult<()> {

    let mut message_text = String::new();
    // Get message text from DM
    match &query.update.kind {
        MessageKind::Common(message_kind) => {
            match &message_kind.media_kind {
                MediaKind::Text(message) => {
                    message_text = message.text.clone();
                }
                _ => (), // Don't care about non text messages
            }
        },
        _ => (), // Don't care
    };

    if message_text.eq("/start") {
        query
            .answer(START_POST)
            .parse_mode(ParseMode::Html)
            .disable_web_page_preview(true)
            .send()
            .await?;
    }

    // respond(()) is a shortcut for ResponseResult::Ok(()).
    respond(())
}

async fn handle_inline_query(
    query: UpdateWithCx<Bot, InlineQuery>
) -> ResponseResult<()> {
    let InlineQuery {
        id, query: text, ..
    } = query.update;

    if text.is_empty() {
        return respond(());
    }

    println!("{:?}", text);

    let results = get_data_from_api(&text);
    query
        .requester
        .answer_inline_query(id, results)
        .cache_time(0)
        .send()
        .await?;
    respond(())
}
