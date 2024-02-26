use cli::cli;

mod cli;
mod codecs;
mod preprocessors;
mod utils;

fn main() {
    let matches = cli().get_matches();
}
