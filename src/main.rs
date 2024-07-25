use cardinal::cli;

fn main() {
    tracing_subscriber::fmt::init();
    cli::handle_subcmd(cli::cmd().get_matches());
}
