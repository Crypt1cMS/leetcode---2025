pub fn interpret(command: String) -> String {
        
    let mut new_command: String = String::new();
    let text: Vec<char> = command.chars().collect();

    for i in 0..text.len() - 1 {
        let current: char = text[i];
        let next: char = text[i + 1];

        if current == '(' && next == ')' {
            new_command += "o";
            continue;
        } else if current == ')' || current == '(' {
            continue;
        } else {
            new_command.push(current);
        }
    }

    if text[text.len() - 1] != '(' && text[text.len() - 1] != ')' {
        new_command.push(text[text.len() - 1]);
    }

    new_command

}