use clap::Parser;
use std::path::PathBuf;
use grrs::{ open_file_reader, search };

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    search_str: String,
    file_path: PathBuf
}

fn main() {
    let cli = Cli::parse();

    let reader = open_file_reader(&cli.file_path);

    search(&cli.search_str, reader);
}
