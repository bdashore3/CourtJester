use rand::prelude::*;

// Switches the case of each character in the word and returns the new word
pub fn get_inverted_string(input: &str) -> String {
    input
        .chars()
        .map(|x| {
            if x.is_uppercase() {
                x.to_lowercase().collect::<Vec<_>>()[0]
            } else if x.is_lowercase() {
                x.to_uppercase().collect::<Vec<_>>()[0]
            } else {
                x
            }
        })
        .collect()
}

/*
 * Makes a spongebob cased string
 * Takes a random value and either makes the letter uppercase or lowercase
 * There is a chance it will output an uppercased or original string due to probability
 */
pub fn get_mock_string(input: &str) -> String {
    input
        .chars()
        .map(|x| {
            if random() {
                x.to_uppercase().collect::<Vec<_>>()[0]
            } else {
                x.to_lowercase().collect::<Vec<_>>()[0]
            }
        })
        .collect()
}

/*
 * Adds x amount of spaces between each character of the string. Whitespace is trimmed at collection
 * If biggspace is true, add a larger space between each character
 */
pub fn get_spaced_string(input: &str, biggspace: bool) -> String {
    let pass_string: String = input.chars().filter(|c| !c.is_whitespace()).collect();

    pass_string
        .split("")
        .map(|x| {
            if rand::random() {
                if biggspace {
                    format!("{}         ", x)
                } else {
                    format!("{}    ", x)
                }
            } else if biggspace {
                format!("{}     ", x)
            } else {
                format!("{}  ", x)
            }
        })
        .collect::<String>()
}

pub fn get_hacked_string(input: &str) -> String {
    input
        .chars()
        .map(|x| match x {
            'l' | 'L' => '1',
            'e' | 'E' => '3',
            'a' | 'A' => '4',
            's' | 'S' => '5',
            't' | 'T' => '7',
            'b' | 'B' => '8',
            'p' | 'P' => '9',
            'o' | 'O' => '0',
            _ => x,
        })
        .collect()
}

pub fn get_uwu_string(input: &str) -> String {
    let mut words = Vec::new();

    for word in input.split(' ') {
        match word {
            "you" => words.push(word.to_string()),
            "uwu" => words.push(word.to_string()),
            "owo" => words.push(word.to_string()),
            "one" => words.push("wone".to_string()),
            "two" => words.push("two".to_string()),
            "three" => words.push("thwee".to_string()),
            "lewd" => words.push("lewd".to_string()),
            "cute" => words.push("cwute".to_string()),
            _ => {
                if word.len() > 2 {
                    let mut w = word
                        .to_string()
                        .replace("our", "\u{200b}w")
                        .replace("r", "w")
                        .replace("R", "W")
                        .replace("l", "w")
                        .replace("L", "W")
                        .replace("ar", " ")
                        .replace("ai", "+")
                        .replace("a", "wa")
                        .replace("wawa", "waa")
                        .replace(" ", "aw")
                        .replace("ie", " ")
                        .replace("i", "wi")
                        .replace("wiwi", "wii")
                        .replace(" ", "ie")
                        .replace("+", "ai")
                        .replace("ge", " ")
                        .replace("ke", "+")
                        .replace("e", "we")
                        .replace("wewe", "wee")
                        .replace(" ", "ge")
                        .replace("+", "ke")
                        .replace("ou", "=")
                        .replace("cho", " ")
                        .replace("o", "wo")
                        .replace("wowo", "woo")
                        .replace(" ", "cho")
                        .replace("gu", " ")
                        .replace("qu", "+")
                        .replace("u", "wu")
                        .replace("wuwu", "wuu")
                        .replace(" ", "gu")
                        .replace("+", "qu")
                        .replace("=", "ouw");

                    if !word.starts_with('A') {
                        w = w.replace("A", "WA");
                    } else {
                        w = w.replace("A", "Wa");
                    }

                    if !word.starts_with('E') {
                        w = w.replace("E", "WE");
                    } else {
                        w = w.replace("E", "We");
                    }

                    if !word.starts_with('I') {
                        w = w.replace("I", "WI");
                    } else {
                        w = w.replace("I", "Wi");
                    }

                    if !word.starts_with('O') {
                        w = w.replace("O", "WO");
                    } else {
                        w = w.replace("O", "Wo");
                    }

                    if !word.starts_with('U') {
                        w = w.replace("U", "WU");
                    } else {
                        w = w.replace("U", "Wu");
                    }

                    w = w.replace("\u{200b}", "ouw").replace("@", "@\u{200b}");

                    words.push(w);
                } else {
                    words.push(word.to_string());
                }
            }
        }

        words.push("uwu".to_string());
    }

    words
        .join(" ")
        .replace("ww", "w")
        .replace("Ww", "W")
        .replace("WW", "W")
}
