extern crate diesel;

use std::io::{self, Write};

use clap::ArgMatches;
use diesel::prelude::*;

use super::super::models::NewCredential;

fn get_input(prompt: &str) -> String {
    let mut input = String::new();

    print!("  -> {}: ", prompt);
    io::stdout().flush().unwrap();

    match io::stdin().read_line(&mut input) {
        Ok(_) => { input.trim().to_string() }
        Err(_) => { "".to_string() }
    }
}

fn create_credential(conn: &SqliteConnection, new_credential: &NewCredential) -> usize {
    use super::super::schema::credentials;

    diesel::insert(new_credential)
        .into(credentials::table)
        .execute(conn)
        .expect("Error saving new credential")
}

fn value_or_input<'a>(value: Option<&'a str>, prompt: &'a str) -> String {
    match value {
        Some(v) => v.to_string(),
        None => get_input(prompt)
    }
}

pub fn command(conn: &SqliteConnection, args: &ArgMatches) {
    let url = args.value_of("URL").unwrap();
    let name = args.value_of("name").unwrap_or(url);
    let username = value_or_input(args.value_of("username"), "Username");
    let password = value_or_input(args.value_of("password"), "Password");

    let new_credential = NewCredential {
        url: url,
        name: name,
        username: &username,
        password: &password
    };

    create_credential(conn, &new_credential);

    println!("\nSaved credential for {}", name);
}
