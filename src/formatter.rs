pub fn text_cleanup(text: &mut String) {
    // Since we are displaying content separately in the inline query,
    // we have to handle it separately and not as a part of text.
    // Handling it as a part of text also wouldn't help with the individual
    // quotations at the beginning and end of content and example.

    // We only want to remove the first and last quotations
    // in each case.
    *text = rem_first_and_last_char(&text).to_string();

    *text = prevent_htmlisation(text.to_string());
}

fn rem_first_and_last_char(initial_string: &str) -> &str {
    let mut final_string = initial_string.chars();
    final_string.next();
    final_string.next_back();
    final_string.as_str()
}

fn prevent_htmlisation(mut text: String) -> String {
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
