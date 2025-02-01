pub fn are_numbers_ascending(s: String) -> bool {
        
    let nums: Vec<i32> = s.split_whitespace().filter_map(|n| n.parse::<i32>().ok()).collect();
    
    nums.windows(2).all(|n| n[0] < n[1])
}