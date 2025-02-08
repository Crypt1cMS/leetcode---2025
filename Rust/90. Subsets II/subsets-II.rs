impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut result: Vec<Vec<i32>> = vec![];
        let mut current: Vec<i32> = vec![];
        Self::backtrack(&nums, 0, &mut current, &mut result);
        result
    }

    pub fn backtrack(nums: &[i32], index: usize, current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {

        result.push(current.clone());

        for i in index..nums.len() {
            if i > index && nums[i] == nums[i - 1] {
                continue;
            }

            current.push(nums[i]);
            Self::backtrack(nums, i + 1, current, result);
            current.pop();    
        }
    }
}
