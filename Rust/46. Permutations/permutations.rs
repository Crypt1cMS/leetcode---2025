impl Solution {
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut current: Vec<i32> = vec![];
        let mut result = vec![];
        let mut used = vec![false; nums.len()];
        nums.sort();
        Self::backtrack(&nums, &mut used, &mut current, &mut result);
        
        result
    }

    pub fn backtrack(nums: &Vec<i32>, used: &mut Vec<bool>, current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>,) {

        if current.len() == nums.len() {
            result.push(current.clone());
            return;
        }

        for i in 0..nums.len() {

            if used[i] {
                continue;
            }

            if i > 0 && nums[i] == nums[i - 1] && !used[i - 1] {
                continue;
            }

            used[i] = true;
            current.push(nums[i]);
            Self::backtrack(nums, used, current, result);
            current.pop();
            used[i] = false;
        }
        
    }
}