use clap::Parser;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    pattern: String,
    file_path: PathBuf
}

fn main() {
    let cli = Cli::parse();

    println!("pattern: {:?}, path: {:?}", cli.pattern, cli.file_path);

    let f = match File::open(&cli.file_path) {
        Ok(handle) => handle,
        Err(_) => panic!("Could not open file {:#?}", cli.file_path)
    };
    let mut reader = BufReader::new(f);

    let mut line = String::new();

    loop {
        let res = reader.read_line(&mut line).expect("Unable to read line");

        if res == 0 {
            break;
        }

        if line.contains(&cli.pattern) {
            println!("{}", line);
        }

        line.clear();
    }
}
