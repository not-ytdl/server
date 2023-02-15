use actix_web::{App, HttpServer, web, guard};
use server_lib::handler::Routes;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // we initialize the client
    HttpServer::new(|| 
        App::new().service(
            web::resource("/search")
                // we use the guard to make sure that the query is present and it's a get to that route
                .guard(guard::Get())
                .route(web::get().to(Routes::search))
        ))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
