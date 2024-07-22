use clap::{crate_version, Command};

#[must_use]
pub fn cmd() -> Command {
    Command::new("cardinal")
        .about("manage user configuration files.")
        .version(crate_version!())
        .subcommand_required(true)
        .arg_required_else_help(true)
}
