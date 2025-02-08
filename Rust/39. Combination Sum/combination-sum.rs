impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        
        let mut result: Vec<Vec<i32>> = vec![];
        let mut current: Vec<i32> = vec![];
        Self::bactrack(&candidates, 0, target, &mut current, &mut result);
        result

    }

    pub fn bactrack(candidates: &[i32], index: usize, target: i32, current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {

        let sum: i32 = current.iter().sum();

        if sum == target {
            result.push(current.clone());
            return;
        }

        if sum > target {
            return;
        }

        for i in index..candidates.len() {
            current.push(candidates[i]);
            Self::bactrack(candidates, i, target, current, result);
            current.pop();    
        }
    }
}
