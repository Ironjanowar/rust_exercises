use std::io::{BufRead, BufReader};

fn read_single_line_integer() -> (u64, u64) {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let number: u64 = split.next().unwrap().parse().unwrap();

    // Reset buffer
    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    let sum: u64 = line.split_whitespace()
        .map(|x| x.parse().unwrap())
        .fold(0, |a, b: u64| a + b);
    (number, sum)
}

fn main() {
    let (number, input_sum): (u64, u64) = read_single_line_integer();
    let full_sum: u64 = (1..(number+1)).fold(0, |a, b| a + b);
    println!("{}", full_sum - input_sum);
}
