pub fn halves_are_alike(s: String) -> bool {
    let mid_point: usize = s.len() / 2;
    let first_half: &str = &s[..mid_point];
    let second_half: &str = &s[mid_point..];
    let vowels: Vec<char> = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

    let mut count_first: i32 = 0;
    let mut count_second: i32 = 0;

    for ch in first_half.chars() {
        if vowels.contains(&ch) {
            count_first += 1;
        }
    }

    for ch in second_half.chars() {
        if vowels.contains(&ch) {
            count_second += 1;
        }
    }

    count_first == count_second
}