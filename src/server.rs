use std::net::TcpListener;

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
            listener.accept();
        }
    }
}



