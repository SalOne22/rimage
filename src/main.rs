use cli::cli;

mod cli;
mod utils;

fn main() {
    let matches = cli().get_matches();
}
