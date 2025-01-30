pub fn count_pairs(nums: Vec<i32>, target: i32) -> i32 {
        
    let mut count = 0;

    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if nums[i] + nums[j] < target {
                count += 1
            }
        }
    }

    count

}

fn main() {
    let nums = vec![-1,1,2,3,1];
    let target = 2;
    println!("{}", count_pairs(nums, target));
}