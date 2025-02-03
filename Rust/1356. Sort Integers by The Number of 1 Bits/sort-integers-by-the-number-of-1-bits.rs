pub fn sort_by_bits(arr: Vec<i32>) -> Vec<i32> {
        
    let mut new_arr: Vec<i32> = arr;

    new_arr.sort_by(|a, b| {
        let count_a = a.count_ones();
        let count_b = b.count_ones();

        count_a.cmp(&count_b).then_with(|| a.cmp(&b))
    });

    new_arr

}