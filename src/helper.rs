use crate::file_handling::functions::*;
use std::fs::File;
use std::io::{Read, Write};
use std::process::Command;
use teloxide::types::{
    InlineQueryResult, InlineQueryResultArticle,
    InputMessageContent, InputMessageContentText,
    InlineKeyboardButton, InlineKeyboardMarkup,
    ParseMode,
};

pub fn get_data_from_api(title: &str) -> Vec<InlineQueryResult> {
    let searchurl;
    if title.contains(" ") {
        // Handle multiple words
        searchurl = format!("https://api.urbandictionary.com/v0/define?term=\"{}\"", title);
    } else {
        searchurl = format!("https://api.urbandictionary.com/v0/define?term={}", title);
    }
    println!("{}", searchurl);
    Command::new("bash")
        .arg("scripts/getapidata.sh")
        .arg(searchurl)
        .output()
        .expect("Script could not be run.");

    let mut source_file = File::open("input.json").unwrap();
    let mut old_data = String::new();
    source_file.read_to_string(&mut old_data).unwrap();
    drop(source_file);

    let json_file = File::open("input.json").unwrap();
    let initial_list: serde_json::Value = serde_json::from_reader(json_file).unwrap();
    let length = initial_list["list"].as_array().unwrap().len();

    // Change required at start of file
    let new_data = old_data.replace("{\"list\":", "");
    // Change required at end of file
    let new_data = new_data.replace("}]}", "}]");
    // check if this word is present in UD library
    let is_valid_word = new_data.chars().any(|c| matches!(c, 'a'..='z')); // returns true/false

    let mut result: Vec<InlineQueryResult> = Vec::new();
    _delete_file("input.json".to_string());
    let mut destination_file = File::create("input.json").unwrap();
    destination_file.write(new_data.as_bytes()).unwrap();
    drop(destination_file);

    if is_valid_word {
        let json_file = File::open("input.json").unwrap();
        let value: serde_json::Value = serde_json::from_reader(json_file).unwrap();

        for i in 0..length {
            result.push(get_each_input(
                            &value,
                            i,
                            length
                        )
                    );
        }
    } else {
        result.push(get_each_input_fallback(
                    title
                    )
                );
    }

    result
}

fn get_each_input_fallback(title: &str) -> InlineQueryResult {
    let content = String::from("This word was not found in UD library.".to_string());
    let id = String::from("-1".to_string());

    let text = format!("ℹ️ <b>Definition of {}:</b>\n{}", title, content);

    let input = InputMessageContent::Text(
                    InputMessageContentText::new(text.to_owned())
                    .parse_mode(ParseMode::Html)
                    .disable_web_page_preview(true)
                );

    InlineQueryResult::Article(InlineQueryResultArticle
            ::new(id, title, input)
            .description(content)
        )
}

fn get_each_input(
    value: &serde_json::Value,
    i: usize,
    _total: usize
) -> InlineQueryResult {
    let mut title = String::from(&value[i]["word"].to_string());
    let mut content = String::from(&value[i]["definition"].to_string());
    let mut example = String::from(&value[i]["example"].to_string());
    let id = String::from(&value[i]["defid"].to_string());

    // Set URL for getting more information
    let ud_shortened_url = String::from(format!("urbanup.com/{}", id));

    // Since we are displaying content separately in the inline query,
    // we have to handle it separately and not as a part of text.
    // Handling it as a part of text also wouldn't help with the individual
    // quotations at the beginning and end of content and example.

    // We only want to remove the first and last quotations
    // in each case.
    title = rem_first_and_last_char(&title).to_string();
    content = rem_first_and_last_char(&content).to_string();
    example = rem_first_and_last_char(&example).to_string();

    content = format_text(content);
    example = format_text(example);

    // This is the final text output sent as a message
    let mut text = format!("ℹ️ <b>Definition of {}:</b>\n{}", title, content);

    // Append examples
    text.push_str(format!("\n\n<b>Examples:</b>\n<i>{}</i>", example).as_str());

    // Use HTML formatting for text
    let input = InputMessageContent::Text(
                    InputMessageContentText::new(text.to_owned())
                    .parse_mode(ParseMode::Html)
                    .disable_web_page_preview(true)
                    );

    let buttons = vec![InlineKeyboardButton::url(
                    "More Information (Urban Dictionary)".to_string(),
                        ud_shortened_url)];

    let inline_keyboard = InlineKeyboardMarkup::default()
                            .append_row(buttons);

    // .description() is what shows in inline request options
    // Keep it same as content so user is not misled.
    InlineQueryResult::Article(InlineQueryResultArticle
                        ::new(id, title, input)
                        .description(content)
                        .reply_markup(inline_keyboard) // Inline Keyboard only if we actually have a result
                    )
}

fn rem_first_and_last_char(initial_string: &str) -> &str {
    let mut final_string = initial_string.chars();
    final_string.next();
    final_string.next_back();
    final_string.as_str()
}

fn format_text(mut text: String) -> String {
    // Replace \" with "
    text = text.replace("\\\"", "\"");

    // Replace \r\n with \n
    text = text.replace("\\r", "\r");
    text = text.replace("\\n", "\n");

    // We are not showcasing additional definitions
    text = text.replace("[", "");
    text = text.replace("]", "");

    // Get rid of fake html tags
    text = text.replace("<", "&lt;");
    text = text.replace(">", "&gt;");

    text
}
