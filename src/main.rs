use args::Args;
use clap::Parser;
use commands::execute_command;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let cli = Args::parse();
    Ok(execute_command(cli.commands)?)
}
