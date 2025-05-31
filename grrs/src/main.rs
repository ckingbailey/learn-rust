use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    pattern: String,
    file_path: std::path::PathBuf
}

fn main() {
    let cli = Cli::parse();

    println!("pattern: {:?}, path: {:?}", cli.pattern, cli.file_path);

    // let path_slc = &file_path[..];
    let contents = match std::fs::read_to_string(&cli.file_path) {
        Ok(contents) => contents,
        Err(_) => panic!("Could not read file {:#?}", cli.file_path)
    };

    println!("Read {} chars from file", contents.len());

    for content_line in contents.split('\n') {
        if content_line.contains(&cli.pattern) {
            println!("{}", content_line);
        }
    }
}
