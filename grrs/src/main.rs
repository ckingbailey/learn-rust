fn main() {
    let pattern = std::env::args().nth(1).expect("No pattern given");
    let file_path = std::env::args().nth(2).expect("No path path given");

    println!("pattern: {:?}, path: {:?}", pattern, file_path);

    let path_slc = &file_path[..];
    let contents = match std::fs::read_to_string(path_slc) {
        Ok(contents) => contents,
        Err(_) => panic!("Could not read file {}", path_slc)
    };

    println!("Read {} chars from file", contents.len());
}
