use std::net::TcpListener;
use zero2prod::startup::run;

//#[tokio::main]
fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8000")
        .expect("failed to bind to port 8000");

    //same as making main async w/ #[tokio::main]
    let body = async move { run(listener)?.await };
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(body)
}
