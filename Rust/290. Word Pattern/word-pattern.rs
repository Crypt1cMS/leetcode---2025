use std::collections::HashMap;

pub fn word_pattern(pattern: String, s: String) -> bool {
        
    let words: Vec<&str> = s.split_whitespace().collect();
    let mut char_to_word: HashMap<char, &str> = HashMap::new();
    let mut word_to_char: HashMap<&str, char> = HashMap::new();

    if pattern.len() != words.len() {
        return false;
    }

    for (ch, word) in pattern.chars().zip(words.iter()) {
        let word = *word;

        if char_to_word.get(&ch) != Some(&word) {
            if char_to_word.contains_key(&ch) || word_to_char.contains_key(&word) {
                return false;
            }
        }

        char_to_word.insert(ch, word);
        word_to_char.insert(word, ch);

    }

    true

}