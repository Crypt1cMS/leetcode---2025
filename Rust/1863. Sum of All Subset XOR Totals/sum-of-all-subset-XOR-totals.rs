impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {  
        let mut result_subsets:  Vec<Vec<i32>> = vec![];
        let mut current: Vec<i32> = vec![];
        Self::backtrack(&nums, 0, &mut current, &mut result_subsets);
        let result: i32 = result_subsets.iter().map(|subset| {subset.iter().fold(0, |acc, &x| acc ^ x)}).sum();
        result
    }

    pub fn backtrack(nums: &[i32], index: usize, current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {

        result.push(current.clone());

        for i in index..nums.len() {
            current.push(nums[i]);
            Self::backtrack(nums, i + 1, current, result);
            current.pop();
        }

    }
}