pub fn is_power_of_three(n: i32) -> bool {

    let mut num_divisible: i32 = n;

    if n <= 0 {
        return false;
    }

    while num_divisible % 3 == 0 {
        num_divisible /= 3;
    }

    num_divisible == 1

}