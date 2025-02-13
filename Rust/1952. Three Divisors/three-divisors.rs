pub fn is_three(n: i32) -> bool {
      
    let mut nums: Vec<i32> = vec![];

    for i in 1..=n {
        if n % i == 0 {
            nums.push(i);
        }
    }

    nums.len() == 3
}