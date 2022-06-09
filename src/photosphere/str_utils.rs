// All `to_*_case` function is this crate assumes that
// the string input is snake_case

fn capitalize(str: &&str) -> String {
    let mut chars = str.chars();

    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

pub fn to_title(str: &str) -> String {
    str.split('_')
        .collect::<Vec<&str>>()
        .iter()
        .map(capitalize)
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn to_pascal_case(str: &str) -> String {
    str.split('_')
        .collect::<Vec<&str>>()
        .iter()
        .map(capitalize)
        .collect::<Vec<String>>()
        .join("")
}

pub fn to_kebab_case(str: &str) -> String {
    str.replace('_', "-")
}

pub fn is_lower_alphanumeric(ch: char) -> bool {
    (ch.is_alphanumeric() && ch.is_lowercase()) || ch.eq(&'_')
}
