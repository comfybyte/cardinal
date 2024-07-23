use cardinal::cli;

fn main() {
    cli::handle_subcmd(cli::cmd().get_matches());
}
