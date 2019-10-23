use std::io::{BufRead, BufReader};

fn read_single_line_integer() -> u64 {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    split.next().unwrap().parse().unwrap()
}

fn calculate_next_number(number: u64) -> u64 {
    if number % 2 == 0 {
        number / 2
    } else {
        number * 3 + 1
    }
}

fn apply_algorithm(number: u64) -> String {
    let mut result: String = number.to_string();
    apply_algorithm_rec(number, &mut result)
}

fn apply_algorithm_rec(number: u64, result: &mut String) -> String {
    if number == 1 {
        result.to_string()
    } else {
        let next_number: u64 = calculate_next_number(number);
        result.push_str(&format!(" {}", next_number));
        apply_algorithm_rec(next_number, result)
    }
}

fn main() {
    let number: u64 = read_single_line_integer();
    let result = apply_algorithm(number);
    println!("{}", result);
}
