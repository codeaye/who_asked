use actix_files::NamedFile;
use actix_web::{get, App, HttpRequest, HttpResponse, HttpServer, Responder};
use rand::{thread_rng, Rng};
use std::{env, path::Path};

const CAP: u8 = 20;

#[get("/")]
async fn image(req: HttpRequest) -> impl Responder {
    let file = NamedFile::open_async(
        Path::new("images")
            .join(thread_rng().gen_range(0..=CAP).to_string())
            .with_extension("webp"),
    )
    .await
    .unwrap();

    file.into_response(&req)
}

#[get("/discord")]
async fn discord(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("discord.html"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT")
        .unwrap_or("8000".to_string())
        .parse::<u16>()
        .unwrap();
    println!("Listening on port {}", port);
    HttpServer::new(|| App::new().service(image).service(discord))
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
