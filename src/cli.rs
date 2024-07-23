use clap::{crate_version, ArgMatches, Command};

use crate::commands;

#[must_use]
pub fn cmd() -> Command {
    Command::new("cardinal")
        .about("Manage user home files.")
        .version(crate_version!())
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(Command::new("realise").about("Install files to user home"))
        .subcommand(Command::new("check").about("Validate the configuration file"))
}

pub fn handle_subcmd(matches: ArgMatches) {
    match matches.subcommand() {
        Some(("realise", _)) => commands::realise::exec(),
        Some(("check", _)) => commands::check::exec(),
        _ => {
            cmd().print_help().expect("can't print help.");
        }
    }
}
