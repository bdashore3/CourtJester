use rand::prelude::*;

// Switches the case of each character in the word and returns the new word
pub fn get_inverted_string(input: &str) -> String {
    input.chars().map(|x| {
        if x.is_uppercase() {
            x.to_lowercase().collect::<Vec<_>>()[0]
        }
        else if x.is_lowercase() {
            x.to_uppercase().collect::<Vec<_>>()[0]
        }
        else {
            x
        }
    }).collect()
}

/*
 * Makes a spongebob cased string
 * Takes a random value and either makes the letter uppercase or lowercase
 * There is a chance it will output an uppercased or original string due to probability
 */
pub fn get_mock_string(input: &str) -> String {
    input.chars().map(|x| {
        if random() {
            x.to_uppercase().collect::<Vec<_>>()[0]
        }
        else {
            x.to_lowercase().collect::<Vec<_>>()[0]
        }
    }).collect()
}

/*
 * Adds x amount of spaces between each character of the string. Whitespace is trimmed at collection
 * If biggspace is true, add a larger space between each character
 */
pub fn get_spaced_string(input: &str, biggspace: bool) -> String {
    let pass_string: String = input.chars().filter(|c| !c.is_whitespace()).collect();

    pass_string.split("").map(|x|
        if rand::random() {
            if biggspace {
                format!("{}        ", x)
            }

            else {
                format!("{}    ", x)
            }

        } else {
            if biggspace {
                format!("{}     ", x)
            }

            else {
                format!("{}  ", x)
            }
        }).collect::<String>()
}

pub fn get_hacked_string(input: &str) -> String {
    input.chars()
        .map(|x| match x {
            'l'|'L' => '1',
            'e'|'E' => '3',
            'a' | 'A' => '4',
            's' | 'S' => '5',
            't' | 'T' => '7',
            'b' | 'B' => '8',
            'p' | 'P' => '9',
            'o' | 'O' => '0',
            _ => x
        }).collect()
}
