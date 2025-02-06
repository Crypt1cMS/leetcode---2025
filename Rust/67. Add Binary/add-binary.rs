pub fn add_binary(a: String, b: String) -> String {

    let new_a: Vec<char> = a.chars().rev().collect();
    let new_b: Vec<char> = b.chars().rev().collect();

    let mut result = Vec::new();
    let mut carry: i32 = 0;

    let len: usize = std::cmp::max(a.len(), b.len());

    for i in 0..len {
        let bit_a: u32 = new_a.get(i).unwrap_or(&'0').to_digit(2).unwrap();
        let bit_b: u32 = new_b.get(i).unwrap_or(&'0').to_digit(2).unwrap();
    
        let sum: i32 = bit_a as i32 + bit_b as i32 + carry;
        result.push(char::from_digit(sum as u32 % 2, 2).unwrap());
        carry = sum / 2;
    }

    if carry > 0 {
        result.push('1')
    }

    result.iter().rev().collect::<String>()

}