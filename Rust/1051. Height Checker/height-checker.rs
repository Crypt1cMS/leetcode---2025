pub fn height_checker(heights: Vec<i32>) -> i32 {

    let heights_len = heights.len();
    let mut expected: Vec<i32> = heights.clone();
    expected.sort();

    let mut count: i32 = 0;

    for i in 0..heights_len {

        if heights[i] != expected[i] {
            count += 1;
        }
    
    }

    count
}