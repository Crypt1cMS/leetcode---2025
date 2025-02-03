use std::collections::HashMap;

pub fn are_occurrences_equal(s: String) -> bool {

    let mut my_map: HashMap<char, i32> = HashMap::new();
    let s_chars = s.chars();

    for ch in s_chars{
        *my_map.entry(ch).or_insert(0) += 1;
    }

    let mut iter = my_map.values();

    if let Some(first_val) = iter.next() {
        iter.all(|&val| val == *first_val)
    } else {
        true
    }

}