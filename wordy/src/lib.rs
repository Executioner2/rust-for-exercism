pub fn answer(command: &str) -> Option<i32> {
    if !command.starts_with("What is ") || !command.ends_with('?') {
        return None;
    }

    let command = &command[8..command.len() - 1];

    let tokens: Vec<&str> = command.split_whitespace().collect();

    let mut result = match tokens[0].parse::<i32>() {
        Ok(num) => num,
        Err(_) => return None,
    };

    let mut i = 1;
    while i < tokens.len() {
        let operator = tokens[i];
        i += 1;

        if operator == "multiplied" || operator == "divided" {
            if tokens.get(i) != Some(&"by") {
                return None;
            }
            i += 1;
        }

        let next_num = match tokens.get(i) {
            Some(&num) => match num.parse::<i32>() {
                Ok(n) => n,
                Err(_) => return None,
            },
            None => return None,
        };
        i += 1;

        match operator {
            "plus" => result += next_num,
            "minus" => result -= next_num,
            "multiplied" => result *= next_num,
            "divided" => {
                if next_num == 0 {
                    return None; // 除零错误
                }
                result /= next_num;
            }
            _ => return None, // 不支持的操作符
        }
    }

    Some(result)
}
