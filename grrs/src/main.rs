fn main() {
    let pattern = std::env::args().nth(1).expect("No pattern given");
    let file_path = std::env::args().nth(2).expect("No path path given");

    println!("pattern: {:?}, path: {:?}", pattern, file_path);

    let contents: String = std::fs::read_to_string(file_path).expect("Could not read file");

    println!("Read {} chars from file", contents.len());
}
