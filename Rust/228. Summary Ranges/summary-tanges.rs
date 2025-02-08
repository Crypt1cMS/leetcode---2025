impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        let n_len: usize = nums.len();

        if n_len == 0 {
            return result;
        }

        let mut start: i32 = nums[0];

        for i in 1..=n_len {

            if i == n_len ||  nums[i] != nums[i - 1] + 1 {
                
                if start == nums[i - 1] {
                    result.push(format!("{}", start));
                } else {
                    result.push(format!("{}->{}", start, nums[i - 1]));
                }

                if i < n_len {
                    start = nums[i]
                } 
            }
        }

        result
    }
}