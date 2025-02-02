pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        
    let mut my_map: HashMap<i32, i32> = HashMap::new();
    let mut result: i32 = 0;

    for num in nums {
        *my_map.entry(num).or_insert(0) += 1;
    }

    for (key, value) in my_map {
        if value == 1 {
            result += key
        }
    }

    result

}