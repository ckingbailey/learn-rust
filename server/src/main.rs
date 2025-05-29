use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpStream, TcpListener},
    thread,
    time::Duration,
};
use server::ThreadPool;

fn main() {
    let port = "7878";
    let listener = TcpListener::bind(format!("127.0.0.1:{}", 7878)).unwrap();
    let pool = ThreadPool::new(4);

    println!("Listending on {}", port);
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    println!("{request_line}");
    let (status_line, content_file) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "index.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let request_body = fs::read_to_string(content_file).unwrap();
    let content_length = request_body.len();

    let response =
        format!("{status_line}\r\nContent-Length: {content_length}\r\n\r\n{request_body}");

    stream.write_all(response.as_bytes()).unwrap();
}
