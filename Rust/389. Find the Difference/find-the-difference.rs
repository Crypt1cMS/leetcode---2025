impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut char_s: Vec<char> = s.chars().collect();
        let mut char_t: Vec<char> = t.chars().collect();
    
        char_s.sort();
        char_t.sort();
    
        for (i, ch) in char_t.iter().enumerate() {
            if i >= char_s.len() || char_s[i] != *ch {
                return *ch;
            }
        }
        
        panic!()
    }
}