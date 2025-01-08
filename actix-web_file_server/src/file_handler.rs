use actix_multipart::Multipart;
use actix_web::{web, HttpResponse};
use futures_util::stream::StreamExt;
use std::io;
use std::path::{Path, PathBuf};
use tokio::fs::File;
use tokio::{fs, io::AsyncWriteExt};

const FILES_PATH: &str = "files";

pub async fn download_file(path: web::Path<String>) -> actix_web::Result<HttpResponse> {
    let path: PathBuf = format!("{}/{}", FILES_PATH, path).parse()?;
    println!("Downloading file: {}", path.display());

    if path.exists() {
        let file = File::open(&path).await?;
        let metadata = file.metadata().await?;
        let stream = tokio_util::io::ReaderStream::new(file);

        Ok(HttpResponse::Ok()
            .insert_header(("Content-Length", metadata.len().to_string()))
            .content_type("application/octet-stream")
            .insert_header((
                actix_web::http::header::CONTENT_DISPOSITION,
                format!(
                    "attachment; filename=\"{}\"",
                    path.file_name().unwrap().to_string_lossy()
                ),
            ))
            .streaming(stream))
    } else {
        Err(actix_web::error::ErrorNotFound("File not found"))
    }
}

pub async fn upload_file_put(
    path: web::Path<String>,
    mut payload: web::Payload,
) -> actix_web::Result<HttpResponse> {
    let filepath = format!("{}/{}", FILES_PATH, path);
    let filepath = Path::new(&filepath);

    if let Some(parent) = filepath.parent() {
        if let Err(e) = fs::create_dir_all(parent).await {
            return Err(actix_web::error::ErrorInternalServerError(e));
        }
    }

    println!("Uploading put file: {}", filepath.display());

    let mut file = fs::File::create(&filepath).await?;
    while let Some(item) = payload.next().await {
        let item = item.map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        file.write_all(&item).await?;
    }

    Ok(HttpResponse::Ok().body(format!(
        "File '{}' uploaded successfully",
        filepath.display()
    )))
}

pub async fn upload_file_post(
    mut payload: Multipart,
    path: Option<web::Path<String>>,
) -> actix_web::Result<HttpResponse> {
    while let Some(item) = payload.next().await {
        let mut field = item?;

        let content_disposition = field.content_disposition();
        let filename = match content_disposition.and_then(|cd| cd.get_filename()) {
            Some(name) => name,
            None => return Ok(HttpResponse::BadRequest().body("Missing content disposition")),
        };
        let path = path.as_ref().map(|p| p.as_str()).unwrap_or("");
        let filepath = if path.is_empty() {
            format!("files/{}", filename)
        } else {
            format!("files/{}/{}", path, filename)
        };
        let filepath = Path::new(&filepath);

        if let Some(parent) = filepath.parent() {
            if let Err(e) = fs::create_dir_all(parent).await {
                return Err(actix_web::error::ErrorInternalServerError(e));
            }
        }

        println!("Uploading post file: {}", filepath.display());

        let mut file = fs::File::create(&filepath).await?;
        while let Some(chunk) = field.next().await {
            let data = chunk?;
            file.write_all(&data).await?;
        }
    }

    Ok(HttpResponse::Ok().body("File uploaded successfully"))
}

pub async fn delete_file(path: web::Path<String>) -> actix_web::Result<HttpResponse> {
    let path: PathBuf = format!("{}/{}", FILES_PATH, path).parse()?;
    println!("Deleting file: {}", path.display());

    if path.exists() {
        if let Err(e) = fs::remove_file(path.clone()).await {
            return Err(actix_web::error::ErrorInternalServerError(e));
        }
        Ok(HttpResponse::Ok().body(format!("File '{}' deleted successfully", path.display())))
    } else {
        Err(actix_web::error::ErrorNotFound("File not found"))
    }
}
