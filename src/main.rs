extern crate clap;

mod cli;
mod cmd;

use cmd::add::command as add_credential;

fn main() {
    let matches = cli::build_cli().get_matches();

    match matches.subcommand() {
        ("add", Some(matches)) => add_credential(matches),
        _ => unreachable!("The CLI parser should prevent this from happening.")
    }
}
