use clap::{Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    pattern: String,
    // #[command(subcommand)]
    // format: Format,
}

fn main() {
    println!("Hello, world!");
}
