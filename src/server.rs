use crate::http::{ParseError, Request, Response, StatusCode};
use std::convert::TryFrom;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Self { address }
    }

    /// Starts a server loop. We use our TcpListener to listen out on an address (Default is local
    /// on port 8080, but is dynamic (with env variables)
    pub fn run(self, mut handler: impl Handler) {
        println!("Server listening on: {}", self.address);

        // Create our listener, unwrap the requests that can be passed to it, and note it can fail.
        let listener = TcpListener::bind(self.address).unwrap();

        loop {
            // First we match on the listener's ability to establish a connection. If it fails
            // we'll return a Failed to Establish Connection error.
            match listener.accept() {
                Ok((mut stream, _)) => {
                    // Declare and initialize request buffer [0; n] n = bytes. Big enough
                    // for our small sever, for now.
                    let mut buffer = [0; 1024];

                    // To deal with bad requests, and the potential failure mentioned above,
                    // we'll match on the buffer. If the buffer is Ok, we'll match
                    // on the request and our handler will handle parsing our request.
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => handler.handle_bad_request(&e),
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send a response: {}", e);
                            }
                        }
                        Err(e) => println!("Failed to read connection: {}", e),
                    }
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}
