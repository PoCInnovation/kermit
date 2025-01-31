use clap::Parser;
use cli::Cli;

mod cli;
mod wallet;

fn main() {
    Cli::parse();
}
