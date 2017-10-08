extern crate clap;

mod cli;

fn main() {
    let matches = cli::build_cli().get_matches();

    match matches.subcommand() {
        _ => unreachable!("The CLI parser should prevent this from happening.")
    }
}
