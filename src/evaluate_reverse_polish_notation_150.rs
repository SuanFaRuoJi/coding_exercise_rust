use std::convert::TryInto;

pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack: Vec<i32> = Vec::new();
    for token in &tokens {
        match token.clone().as_str() {
            "+" => {
                let b: i32 = stack.pop().unwrap();
                let a: i32 = stack.pop().unwrap();
                stack.push(a + b);
            },
            "-" => {
                let b: i32 = stack.pop().unwrap();
                let a: i32 = stack.pop().unwrap();
                stack.push(a - b);
            },
            "*" => {
                let b: i32 = stack.pop().unwrap();
                let a: i32 = stack.pop().unwrap();
                stack.push(a * b);
            },
            "/" => {
                let b: i32 = stack.pop().unwrap();
                let a: i32 = stack.pop().unwrap();
                stack.push(a / b);
            },
            _ => {
                stack.push(token.clone().parse().unwrap());
            },
        }
    }
    return stack[0];
}