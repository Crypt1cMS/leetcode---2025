

pub fn difference_of_sum(nums: Vec<i32>) -> i32 {

    let sum_of_nums: i32 = nums.iter().sum();

    let sum_of_digits: i32 = nums.iter().map(|&n| {
        let mut num = n;
        let mut sum: i32 = 0;
        while num > 0 {
            sum += num % 10;
            num /= 10;
        }
        sum
    }).sum();

    (sum_of_nums - sum_of_digits ).abs()

}

fn main() {
    let nums: Vec<i32> = vec![1,15,6,3];
    println!("{:?}", difference_of_sum(nums));
}