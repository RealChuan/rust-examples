use actix_files::NamedFile;
use actix_multipart::Multipart;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use futures_util::stream::StreamExt;
use std::io;
use std::path::PathBuf;
use tokio::{fs::File, io::AsyncWriteExt};

async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}

async fn download_file(filename: web::Path<String>) -> actix_web::Result<NamedFile> {
    let path: PathBuf = format!("uploads/{}", filename).parse()?;
    Ok(NamedFile::open(path)?)
}

async fn upload_file_put(
    path: web::Path<String>,
    mut payload: web::Payload,
) -> actix_web::Result<HttpResponse> {
    let filename = path.into_inner();
    let filepath = format!("uploads/{}", filename);

    let mut file = File::create(&filepath).await?;
    while let Some(item) = payload.next().await {
        let item = item.map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        file.write_all(&item).await?;
    }

    Ok(HttpResponse::Ok().body(format!("File '{}' uploaded successfully", filename)))
}

async fn upload_file_post(mut payload: Multipart) -> actix_web::Result<HttpResponse> {
    while let Some(item) = payload.next().await {
        let mut field = item?;

        let content_disposition = field.content_disposition();
        let filename = match content_disposition.and_then(|cd| cd.get_filename()) {
            Some(name) => name,
            None => return Ok(HttpResponse::BadRequest().body("Missing content disposition")),
        };
        let filepath = format!("uploads/{}", filename);

        let mut file = File::create(&filepath).await?;
        while let Some(chunk) = field.next().await {
            let data = chunk?;
            file.write_all(&data).await?;
        }
    }

    Ok(HttpResponse::Ok().body("File uploaded successfully"))
}

async fn delete_file(filename: web::Path<String>) -> actix_web::Result<HttpResponse> {
    let path: PathBuf = format!("uploads/{}", filename).parse()?;
    tokio::fs::remove_file(path).await?;
    Ok(HttpResponse::Ok().body(format!("File '{}' deleted successfully", filename)))
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").to(|| async { "Hello, world!" }))
            .service(web::resource("/hello/{name}").route(web::get().to(greet)))
            .service(web::resource("/download/{filename}").route(web::get().to(download_file)))
            .service(web::resource("/upload/{filename}").route(web::put().to(upload_file_put)))
            .service(web::resource("/upload").route(web::post().to(upload_file_post)))
            .service(web::resource("/delete/{filename}").route(web::delete().to(delete_file)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
