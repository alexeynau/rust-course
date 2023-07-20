use hello_multithread::ThreadPool;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    // create TCP Listener for connection
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // create thread pool with size 4
    let pool = ThreadPool::new(4);
    // listen request in stream
    for stream in listener.incoming() {
        // get a stream
        let stream = stream.unwrap();
        // give job to thread pool
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    // read a request from stream
    let buf_reader = BufReader::new(&mut stream);
    // get a request from buffer
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    // return page according to request
    let (status_line, filename) = match &request_line[..] {
        // successz request
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            // example of long request
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        // fail request
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };
    // read html page
    let contents = fs::read_to_string(filename).unwrap();
    // get content length
    let length = contents.len();
    // create a response
    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    // send response
    stream.write_all(response.as_bytes()).unwrap();
}
