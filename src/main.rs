use std::{char, io};
fn main() {
    println!("Hello, world!");

    let mut state = true;
    let mut count = 0;

    while state {
        let response = get_char();
        match response {
            'y' => {
                println!("Welcome to the rust club.");
                state = false;
            }
            'n' => {
                println!("Take your time and get back to us when you are ready.");
                state = false;
            }
            _ => println!("Man... we don't understand your response."),
        }
        count += 1;

        if count == 3 {
            println!(
                "We cannot assist you at this time. Read the instructions and try again later."
            );
            state = false;
        }
    }
}

fn get_char() -> char {
    println!("Do you agree, y/n?");

    let mut string = String::new();

    io::stdin()
        .read_line(&mut string)
        .expect("Couldn't read line.");

    let string: char = string
        .trim()
        .parse()
        .expect("This must be a unicode character.");
    return string;
}
