use std::path::PathBuf;
use clap::Parser;
use corosensei::CoroutineResult;
use grrs::{ open_file_reader, make_searcher };

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    search_str: String,
    file_path: PathBuf
}

fn main() {
    let cli = Cli::parse();

    let reader = open_file_reader(&cli.file_path);

    let search = make_searcher(reader);
    loop {
        match search.resume(&cli.search_str) {
            CoroutineResult::Yield(i) => println!("{}", i),
            CoroutineResult::Return(()) => break,
        }
    }
}
