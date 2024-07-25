use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

fn main() {

    // Create a new listener.
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // Printing that we have a connection established.
    for stream in listener.incoming() {
        let stream = stream.unwrap(); // Get the TCP stream or panic if there is an error.

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024]; // Create a buffer of 1024 bytes to hold the data that is read.
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let contents = fs::read_to_string("index.html").unwrap(); // Read the contents of the file into a string.

    let (status_line, filename) = 
        if buffer.starts_with(get) {
            ("HTTP/1.1 200 OK", "index.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };

    let contents = fs::read_to_string(filename).unwrap(); // Read the contents of the 404 file into a string.
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    ); // Create the response with the contents of the 404 file.

    stream.write(response.as_bytes()).unwrap(); // Write the response to the stream.
    stream.flush().unwrap(); // Flush the stream to ensure that all data is written to the stream.

    
}
