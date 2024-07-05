use std::io::{Read, Write};
use std::net::TcpListener;
use crate::request::Request;
use crate::response::{Response, StatusCode};


pub struct Server {
    addr: String,

}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr,
        }
    }

    pub fn run(&self) {
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, addr)) => {
                    let mut buffer = [0; 1024]; // there is a chance of breakage
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(req) => {
                                    Response::new(StatusCode::Ok, Some("<h1> Hello world </h1>".to_string()))
                                }
                                Err(e) => {
                                    println!("Failed to pass an request {}", e);
                                    Response::new(StatusCode::BadRequest, None)
                                }
                            };

                            if let Err(e) = response.send(&mut stream  ) {
                                println!("Failed to send response")
                            }
                        }
                        Err(e) => println!("Failed to read from connection {}", e)
                    }
                }
                Err(e) => {
                    println!("Error failed to establish connection {}", e);
                    continue;
                }
            }
        }
    }
}



