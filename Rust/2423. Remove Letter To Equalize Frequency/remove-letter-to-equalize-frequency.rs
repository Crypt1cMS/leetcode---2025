use std::collections::{HashMap, HashSet};

pub fn equal_frequency(word: String) -> bool {
    let mut my_map: HashMap<char, i32> = HashMap::new();


    for ch in word.chars() {
        *my_map.entry(ch).or_insert(0) += 1;
    }

    for (&key, &val) in my_map.iter() {

        if val > 0 {
            let mut temp_map = my_map.clone();
            *temp_map.get_mut(&key).unwrap() -= 1;


            if temp_map[&key] == 0 {
                temp_map.remove(&key);
            }

            let unique: HashSet<_> = temp_map.values().collect();

            if unique.len() == 1 {
                return true;
            }
        }
    }

    false
}