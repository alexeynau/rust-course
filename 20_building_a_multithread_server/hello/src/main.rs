use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    // TCP socket server listening for connections bind to 127.0.0.1:7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // iterating over the connections being received on this listener
    for stream in listener.incoming() {
        // get a TCP stream between a local and a remote socket
        let stream = stream.unwrap();
        // handling a connection
        handle_connection(stream);
    }
}

/// Read data from the TCP stream and return html page according to request
fn handle_connection(mut stream: TcpStream) {
    // BufReader adds buffering by managing calls to the std::io::Read trait methods
    let buf_reader = BufReader::new(&mut stream);
    // getting a request as a String
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    // handling a request, choose status and page according to request
    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        // if ok
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        // if not ok
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    // read html file
    let contents = fs::read_to_string(filename).unwrap();
    // get content length
    let length = contents.len();
    // make a response from status, content length and content
    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    // send a response in stream as bytes
    stream.write_all(response.as_bytes()).unwrap();
}

