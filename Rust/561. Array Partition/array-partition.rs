pub fn array_pair_sum(nums: Vec<i32>) -> i32 {

    let mut sorted_nums: Vec<i32> = nums;
    sorted_nums.sort();

    let mut result: i32 = 0;

    for i in (0..sorted_nums.len()).step_by(2) {
        result += sorted_nums[i]
    }

    result
}
