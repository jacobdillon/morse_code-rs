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
fn translate_char(c: char) -> String {
    CHARACTER_TO_MORSE.get(&c).unwrap_or(&"").to_string()
}

// Iterate through a string, translating each character one by one and returing
// a fully translated string
pub fn translate_string(untranslated: String) -> String {
    let mut translated = String::new();
    for c in untranslated.to_lowercase().chars() {
        let translated_char = translate_char(c);
        translated = {
            if translated_char != "" {
                format!("{}{} ", translated, translated_char)
            } else {
                translated
            }
        };
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
