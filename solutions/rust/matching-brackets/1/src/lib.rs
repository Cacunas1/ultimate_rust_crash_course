pub fn brackets_are_balanced(string: &str) -> bool {
    // todo!("Check if the string \"{string}\" contains balanced brackets");
    let mut stack: Vec<char> = Vec::new();

    for c in string
        .chars()
        .filter(|c| ['(', '[', '{', '}', ']', ')'].contains(c))
    {
        if ['(', '[', '{'].contains(&c) {
            stack.push(c);
        } else if ['}', ']', ')'].contains(&c) {
            match c {
                '}' => {
                    if *stack.last().unwrap_or(&'-') == '{' {
                        stack.pop();
                    } else {
                        return false;
                    }
                }
                ']' => {
                    if *stack.last().unwrap_or(&'-') == '[' {
                        stack.pop();
                    } else {
                        return false;
                    }
                }
                ')' => {
                    if *stack.last().unwrap_or(&'-') == '(' {
                        stack.pop();
                    } else {
                        return false;
                    }
                }
                _ => {
                    unreachable!();
                }
            }
        }
    }

    stack.is_empty()
}
