pub fn is_array_special(nums: Vec<i32>) -> bool {
    nums.windows(2).all(|w| (w[0] % 2) != (w[1] % 2))    
}
