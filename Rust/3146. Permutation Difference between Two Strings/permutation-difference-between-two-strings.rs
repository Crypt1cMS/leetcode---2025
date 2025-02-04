pub fn find_permutation_difference(s: String, t: String) -> i32 {

    let mut count:i32 = 0;

    for (i, c1) in s.chars().enumerate() {
        for (j, c2) in t.chars().enumerate() {

            if c1 == c2 {
                count += (i as i32 - j as i32).abs();
            }

        }
    }

    count

}