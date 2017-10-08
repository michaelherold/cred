use clap::{Arg, App, AppSettings, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
    let add_command = SubCommand::with_name("add")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .alias("a")
        .arg(
            Arg::with_name("URL")
                .help("The website the credential is associated with")
                .required(true)
        )
        .arg(
            Arg::with_name("name")
                .help("The name that you want to use to refer to the credential")
                .short("n")
                .long("name")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("username")
                .help("The username to use when logging in")
                .short("u")
                .long("username")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("password")
                .help("The password to use when logging in")
                .short("p")
                .long("password")
                .takes_value(true)
        );

    App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .after_help("You can also run `cred SUBCOMMAND -h` to get more information about that subcommand.")
        .subcommand(add_command)
}
