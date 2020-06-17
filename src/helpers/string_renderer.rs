fn get_split_string(message_string: &str) -> Vec<&str> {
    message_string[1..].split_whitespace().collect::<Vec<&str>>()
}

pub fn get_message_word(message_string: &str, index: usize) -> &str {
    get_split_string(message_string)[index]
}

pub fn join_string(message_string: &str) -> String {
    let mut words = get_split_string(message_string);
    words.remove(0);
    words.join(" ")
}

pub fn get_command_length(message_string: &str) -> usize {
    get_split_string(message_string).len()
}