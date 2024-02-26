use cli::cli;

mod cli;
mod preprocessors;
mod utils;

fn main() {
    let matches = cli().get_matches();
}
