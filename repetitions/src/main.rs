use std::io::{BufRead, BufReader};
use std::collections::HashMap;


fn read_single_line() -> String {
    let mut input = BufReader::new(std::io::stdin());
    let mut line = "".to_string();
    input.read_line(&mut line).unwrap();
    line.to_string()
}

fn main() {
    let dna_chain: String = read_single_line();

    let repetitions = dna_chain.chars()
        .fold(HashMap::new(), |mut m, c| {
            *m.entry(c.to_string()).or_insert(0) += 1;
            m
        });

    let (_, longest_repetition) = repetitions.iter().max_by_key(|(_, &v)| v).unwrap();
    println!("{}", longest_repetition);
}
