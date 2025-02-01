use std::collections::HashMap;

pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
        
    let mut my_map: HashMap<i32, i32> = HashMap::new();

    for &num in &nums{
        *my_map.entry(num).or_insert(0) += 1;
    }

    println!("{:?}", my_map);

    let mut sorted = nums.clone();

    sorted.sort_by(|a, b| {
        let frequency_a = my_map.get(a).unwrap();
        let frequency_b = my_map.get(b).unwrap();

        if frequency_a != frequency_b {
            frequency_a.cmp(frequency_b)
        } else {
            b.cmp(a)
        }
    });

    sorted

}