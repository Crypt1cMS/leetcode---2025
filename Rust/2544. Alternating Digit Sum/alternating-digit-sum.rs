impl Solution {
    pub fn alternate_digit_sum(n: i32) -> i32 {
      let mut result: i32 = 0;
      let vec: Vec<char> = n.to_string().chars().collect();
  
  
      for (i, ch) in vec.iter().enumerate() {
        let num = ch.to_digit(10).expect("Invalid") as i32;
  
        if i % 2 == 0 {
          result += num
        } else {
            result -= num
        }
  
      }
  
      result
      
    }
}