use std::io::{self, Write};
use clap::ArgMatches;

fn get_input(prompt: &str) -> String {
    let mut input = String::new();

    print!("  -> {}: ", prompt);
    io::stdout().flush().unwrap();

    match io::stdin().read_line(&mut input) {
        Ok(_) => { input.trim().to_string() }
        Err(_) => { "".to_string() }
    }
}

fn value_or_input<'a>(value: Option<&'a str>, prompt: &'a str) -> String {
    match value {
        Some(v) => v.to_string(),
        None => get_input(prompt)
    }
}

pub fn command(args: &ArgMatches) {
    let url = args.value_of("URL").unwrap();
    let name = args.value_of("name").unwrap_or(url);
    let username = value_or_input(args.value_of("username"), "Username");
    let password = value_or_input(args.value_of("password"), "Password");

    println!("Name: {}", name);
    println!("URL: {}", url);
    println!("Username: {}", username);
    println!("Password: {}", password);
}
