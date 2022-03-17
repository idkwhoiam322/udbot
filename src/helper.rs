use crate::file_handling::functions::*;
use crate::formatter::text_cleanup;
use std::fs::File;
use std::io::{Read, Write};
use curl::easy::Easy;
use teloxide::types::{
    InlineQueryResult, InlineQueryResultArticle,
    InputMessageContent, InputMessageContentText,
    InlineKeyboardButton, InlineKeyboardMarkup,
    ParseMode,
};

/*
 * Information about functions:
 * get_top_result() - Gets only the top result
 * get_inline_results() - Gets all possible results for inline queries
 */

pub fn get_top_result(title: &str, user_id: i64) -> String {
    let file_name = &format!("PMQuery_{}.json", user_id)[..];

    let searchurl = match title {
                        "/wotd" | "/wordoftheday" => format!("https://api.urbandictionary.com/v0/words_of_the_day"),
                        "/random" => format!("https://api.urbandictionary.com/v0/random"),
                        _ => get_searchurl(title),
                    };

    log::info!("Query: {}", title);

    let mut temp_data = Vec::new();
    let mut easy = Easy::new();
    easy.url(searchurl.as_str()).unwrap();
    let mut transfer = easy.transfer();
    transfer.write_function(|data| {
        temp_data.extend_from_slice(data);
        let api_data = match std::str::from_utf8(&temp_data) {
            Ok(value) => value,
            Err(_) => "",
        };
        delete_file(file_name);
        create_file(file_name);
        write_to_file(file_name, api_data);
        Ok(data.len())
    }).unwrap();
    transfer.perform().unwrap();

    let mut source_file = File::open(file_name).unwrap();
    let mut old_data = String::new();
    source_file.read_to_string(&mut old_data).unwrap();
    drop(source_file);

    let json_file = File::open(file_name).unwrap();
    let initial_list: serde_json::Value = serde_json::from_reader(json_file).unwrap();
    let length = match initial_list["list"].as_array() {
                        Some(arr) => arr.len(),
                        None => {
                            log::debug!("{} could not get a list from {}", file_name, get_searchurl(title));
                            0
                        }
                    };

    let mut new_data; // modified json
    let is_valid_word; // In case the query is invalid or does not exist in UD API

    if length != 0 {
        // Change required at start of file
        new_data = old_data.replace("{\"list\":", "");
        // Change required at end of file
        new_data = new_data.replace("}]}", "}]");
        // check if this word is present in UD API
        is_valid_word = new_data.chars().any(|c| matches!(c, 'a'..='z')); // returns true/false

        delete_file(file_name);
        let mut destination_file = File::create(file_name).unwrap();
        destination_file.write(new_data.as_bytes()).unwrap();
        drop(destination_file);
    } else {
        is_valid_word = false;
    }

    let mut result = String::new();
    if is_valid_word {
        let json_file = File::open(file_name).unwrap();
        let value: serde_json::Value = serde_json::from_reader(json_file).unwrap();

        result.push_str(&get_each_input(&value, 0, length));
    } else {
        result.push_str(&get_each_input_fallback(title));
    }

    // Delete file after it is used
    delete_file(file_name);

    result
}

fn get_each_input_fallback(title: &str) -> String {
    let content = String::from("This word was not found in UD API.".to_string());

    let mut text = format!("ℹ️ <b>Definition of {}:</b>\n{}", title, content);

    let ud_url = String::from(format!("https://www.urbandictionary.com/define.php?term={}", title));

    text.push_str(format!("\n\n<a href='{}'>Search urbandictionary.com</a>", ud_url).as_str());

    text
}

fn get_each_input(
    value: &serde_json::Value,
    i: usize,
    _total: usize
) -> String {
    let mut title = String::from(&value[i]["word"].to_string());
    let mut content = String::from(&value[i]["definition"].to_string());
    let mut example = String::from(&value[i]["example"].to_string());
    let id = String::from(&value[i]["defid"].to_string());

    // Data can be cleaned up in its own function
    text_cleanup(&mut title);
    text_cleanup(&mut content);
    text_cleanup(&mut example);

    // Set URL for getting more information
    let ud_shortened_url = String::from(format!("urbanup.com/{}", id));

    // This is the final text output sent as a message
    let mut text = String::new();

    text.push_str(format!("ℹ️ <b>Definition of {}:</b>\n{}", title, content).as_str());

    // Append examples if ( and only if ) there are any
    if example.ne("") {
        text.push_str(format!("\n\n📝 <b>Examples:</b>\n<i>{}</i>", example).as_str());
    }

    // Append source
    text.push_str(format!("\n\n<a href='{}'>Source (Urban Dictionary)</a>", ud_shortened_url).as_str());

    text.push_str(format!("\n\nTo get more results, use the inline query method. See /help for more info.").as_str());

    text
}

