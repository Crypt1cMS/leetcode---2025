pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = vec![];
    arr.sort();

    let min_val = arr.windows(2)
      .map(|w| (w[1] - w[0]).abs())
      .min()
      .unwrap_or(0);

    for i in 0..arr.len() - 1 {
      let diff = (arr[i + 1] - arr[i]).abs();

      if diff == min_val {
        result.push(vec![arr[i], arr[i + 1]]);
      }
    }

    result
}