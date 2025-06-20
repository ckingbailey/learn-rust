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

pub fn make_search_pattern(search_str: &str) -> Regex {
    Regex::new(search_str)
        .unwrap_or_else(
            |_| panic!("Cloud not parse string {} as regex", search_str)
        )
}

pub enum SearchResult {
    Match(String),
    NoMatch,
    End,
}

pub fn make_searcher<R>(search_pattern: Regex) -> impl Fn(&BufReader<R>) -> SearchResult
where
    R: Read
{
    |mut reader: &BufReader<R>| -> SearchResult {
        // QUESTION: Is it more efficient to create a new String buffer on every loop?
        // By creating the unsized String outside the loop, it has to be reallocated on every loop anyway
        let mut line = String::new();
    
        let len = reader.read_line(&mut line).expect("Unable to read line");
    
        if len == 0 {
            return SearchResult::End
        }
    
        if search_pattern.find(&line).is_some() {
            SearchResult::Match(line);
        }
    
        SearchResult::NoMatch
    }
}
