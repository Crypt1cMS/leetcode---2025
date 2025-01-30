    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut vector: Vec<i32> = Vec::new();

        for i in 0..nums.len() {
        let mut n = 0;

            for j in 0..nums.len() {
                if nums[i] > nums[j] {
                    n += 1
                }
            }

            vector.push(n);
        }

        vector
    }