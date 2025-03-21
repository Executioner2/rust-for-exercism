/// Check if the string \"{string}\" contains balanced brackets
pub fn brackets_are_balanced(string: &str) -> bool {
    println!("{string}");
    let mut stack = Vec::new();

    let f = |a: char, b: char| -> bool {
        (a == '(' && b == ')') || (a == '[' && b == ']') || (a == '{' && b == '}')
    };

    for c in string.chars() {
        match c {
            x if ['{', '[', '('].contains(&x) => stack.push(x),
            x if ['}', ']', ')'].contains(&x) => {
                if stack.is_empty() {
                    return false;
                }
                if let Some(last) = stack.last() {
                    if f(*last, x) {
                        stack.pop();
                        continue;
                    }
                }
                break;
            }
            _ => continue,
        }
    }

    stack.is_empty()
}
