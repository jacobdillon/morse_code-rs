#![feature(plugin)]
#![plugin(phf_macros)]

extern crate phf;

// http://morsecode.scphillips.com/morse2.html
static CHARACTER_TO_MORSE: phf::Map<char, &'static str> = phf_map! {
    'a' => ".-",
    'b' => "-...",
    'c' => "-.-.",
    'd' => "-..",
    'e' => ".",
    'f' => "..-.",
    'g' => "--.",
    'h' => "....",
    'i' => "..",
    'j' => ".---",
    'k' => "-.-",
    'l' => ".-..",
    'm' => "--",
    'n' => "-.",
    'o' => "---",
    'p' => ".--.",
    'q' => "--.-",
    'r' => ".-.",
    's' => "...",
    't' => "-",
    'u' => "..-",
    'v' => "...-",
    'w' => ".--",
    'x' => "-..-",
    'y' => "-.--",
    'z' => "--..",
    '0' => "-----",
    '1' => ".----",
    '2' => "..---",
    '3' => "...--",
    '4' => "....-",
    '5' => ".....",
    '6' => "-....",
    '7' => "--...",
    '8' => "---..",
    '9' => "----.",
    '.' => ".-.-.-",
    ',' => "--..--",
    ':' => "---...",
    '?' => "..--..",
    '\'' => ".----.",
    '-' => "-....-",
    '/' => "-..-.",
    '(' => "-.--.-",
    ')' => "-.--.-",
    '"' => ".-..-.",
    '@' => ".--.-.",
    '=' => "-...-",
    ' ' => "/",
};

// Look up a character in the map, returning the morse code translation if found
// and "" if not
fn translate_char_to_morse(c: char) -> Option<String> {
    if CHARACTER_TO_MORSE.contains_key(&c) {
        Some(CHARACTER_TO_MORSE.get(&c).unwrap().to_string())
    } else {
        None
    }
}

fn translate_morse_to_char(s: String) -> Option<char> {
    for key in CHARACTER_TO_MORSE.keys() {
        if CHARACTER_TO_MORSE.get(key).unwrap().to_string() == s {
            return Some(key.clone());
        }
    }
    None
}

// Iterate through a string, translating each character one by one and returing
// a fully translated string
pub fn translate_string(untranslated: String, totext: bool) -> String {
    let mut translated = String::new();
    if !totext {
        for c in untranslated.to_lowercase().chars() {
            match translate_char_to_morse(c) {
                Some(translated_char) => translated = format!("{}{} ", translated, translated_char),
                None => (),
            }
        }
    } else {
        let untranslated_vec: Vec<&str> = untranslated.split(' ').collect();
        for s in &untranslated_vec {
            match translate_morse_to_char(s.to_string().clone()) {
                Some(translated_morse) => translated.push(translated_morse),
                None => (),
            }
        }
    }
    translated
}

#[cfg(test)]
mod test {
    use super::*;

    // Simple test comparing a translated "Hello, world!" to an online translator's
    // interpretation
    #[test]
    fn hello_world_check() {
        assert_eq!(translate_string("Hello, world!".to_string()),
                   ".... . .-.. .-.. --- --..-- / .-- --- .-. .-.. -.. ");
    }
}
