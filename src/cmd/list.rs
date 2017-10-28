use clap::ArgMatches;
use diesel::prelude::*;

use super::super::models::Credential;

fn list_all_credentials(conn: &SqliteConnection) -> Vec<Credential> {
    use super::super::schema::credentials::dsl::*;

    credentials
        .limit(5)
        .load::<Credential>(conn)
        .expect("Error loading credentials")
}

fn search_credentials(conn: &SqliteConnection, q: &str) -> Vec<Credential> {
    use super::super::schema::credentials::dsl::*;

    let query = format!("%{}%", q);

    credentials
        .filter(name.like(query.clone()).or(url.like(query)))
        .limit(5)
        .load::<Credential>(conn)
        .expect("Error loading credentials")
}

pub fn command(conn: &SqliteConnection, args: &ArgMatches) {
    let query = args.value_of("query");

    let credentials = match query {
        Some(q) => search_credentials(conn, q),
        None => list_all_credentials(conn),
    };

    println!("Listing {} credentials\n", credentials.len());

    for credential in &credentials {
        println!("{}", credential);
    }
}
