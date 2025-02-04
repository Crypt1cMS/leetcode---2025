pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {

    let mut max_sum: i32 = 0;
    let mut current_sum: i32 = nums[0];

    for i in 1..nums.len() {
        if nums[i] > nums[i - 1] {
            current_sum += nums[i]
        } else {
            max_sum = max_sum.max(current_sum);
            current_sum = nums[i];
        }
    }

    max_sum.max(current_sum)

}