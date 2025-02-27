pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {

    let mut result: Vec<i32> = vec![];

    for i in 0..n as usize {
        result.push(nums[i]);
        result.push(nums[i + n as usize]);
    }

    result

}