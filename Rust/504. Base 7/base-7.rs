struct Solution{}

impl Solution {
    pub fn convert_to_base7(num: i32) -> String {  
        
        if num == 0 {
            return "0".to_string();
        }
        
        let mut num = num;
        let mut result = String::new();
        let negative = num < 0;

        if negative {
        num = -num
        }
        
        while num > 0 {
        result.push_str(&(num % 7).to_string());
        num /= 7;
        }

        if negative {
        result.push('-');
        }

        result.chars().rev().collect()
    }
}