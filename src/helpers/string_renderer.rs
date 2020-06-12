pub fn get_split_string(message_string: &str) -> Vec<&str> {
    message_string[1..].split_whitespace().collect::<Vec<&str>>()
}

pub fn get_command(message_string: &str) -> &str {
    get_split_string(message_string)[0]
}