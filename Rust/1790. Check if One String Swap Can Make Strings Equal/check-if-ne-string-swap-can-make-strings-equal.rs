pub fn are_almost_equal(s1: String, s2: String) -> bool {

    if s1 == s2 {
        return  true;
    }

    let mut new_s1: Vec<char> = s1.chars().collect(); 
    let mut new_s2: Vec<char> = s2.chars().collect();

    let mut diff_indices = Vec::new();

    for i in 0..new_s1.len() {
        if new_s1[i] != new_s2[i] {
            diff_indices.push(i);
        }
    }

    if diff_indices.len() == 2 {
        let i = diff_indices[0];
        let j = diff_indices[1];

        new_s1.swap(i, j);

        return new_s1 == new_s2;
    }

    false
}