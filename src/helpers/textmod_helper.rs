use rand::prelude::*;

// Switches the case of each character in the word and returns the new word
pub fn get_inverted_string(input: &str) -> String {
    
    let mut output = String::with_capacity(input.len());

    for x in input.chars() {
        if x.is_uppercase() {
            output.push(x.to_lowercase().collect::<Vec<_>>()[0]);
        }
        else if x.is_lowercase() {
            output.push(x.to_uppercase().collect::<Vec<_>>()[0]);
        }
        else {
            output.push(x);
        }
    }

    output
}

/*
 * Makes a spongebob cased string
 * Takes a random value and either makes the letter uppercase or lowercase
 * There is a chance it will output an uppercased or original string due to probability
 */
pub fn get_mock_string(input: &str) -> String {

    let mut output = String::with_capacity(input.len());

    for x in input.chars() {
        if random() {
            output.push(x.to_uppercase().collect::<Vec<_>>()[0]);
        }
        else {
            output.push(x.to_lowercase().collect::<Vec<_>>()[0]);
        }
    }

    output
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