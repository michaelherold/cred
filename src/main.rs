#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate clap;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;

pub mod models;
pub mod schema;

mod cli;
mod cmd;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use cmd::add::command as add_credential;
use cmd::list::command as list_credentials;

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    let conn = establish_connection();
    let matches = cli::build_cli().get_matches();

    match matches.subcommand() {
        ("add", Some(matches)) => add_credential(&conn, matches),
        ("list", Some(matches)) => list_credentials(&conn, matches),
        _ => unreachable!("The CLI parser should prevent this from happening."),
    }
}
