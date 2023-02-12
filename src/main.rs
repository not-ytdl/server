use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use ytdlrs_lib::api::{client, downloader, fetch, search};

#[get("/")]
async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello_world)
            
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
