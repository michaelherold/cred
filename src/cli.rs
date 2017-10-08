use clap::{App, AppSettings};

pub fn build_cli() -> App<'static, 'static> {
    App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .after_help("You can also run `cred SUBCOMMAND -h` to get more information about that subcommand.")
}
