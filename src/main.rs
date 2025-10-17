
use zero2prod::run;

//#[tokio::main]
fn main() -> Result<(), std::io::Error> {
    let body = async move {
        run()?.await
    };

    //same as making main async w/ #[tokio::main]
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(body)
}
