pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
    let mut count: i32 = 0;

    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if nums[i] - nums[j] == k {
                count += 1;
            }
        }
    }

    count 
}