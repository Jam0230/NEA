use regex::Regex;

pub fn remove_whitespace(contents: &mut String) {
    // Removes whispace from string
    contents.retain(|c| !c.is_whitespace());
}

pub fn remove_comments(contents: &String) -> String {
    // removes comments form string
    let comment_regex = Regex::new(r"\/\/.*|\/\*(\n|.)*?\*\/").unwrap(); // fun regex :I

    comment_regex
        .split(contents.as_str())
        .collect::<String>()
        .replace("\n", "\n")
}
