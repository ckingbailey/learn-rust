use std::fs::File;
use std::io::{BufReader, BufRead, Read};
use std::path::PathBuf;
use onig::Regex;

pub fn open_file_reader(file_path: &PathBuf) -> BufReader<File> {
    let f = File::open(file_path)
        .unwrap_or_else(
            |_| panic!("Could not open file {:#?}", file_path)
        );
    
    BufReader::new(f)
}

pub fn search<R>(search_str: &str, mut reader: BufReader<R>)
where
    R: Read
{
    let pat = Regex::new(search_str)
        .unwrap_or_else(
            |_| panic!("Could not parse string {} as regex", search_str)
        );

    // QUESTION: Is it more efficient to create a new String buffer on every loop?
    // By creating the unsized String outside the loop, it has to be reallocated on every loop anyway
    let mut line = String::new();

    loop {
        let len = reader.read_line(&mut line).expect("Unable to read line");

        if len == 0 {
            break;
        }

        if pat.find(&line).is_some() {
            println!("{}", line.trim());
        }

        line.clear();
    }
}
