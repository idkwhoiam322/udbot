use crate::file_handling::functions::*;
use std::fs::File;
use std::io::{Read, Write};
use std::process::Command;
use teloxide::types::{
    InlineQueryResult, InlineQueryResultArticle, InputMessageContent, InputMessageContentText,
    ParseMode,
};

pub fn get_data_from_api(title: &str) -> Vec<InlineQueryResult> { //
    let searchurl:String = format!("https://api.urbandictionary.com/v0/define?term={}", title);
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
    // Change required at start of file
    let new_data = old_data.replace("{\"list\":", "");
    // Change required at end of file
    let new_data = new_data.replace("}]}", "}]");
    // check if this word is present in UD library
    let is_valid_word = new_data.chars().any(|c| matches!(c, 'a'..='z')); // returns true/false

    let mut result: Vec<InlineQueryResult> = Vec::new();
    if is_valid_word {
        _delete_file("input.json".to_string());
        let mut destination_file = File::create("input.json").unwrap();
        destination_file.write(new_data.as_bytes()).unwrap();
        drop(destination_file);

        let json_file = File::open("input.json").unwrap();
        let value: serde_json::Value = serde_json::from_reader(json_file).unwrap();

        let mut i = 0;
        while value[i]["definition"].to_string().ne("null") {
            result.push(get_each_input(
                            &value[i]["word"].to_string(),
                            &value[i]["definition"].to_string(),
                            &value[i]["example"].to_string(),
                            &value[i]["defid"].to_string().to_string()
                        )
                    );
            i = i + 1;
        }
    } else {
        println!("This word was not found in UD library.");
        result.push(get_each_input(
                        title,
                        "This word was not found in UD library.",
                        "",
                        "-1"
                    )
                );
    }

   result
}

fn get_each_input(
    original_title: &str, original_content: &str,
    original_example: &str, id: &str
) -> InlineQueryResult {
    // Don't modify original data
    let mut title = original_title;
    let mut content = String::from(original_content);
    let mut example = String::from(original_example);

    // URL does not need to be set for fallback case
    let mut urlinfo = String::from("");
    // Do NOT cleanup text for fallback case
    if id.ne("-1") {
        // We only want to remove the first and last quotations
        // in each case.
        title = rem_first_and_last_char(title);
//        content = rem_first_and_last_char(&content).to_string();
        example = rem_first_and_last_char(&example).to_string();
        // Replace \" with "
        example = example.replace("\\\"", "\"");
        // Replace \r\n with \n
        example = example.replace("\\r\\n", "\n");
        // We are not showcasing additional definitions
        content = content.replace("[", "");
        content = content.replace("]", "");
        example = example.replace("[", "");
        example = example.replace("]", "");

        // Get rid of fake html tags
        content = content.replace("<", "&lt;");
        content = content.replace(">", "&gt;");
        example = example.replace("<", "&lt;");
        example = example.replace(">", "&gt;");

        // Set URL for getting more information
        urlinfo = format!("<a href='urbanup.com/{}'> More information at urbandictionary.com</a>", id);
    }

    // This is the final text output sent as a message
    let text;
    if example.eq("") {
        // No example available.
        text = format!("ℹ️ <b>Definition of {}:</b>\n{}\n\n{}",
                            title, content, urlinfo);
    } else {
        text = format!("ℹ️ <b>Definition of {}:</b>\n{}\n\n<b>Examples:</b>\n<i>{}</i>\n\n{}",
                            title, content, example, urlinfo);
    }

    // Use HTML formatting for text
    let input = InputMessageContent::Text(
                    InputMessageContentText::new(text.to_owned())
                    .parse_mode(ParseMode::Html)
                    .disable_web_page_preview(true)
                    );

    // .description() is what shows in inline request options
    // Keep it same as content so user is not misled.
    InlineQueryResult::Article(InlineQueryResultArticle
                        ::new(id, title, input)
                        .description(content))
}

fn rem_first_and_last_char(initial_string: &str) -> &str {
    let mut final_string = initial_string.chars();
    final_string.next();
    final_string.next_back();
    final_string.as_str()
}
