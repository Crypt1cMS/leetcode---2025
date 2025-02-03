use std::collections::HashSet;

pub fn repeated_character(s: String) -> char {
    let mut my_set: HashSet<char> = HashSet::new();

    for ch in s.chars() {
        if my_set.contains(&ch) {
            return ch;
        } else {
            my_set.insert(ch); 
        }
    }

    '\0'
}