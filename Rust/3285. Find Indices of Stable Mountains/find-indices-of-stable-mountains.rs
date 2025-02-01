pub fn stable_mountains(height: Vec<i32>, threshold: i32) -> Vec<i32> {
        
    let mut result: Vec<i32> = vec![];

    for i in 1..height.len() {
        if height[i - 1] > threshold {
            result.push(i as i32);
        }
    }

    result

}