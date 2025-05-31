use std::fs::File;
use std::io::{BufReader, BufRead, Read};
use std::path::PathBuf;

pub fn open_file_reader(file_path: &PathBuf) -> BufReader<File> {
    let f = match File::open(file_path) {
        Ok(handle) => handle,
        Err(_) => panic!("Could not open file {:#?}", file_path)
    };
    
    BufReader::new(f)
}

pub fn search<R>(pat: &str, mut reader: BufReader<R>)
where
    R: Read
{
    // QUESTION: Is it more efficient to create a new String buffer on every loop?
    // By creating the unsized String outside the loop, it has to be reallocated on every loop anyway
    let mut line = String::new();

    loop {
        let res = reader.read_line(&mut line).expect("Unable to read line");

        if res == 0 {
            break;
        }

        if line.contains(&pat) {
            println!("{}", line.trim());
        }

        line.clear();
    }
}
