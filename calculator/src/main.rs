use std::io::{self, Write};

fn main() {
    println!("Zaawansowany kalkulator Rust (obsługuje wyrażenia, np. 2 + 3 * (4 - 1) / 2)");
    println!("Wpisz wyrażenie lub 'exit' aby zakończyć.");
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Błąd odczytu");
        let input = input.trim();
        if input.eq_ignore_ascii_case("exit") {
            println!("Do widzenia!");
            break;
        }
        match eval_expr(input) {
            Ok(result) => println!("Wynik: {}", result),
            Err(e) => println!("Błąd: {}", e),
        }
    }
}

fn eval_expr(expr: &str) -> Result<f64, &'static str> {
    let tokens = tokenize(expr)?;
    let rpn = shunting_yard(&tokens)?;
    eval_rpn(&rpn)
}

fn tokenize(expr: &str) -> Result<Vec<String>, &'static str> {
    let mut tokens = Vec::new();
    let mut num = String::new();
    for c in expr.chars() {
        if c.is_ascii_digit() || c == '.' || c == ',' {
            if c == ',' { num.push('.'); } else { num.push(c); }
        } else if c == ' ' {
            if !num.is_empty() {
                tokens.push(num.clone());
                num.clear();
            }
        } else if "+-*/()".contains(c) {
            if !num.is_empty() {
                tokens.push(num.clone());
                num.clear();
            }
            tokens.push(c.to_string());
        } else {
            return Err("Nieprawidłowy znak w wyrażeniu");
        }
    }
    if !num.is_empty() {
        tokens.push(num);
    }
    Ok(tokens)
}

fn shunting_yard(tokens: &[String]) -> Result<Vec<String>, &'static str> {
    let mut output = Vec::new();
    let mut ops: Vec<String> = Vec::new();
    let prec = |op: &str| match op {
        "+" | "-" => 1,
        "*" | "/" => 2,
        _ => 0,
    };
    for token in tokens {
        if let Ok(_) = token.parse::<f64>() {
            output.push(token.clone());
        } else if ["+", "-", "*", "/"].contains(&token.as_str()) {
            while let Some(top) = ops.last() {
                if ["+", "-", "*", "/"].contains(&top.as_str()) && prec(top) >= prec(token) {
                    output.push(ops.pop().unwrap());
                } else {
                    break;
                }
            }
            ops.push(token.clone());
        } else if token == "(" {
            ops.push(token.clone());
        } else if token == ")" {
            while let Some(top) = ops.last() {
                if top != "(" {
                    output.push(ops.pop().unwrap());
                } else {
                    break;
                }
            }
            if ops.last() == Some(&"(".to_string()) {
                ops.pop();
            } else {
                return Err("Brakujący nawias otwierający");
            }
        } else {
            return Err("Nieznany token");
        }
    }
    while let Some(op) = ops.pop() {
        if op == "(" || op == ")" {
            return Err("Brakujący nawias zamykający");
        }
        output.push(op);
    }
    Ok(output)
}

fn eval_rpn(rpn: &[String]) -> Result<f64, &'static str> {
    let mut stack = Vec::new();
    for token in rpn {
        if let Ok(num) = token.parse::<f64>() {
            stack.push(num);
        } else {
            let b = stack.pop().ok_or("Błąd stosu (brak argumentu)")?;
            let a = stack.pop().ok_or("Błąd stosu (brak argumentu)")?;
            let res = match token.as_str() {
                "+" => a + b,
                "-" => a - b,
                "*" => a * b,
                "/" => {
                    if b == 0.0 {
                        return Err("Dzielenie przez zero");
                    }
                    a / b
                },
                _ => return Err("Nieznany operator w RPN"),
            };
            stack.push(res);
        }
    }
    if stack.len() == 1 {
        Ok(stack[0])
    } else {
        Err("Błąd wyrażenia (za dużo argumentów)")
    }
}
