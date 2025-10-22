use std::net::TcpListener;
use zero2prod::run;

//#[tokio::main]
fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    println!("Server will start on port:{}", port);

    //same as making main async w/ #[tokio::main]
    let body = async move { run(listener)?.await };
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(body)
}
