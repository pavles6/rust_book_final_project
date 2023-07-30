use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use rust_book_final_project::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    let thread_pool = ThreadPool::new(4);

    println!("Listening on port 8080");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread_pool.execute(|| {
            handle_conn(stream);
        });
    }
}

fn handle_conn(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let req_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &req_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}",);

    match stream.write_all(response.as_bytes()) {
        Ok(_) => {}
        Err(e) => println!("Error writing to stream: {}", e),
    }
}
