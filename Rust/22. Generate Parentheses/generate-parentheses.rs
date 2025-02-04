pub fn generate_parenthesis(n: i32) -> Vec<String> {

    let mut result: Vec<String> = Vec::new();
    let mut current: String = String::new();
    Self::backtrack_bacano(n, n, &mut current, &mut result);
    result

}

pub fn backtrack_bacano(open: i32, close: i32, current: &mut String, result: &mut Vec<String>) {

    println!("{:?}", current);
    
    if open == 0 && close == 0 {
        result.push(current.clone());
    }

    if open > 0 {
        current.push('(');
        Self::backtrack_bacano(open - 1, close, current, result);
        current.pop();
    }

    if close > open {
        current.push(')');
        Self::backtrack_bacano(open, close - 1, current, result);
        current.pop();
    }

}