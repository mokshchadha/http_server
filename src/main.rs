use http_server::method::Method;
use http_server::server::{Server};


fn main() {
    let get = Method::GET;
    let post = Method::POST;
    let delete = Method::DELETE;
    let put = Method::PUT;

    let server = Server::new("127.0.0.1:8000".to_string());
    server.run();
}