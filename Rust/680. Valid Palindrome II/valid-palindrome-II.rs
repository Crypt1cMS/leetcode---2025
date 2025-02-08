impl Solution {
    pub fn is_palindrome(s: &str, left: usize, right: usize) -> bool {
    
        let s_chars: Vec<char> = s.chars().collect();
        let (mut l, mut r) = (left, right);

        while l < r {
            if s_chars[l] != s_chars[r] {
                return false;
            }

            l += 1;
            r -= 1;
        }

        true

    }

    pub fn valid_palindrome(s: String) -> bool {
        
        let s_chars: Vec<char> = s.chars().collect();
        let n = s.len();
        let (mut left, mut right) = (0, n - 1);

            while left< right {
                if s_chars[left] != s_chars[right] {
                    return Self::is_palindrome(&s, left + 1, right) || Self::is_palindrome(&s, left, right - 1);
                }

                left += 1;
                right -= 1;
            }
            
        true
    }
}