pub fn get_inline_results(title: &str, user_id: i64, query_id: i64) -> Vec<InlineQueryResult> {
    let file_name = &format!("InlineQuery_{}_{}.json", user_id, query_id)[..];
    let searchurl = get_searchurl(title);

    log::info!("Inline Query: {}", title);

    let mut temp_data = Vec::new();
    let mut easy = Easy::new();
    easy.url(searchurl.as_str()).unwrap();
    let mut transfer = easy.transfer();
    transfer.write_function(|data| {
        temp_data.extend_from_slice(data);
        let api_data = match std::str::from_utf8(&temp_data) {
            Ok(value) => value,
            Err(_) => "",
        };
        delete_file(file_name);
        create_file(file_name);
        write_to_file(file_name, api_data);
        Ok(data.len())
    }).unwrap();
    transfer.perform().unwrap();

    let mut source_file = File::open(file_name).unwrap();
    let mut old_data = String::new();
    source_file.read_to_string(&mut old_data).unwrap();
    drop(source_file);

    let json_file = File::open(file_name).unwrap();
    let initial_list: serde_json::Value = serde_json::from_reader(json_file).unwrap();
    let length = match initial_list["list"].as_array() {
                        Some(arr) => arr.len(),
                        None => {
                            log::debug!("{} could not get a list from {}", file_name, get_searchurl(title));
                            0
                        }
                    };

    let mut new_data; // modified json
    let is_valid_word; // In case the query is invalid or does not exist in UD API

    if length != 0 {
        // Change required at start of file
        new_data = old_data.replace("{\"list\":", "");
        // Change required at end of file
        new_data = new_data.replace("}]}", "}]");
        // check if this word is present in UD API
        is_valid_word = new_data.chars().any(|c| matches!(c, 'a'..='z')); // returns true/false

        delete_file(file_name);
        let mut destination_file = File::create(file_name).unwrap();
        destination_file.write(new_data.as_bytes()).unwrap();
        drop(destination_file);
    } else {
        is_valid_word = false;
    }

    let mut result: Vec<InlineQueryResult> = Vec::new();
    if is_valid_word {
        let json_file = File::open(file_name).unwrap();
        let value: serde_json::Value = serde_json::from_reader(json_file).unwrap();

        for i in 0..length {
            result.push(get_each_input_inline(&value, i, length));
        }
    } else {
        result.push(get_each_input_fallback_inline(title));
    }

    // Delete file after it is used
    delete_file(file_name);

    result
}

fn get_each_input_fallback_inline(title: &str) -> InlineQueryResult {
    let mut content = String::from("This word was not found in UD API.".to_string());
    let id = String::from("-1".to_string());

    let text = format!("ℹ️ <b>Definition of {}:</b>\n{}", title, content);
    content.push_str("\n\nClick here for an option to search anyways.");

    let input = InputMessageContent::Text(
                    InputMessageContentText::new(text.to_owned())
                    .parse_mode(ParseMode::Html)
                    .disable_web_page_preview(true)
                );

    let ud_url = String::from(format!("https://www.urbandictionary.com/define.php?term={}", title));

    let buttons = vec![
                    InlineKeyboardButton::url(
                        "Search urbandictionary.com".to_string(), ud_url)
                    ];

    let inline_keyboard = InlineKeyboardMarkup::default()
                            .append_row(buttons);

    InlineQueryResult::Article(InlineQueryResultArticle
                                    ::new(id, title, input)
                                    .description(content)
                                    .reply_markup(inline_keyboard)
                            )
}

fn get_each_input_inline(
    value: &serde_json::Value,
    i: usize,
    _total: usize
) -> InlineQueryResult {
    let mut title = String::from(&value[i]["word"].to_string());
    let mut content = String::from(&value[i]["definition"].to_string());
    let mut example = String::from(&value[i]["example"].to_string());
    let id = String::from(&value[i]["defid"].to_string());

    // Data can be cleaned up in its own function
    text_cleanup(&mut title);
    text_cleanup(&mut content);
    text_cleanup(&mut example);

    // Set URL for getting more information
    let ud_shortened_url = String::from(format!("urbanup.com/{}", id));

    // This is the final text output sent as a message
    let mut text = format!("ℹ️ <b>Definition of {}:</b>\n{}", title, content);

    // Append examples if ( and only if ) there are any
    if example.ne("") {
        text.push_str(format!("\n\n📝 <b>Examples:</b>\n<i>{}</i>", example).as_str());
    }

    // Use HTML formatting for text
    let input = InputMessageContent::Text(
                    InputMessageContentText::new(text.to_owned())
                    .parse_mode(ParseMode::Html)
                    .disable_web_page_preview(true)
                    );

    let buttons = vec![
                    InlineKeyboardButton::url(
                        "More Information (Urban Dictionary)".to_string(), ud_shortened_url)
                    ];

    let inline_keyboard = InlineKeyboardMarkup::default()
                            .append_row(buttons);

    // .description() is what shows in inline request options
    // Keep it same as content so user is not misled.
    InlineQueryResult::Article(InlineQueryResultArticle
                                    ::new(id, title, input)
                                    .description(content)
                                    .reply_markup(inline_keyboard)
                            )
}

fn get_searchurl(title: &str) -> String {
    let searchurl;
    if title.contains(" ") {
        // Handle multiple words
        let modified_search_query = title.replace(" ", "%20");
        searchurl = format!("https://api.urbandictionary.com/v0/define?term=\"{}\"", modified_search_query);
    } else {
        searchurl = format!("https://api.urbandictionary.com/v0/define?term={}", title);
    }
    log::info!("{}", searchurl);
    searchurl
}
