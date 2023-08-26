use crate::http::{Request, RequestError, Response};
use std::{io::Read, net::TcpListener};

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_error_request(&mut self, e: &RequestError) -> Response;
}

#[derive(Debug)]
pub struct Server {
    address: String,
    port: u16,
}

impl Server {
    pub fn default() -> Self {
        Default::default()
    }

    pub fn new(address: String, port: u16) -> Self {
        Self { address, port }
    }

    pub fn new_full_address(full_address: &str) -> Result<Self, &str> {
        // If not contain ':'
        if !full_address.contains(':') {
            return Err("Invalid address");
        }

        // Split out with ':' for address
        let _v: Vec<&str> = full_address.trim().split(':').collect();

        if _v.is_empty() {
            return Err("Invalid address");
        }

        let first = _v.first().unwrap_or(&"127.0.0.1").to_string();
        let last: u16 = _v.get(1).unwrap_or(&"8080").parse().unwrap_or(8080);

        Ok(Self {
            address: first,
            port: last,
        })
    }

    pub fn full_address(&self) -> String {
        format!("{}:{}", self.address, self.port)
    }

    pub fn run(&self, mut handler: impl Handler) {
        let full_address = self.full_address();
        println!("Listening on {}", full_address);

        let listener = TcpListener::bind(&full_address).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    println!("Connected successfully!");

                    let mut buffer = [0; 1024];

                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(req) => handler.handle_request(&req),
                                Err(e) => handler.handle_error_request(&e),
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response : {:?}", e);
                            }
                        }
                        Err(e) => println!("Failed to read from connection: {:?}", e),
                    }
                }
                Err(e) => {
                    println!("Failed to establish a connection: {:?}", e);

                    continue;
                }
            }
        }
    }
}

impl Default for Server {
    fn default() -> Self {
        Self {
            address: String::from("127.0.0.1"),
            port: 8080,
        }
    }
}
