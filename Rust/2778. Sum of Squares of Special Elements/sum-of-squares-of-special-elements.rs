pub fn sum_of_squares(nums: Vec<i32>) -> i32 {

    let n = nums.len();
    let mut result: i32 = 0;

    for i in 1..=n {
        if n % i == 0 {
            result += nums[i - 1] * nums[i - 1]
        };
    }

    result

}