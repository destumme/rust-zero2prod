use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

//#[tokio::main]
fn main() -> Result<(), std::io::Error> {
    let config = get_configuration().expect("failed to read configuration");
    let address = format!("127.0.0.1:{}", config.app_port);
    let listener = TcpListener::bind(address)?;

    //same as making main async w/ #[tokio::main]
    let body = async move { run(listener)?.await };
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(body)
}
