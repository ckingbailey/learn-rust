use clap::Parser;
use std::path::PathBuf;
use grrs::{ make_search_pattern, open_file_reader, make_searcher };

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    search_str: String,
    file_path: PathBuf
}

fn main() {
    let cli = Cli::parse();

    let reader = open_file_reader(&cli.file_path);
    let pat = make_search_pattern(&cli.search_str);
    let search = make_searcher(pat);

    loop {
        search(reader);
    }
}
