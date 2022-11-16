use actix_files::NamedFile;
use actix_web::{get, App, HttpRequest, HttpServer, Responder};
use rand::{thread_rng, Rng};
use std::path::Path;

const CAP: u8 = 20;

#[get("/")]
async fn index(req: HttpRequest) -> impl Responder {
    let file = NamedFile::open_async(
        Path::new("images")
            .join(thread_rng().gen_range(0..=CAP).to_string())
            .with_extension("webp"),
    )
    .await
    .unwrap();

    file.into_response(&req)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
