use eframe::egui;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Kalkulator GUI",
        native_options,
        Box::new(|_cc| Box::new(CalculatorApp::default())),
    ).unwrap();
}

#[derive(Default)]
struct CalculatorApp {
    input: String,
    result: String,
}

impl eframe::App for CalculatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Zaawansowany kalkulator Rust");
            ui.label("Wpisz wyrażenie matematyczne:");
            ui.text_edit_singleline(&mut self.input);
            ui.add_space(8.0);
            egui::Grid::new("calc_grid").spacing([8.0, 8.0]).show(ui, |ui| {
                let buttons = [
                    ["7", "8", "9", "/", "("],
                    ["4", "5", "6", "*", ")"],
                    ["1", "2", "3", "-", "C"],
                    ["0", ".", "+", "=", "<-"],
                ];
                for row in buttons.iter() {
                    for &b in row.iter() {
                        if ui.button(b).clicked() {
                            match b {
                                "C" => { self.input.clear(); self.result.clear(); },
                                "<-" => { self.input.pop(); },
                                "=" => {
                                    match eval_expr(&self.input) {
                                        Ok(val) => self.result = format!("Wynik: {}", val),
                                        Err(e) => self.result = format!("Błąd: {}", e),
                                    }
                                },
                                _ => self.input.push_str(b),
                            }
                        }
                    }
                    ui.end_row();
                }
            });
            ui.add_space(8.0);
            if !self.result.is_empty() {
                ui.label(&self.result);
            }
        });
    }
}

// Funkcje eval_expr, tokenize, shunting_yard, eval_rpn:
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
