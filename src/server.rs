use std::io::Read;
use std::net::TcpListener;
use crate::request::Request;


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
        println!("inside run function my niga");
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, addr)) => {
                    let mut buffer= [0; 1024]; // there is a chance of breakage
                   match stream.read(&mut buffer) {
                       Ok(_) => {
                           println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                           match Request::try_from(&buffer[..]) {
                               Ok(req) => {},
                               Err(e) => println!("Failed to pass an request {}", e)
                           }

                       },
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



