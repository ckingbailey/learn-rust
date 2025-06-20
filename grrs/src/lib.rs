use onig::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::PathBuf;

pub fn open_file_reader(file_path: &PathBuf) -> BufReader<File> {
    let f =
        File::open(file_path).unwrap_or_else(|_| panic!("Could not open file {:#?}", file_path));

    BufReader::new(f)
}

pub fn make_search_pattern(search_str: &str) -> Regex {
    Regex::new(search_str)
        .unwrap_or_else(|_| panic!("Cloud not parse string {} as regex", search_str))
}

pub enum SearchResult {
    Match(String),
    NoMatch,
    End,
}

pub fn read_next_line<R>(mut reader: &BufReader<R>) -> Option<String>
where
    R: Read,
{
    let mut line = String::new();

    let len = reader.read_line(&mut line).expect("Unable to read line");

    if len == 0 {
        return None;
    }

    Some(line)
}

pub fn search(line: &str, pattern: &Regex) -> Option<str> {
    search_pattern.find(&line)
}
