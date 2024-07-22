use cardinal::cli;

fn main() {
    match cli::cmd().get_matches().subcommand() {
        _ => {
            cli::cmd().print_help().expect("can't print help.");
        }
    }
}
