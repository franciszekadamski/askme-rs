use color_eyre::Result;
use clap::Parser;

mod app;
mod crossterm;
mod deck;
use crate::deck::Deck;


#[derive(Debug, Parser)]
struct Cli {
    #[arg(default_value_t = String::from("test.csv"))]
    filename: String,
}

fn main() -> color_eyre::Result<()> {
    let cli = Cli::parse();
    crossterm::run(&cli.filename)
}

