use clap::{arg, command, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "cli-app", version = "1.0", about = "PngMe command")]
pub struct Args {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}
// an enum in Rust can have variants that include data, and the data can be of any type, including structs, tuples, or even other enums. The () in the enum variant indicates that the variant holds data of a specific type. For example, below example shows that Shape enum will have Circle which is type f64

#[derive(Parser, Debug)]
pub struct EncodeArgs {
    #[arg(short, long)]
    pub in_file_path: PathBuf,
    #[arg(short, long)]
    pub chunk_type: String,
    #[arg(short, long)]
    pub message: String,
    #[arg(short, long)]
    pub out_file_path: PathBuf,
}

#[derive(Parser, Debug)]
pub struct DecodeArgs {
    #[arg(short, long)]
    pub in_file_path: PathBuf,
    #[arg(short, long)]
    pub chunk_type: String,
}

#[derive(Parser, Debug)]
pub struct RemoveArgs {
    #[arg(short, long)]
    pub in_file_path: PathBuf,
    #[arg(short, long)]
    pub chunk_type: String,
}

#[derive(Parser, Debug)]
pub struct PrintArgs {
    #[arg(short, long)]
    pub in_file_path: PathBuf,
}
