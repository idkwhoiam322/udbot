mod file_handling;
mod logger;
mod helper;
mod formatter;
use helper::{
    get_top_result,
    get_inline_results,
};
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
I post word definitions from Urban Dictionary.
See /help for information on how to use this bot.

Owner: @idkwhoiam322
Source code: https://github.com/idkwhoiam322/udbot";

const HELP_POST: &str = "List of available commands:
/help - This message.
/start - About this bot.
/wotd or /wordoftheday - Get the word of the day.
/random - Get a random word!

Use this bot inline:
Usage: @rsurbandictionarybot &lt;word&gt;
Example: @rsurbandictionarybot hello

Report issues to: @idkwhoiam322";

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

    logger::run(&tg_bot, chat_id).await;
    log::info!("Logging completed. Queries will now be accepted.");

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
    // Assume not a text query by default
    let mut is_text_query = false;
    // User ID will be obtained from message
    let mut user_id = 0;

    // Get message text from DM
    if let MessageKind::Common(message_kind) = &query.update.kind {
        if let MediaKind::Text(message) = &message_kind.media_kind {
            message_text = message.text.clone();
            is_text_query = true;
        };
        if let Some(user) = &message_kind.from {
            user_id = user.id;
        }
    };

    if is_text_query {
        let mut result = String::new();

        match message_text.as_str() {
            "/start" => {
                result = START_POST.to_string();
            },
            "/help" => {
                result = HELP_POST.to_string();
            },
            "/wotd" | "/wordoftheday" => {
                result = get_top_result(&message_text, user_id);
            },
            "/random" => {
                result = get_top_result(&message_text, user_id);
            }
            _ => {
                if !message_text.contains("??????") {
                    result = get_top_result(&message_text, user_id);
                } else if message_text.contains("??????") {
                    log::info!("Ignoring InlineQuery sent in DM.");
                } else {
                    log::info!("Ignoring special request sent in DM: {}", message_text);
                }
            }
        }

        if result.ne("") {
            query
                .answer(result)
                .parse_mode(ParseMode::Html)
                .disable_web_page_preview(true)
                .send()
                .await?;
        }
    } else {
        log::info!("Ignoring non text request sent in DM.");
    }

    // respond(()) is a shortcut for ResponseResult::Ok(()).
    respond(())
}

async fn handle_inline_query(
    query: UpdateWithCx<Bot, InlineQuery>
) -> ResponseResult<()> {
    let InlineQuery {
        id, query: text, from, ..
    } = query.update;

    let user_id = from.id;
    let query_id = id.parse().unwrap_or(0);

    if text.is_empty() {
        return respond(());
    }

    let results = get_inline_results(&text, user_id, query_id);
    query
        .requester
        .answer_inline_query(id, results)
        .cache_time(0)
        .send()
        .await?;
    respond(())
}
