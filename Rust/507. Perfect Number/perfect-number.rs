struct Solution{}

impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {

        if num == 1 {
            return false;
        }

        let mut result = 1;
        let sqrt = (num as f64).sqrt() as i32;

        for i in 2..=sqrt {

            if num % i == 0 {
                result += i;

                if i != num / i {
                    result += num / i
                }
            }

        }

        result == num

    }
}