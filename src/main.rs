mod file_handling;
mod helper;
use helper::get_data_from_api;
use std::error::Error;
use teloxide::{
    prelude::*,
    requests::ResponseResult,
    types:: {
        ParseMode, Me,
    }
};
use tokio_stream::wrappers::UnboundedReceiverStream;
use chrono::prelude::*;

const WELCOME_MESSAGE: &str = "Hi! I post word definitions from Urban Dictionary.\n
USAGE: @rsurbandictionarybot your_word_of_choice\n
EXAMPLE: @rsurbandictionarybot hello\n
Owner: @idkwhoiam322\n
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
                handle_message(query)
                    .await
                    .log_on_error()
                    .await
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
    query
        .answer(WELCOME_MESSAGE)
        .parse_mode(ParseMode::Html)
        .send()
        .await?;

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
