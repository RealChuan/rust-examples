use actix_web::{web, App, HttpServer, Responder};

mod file_handler;

async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").to(|| async { "Hello, world!" }))
            .service(web::resource("/hello/{name}").route(web::get().to(greet)))
            .service(
                web::resource("/files/{path:.*}")
                    .route(web::get().to(file_handler::download_file))
                    .route(web::put().to(file_handler::upload_file_put))
                    .route(web::delete().to(file_handler::delete_file))
                    .route(web::post().to(file_handler::upload_file_post)),
            )
            .service(web::resource("/files").route(web::post().to(file_handler::upload_file_post)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
