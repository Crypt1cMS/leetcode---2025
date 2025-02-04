pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
        
    let mut result: Vec<i32> = vec![];    

    for i in 0..index.len() {
        result.insert(index[i] as usize, nums[i]);
    }

    result

}