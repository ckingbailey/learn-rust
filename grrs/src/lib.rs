use std::fs::File;
use std::io::{BufReader, BufRead, Read};
use std::path::PathBuf;
use corosensei::Coroutine;
use onig::Regex;

pub fn open_file_reader(file_path: &PathBuf) -> BufReader<File> {
    let f = File::open(file_path)
        .unwrap_or_else(
            |_| panic!("Could not open file {:#?}", file_path)
        );
    
    BufReader::new(f)
}

pub fn make_matcher(search_str: &str) -> Regex {
    Regex::new(search_str)
        .unwrap_or_else(
            |_| panic!("Could not parse string {} as regex", search_str)
        )
}

pub fn make_searcher<'a, R>(mut reader: BufReader<R>) -> corosensei::Coroutine<&'a Regex, String, ()>
where
    R: Read + 'a
{
    Coroutine::new(move |yielder, pat: &'a Regex| {
        loop {
            // QUESTION: Is it more efficient to create a new String buffer on every loop?
            // By creating the unsized String outside the loop, it has to be reallocated on every loop anyway
            let mut line = String::new();
            let len = reader.read_line(&mut line).expect("Unable to read line");
    
            if len == 0 {
                break;
            }
    
            if pat.find(&line).is_some() {
                // NOTE: This String will have terminal \n
                yielder.suspend(line);
            }
        }
    })    
}
