use std::{
    fs,
    // We bring std::io::prelude and std::io::BufReader into 
    // scope to get access to traits and types that let us 
    // read from and write to the stream.
    io::{prelude::*, BufReader}, 
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use hello::ThreadPool;

fn main() {
    // The bind function in this scenario works like the new function in 
    // that it will return a new TcpListener instance. The function is 
    // called bind because, in networking, connecting to a port to listen 
    // to is known as “binding to a port.”

    // The bind function returns a Result<T, E>, which indicates 
    // that it’s possible for binding to fail. we use unwrap to stop the 
    // program if errors happen.
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    // The incoming method on TcpListener returns an iterator that gives us 
    // a sequence of streams (more specifically, streams of type TcpStream). 
    // A single stream represents an open connection between the client and 
    // the server. A connection is the name for the full request and response 
    // process in which a client connects to the server, the server generates 
    // a response, and the server closes the connection. As such, we will read 
    // from the TcpStream to see what the client sent and then write our 
    // response to the stream to send data back to the client. Overall, this 
    // for loop will process each connection in turn and produce a series 
    // of streams for us to handle
    for stream in listener.incoming() {
        //  The reason we might receive errors from the incoming method 
        // when a client connects to the server is that we’re not actually 
        // iterating over connections. Instead, we’re iterating over connection 
        // attempts. The connection might not be successful for a number of 
        // reasons, many of them operating system specific. For example, many 
        // operating systems have a limit to the number of simultaneous open 
        // connections they can support; new connection attempts beyond that 
        // number will produce an error until some of the open connections are closed.

        // ata. When stream goes out of scope and is dropped at the end of the loop, 
        // the connection is closed as part of the drop implementation. Browsers 
        // sometimes deal with closed connections by retrying, because the problem 
        // might be temporary. The important factor is that we’ve successfully 
        // gotten a handle to a TCP connection!
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}


fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();



    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    // Next, we use format! to add the file’s contents as the 
    // body of the success response. To ensure a valid HTTP response, 
    // we add the Content-Length header which is set to 
    // the size of our response body, in this case the size of hello.html.
    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");


    // The first new line defines the response variable that 
    // holds the success message’s data. Then we call as_bytes 
    // on our response to convert the string data to bytes. The 
    // write_all method on stream takes a &[u8] and sends those bytes 
    // directly down the connection. Because the write_all operation 
    // could fail, we use unwrap on any error result as before. 
    // Again, in a real application you would add error handling here.
    stream.write_all(response.as_bytes()).unwrap();
    
}