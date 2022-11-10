use std::env::{args, Args};

fn main() {
    let mut args: Args = args();

    let first = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second = args.nth(0).unwrap();

    let first_number: f32 = first.parse().unwrap();
    // can use syntax above to type or turbofish below
    let second_number = second.parse::<f32>().unwrap();
    let result = operate(operator, first_number, second_number);

    println!("{:?}", output(first_number, operator, second_number, result));
}

// whats going on with the nth function
// nth calls next, which means we don't have to look for specific
// index. It's already going to the next one if that's what you need
// fn nth(&mut self, n:usize) -> Option<String> {
//     // assume n = 0;
//     // inner = ["1", "2"]
//     self.inner.next(); // "1"
//     // calling next again results in second element
//     self.inner.next(); // "2"
// }

fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
    match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '/' => first_number / second_number,
        '*' | 'x' | 'X' => first_number * second_number,
        _ => panic!("Invalid operator used.")
    }
}

fn output(first_number: f32, operator: char, second_number: f32, result: f32) -> String {
    format!("{} {} {} = {}", first_number, operator, second_number, result)
}