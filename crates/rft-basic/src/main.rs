use std::io;

mod control_flow;
mod data_type;
mod formatting;
mod variable;

fn main() {
    println!("Hi, I'm a Rust program! Please type your name:");

    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();

    println!("Hello, {}! How old are you?", name.trim());

    let mut age_input = String::new();

    loop {
        age_input.clear(); // clear previous input
        io::stdin().read_line(&mut age_input).unwrap();

        if age_input.trim().parse::<u32>().is_ok() {
            break;
        }

        println!("Please enter a valid age:");
    }

    let age: u32 = age_input.trim().parse().unwrap();

    println!("Hello, {}! You are {} years old.", name.trim(), age);
}
