use std::io;
use rand::Rng;

fn get_integer_input() -> u32 {
    println!("Input a number between 1-100:");
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    input.trim().parse().expect("Please try a number!")
}

fn generate_number() -> u32 {
    rand::thread_rng().gen_range(1, 100)
}

fn guess_game() {
    let number: u32 = get_integer_input();
    let generated_number: u32 = generate_number();

    if number == generated_number {
        println!("Congrats! You guessed right, the number is {}", number);
    } else {
        println!("Damn! You failed, the generated number is {}. Your input was {}", generated_number, number);
    }
}

fn find_your_number() {
    let number: u32 = get_integer_input();
    let mut generated_number: u32 = generate_number();
    let mut tries: u32 = 0;
    let max_tries: u32 = 1000;
    let mut number_found: bool = false;

    while generated_number != number {
        generated_number = generate_number();
        if tries >= max_tries {
            number_found = false;
            break;
        } else {
            number_found = true;
            tries = &tries + 1;
        }
    }

    if number_found {
        println!("Generated number {} after {} tries", number, &tries);
    } else {
        println!("Max tries reached... I'm sorry I couldn't find your number");
    }
}

fn main() {
    println!("Hi! What do you want to play?");
    println!("  1. Guess game");
    println!("  2. Find your number");

    let selected_option = get_integer_input();

    match selected_option {
        1 => guess_game(),
        2 => find_your_number(),
        _ => println!("Sorry, {} is not an option.", selected_option),
    }
}
