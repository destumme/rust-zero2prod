use sqlx::{Connection, PgPool};
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

//#[tokio::main]
fn main() -> Result<(), std::io::Error> {
    //same as making main async w/ #[tokio::main]
    let body = async move {
        let config = get_configuration().expect("failed to read configuration");
        let connection_pool = PgPool::connect(&config.db.connection_string())
            .await
            .expect("failed to connection Postgres");

        let address = format!("127.0.0.1:{}", config.app_port);
        let listener = TcpListener::bind(address)?;

        run(listener, connection_pool)?.await
    };
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(body)
}
