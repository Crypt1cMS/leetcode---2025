pub fn sum_indices_with_k_set_bits(nums: Vec<i32>, k: i32) -> i32 {
    let mut result: i32 = 0;

    for i in 0..nums.len() {
        if i.count_ones() == k as u32 {
            result += nums[i]
        }
    }

    result
}