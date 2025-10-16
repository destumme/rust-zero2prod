use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

//#[tokio::main]
fn main() -> Result<(), std::io::Error> {
    let startup = async move {HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
    };

    //same as making main async w/ #[tokio::main]
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(startup)
}
