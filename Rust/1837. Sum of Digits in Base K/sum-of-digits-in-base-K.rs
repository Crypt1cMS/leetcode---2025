
struct Solution {}

impl Solution {
  pub fn sum_base(mut n: i32, k: i32) -> i32 {
      
    let mut result: i32 = 0;

    while n > 0 {
        let remainder = n % k;
        println!("{:?}", remainder);
        result += remainder;
        n /= k
    }

    result
  }
}