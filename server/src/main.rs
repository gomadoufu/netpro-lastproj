use actix_files::{Files, NamedFile};
use actix_multipart::Multipart;
use actix_web::{post, web, App, HttpServer, Result};
use futures::stream::StreamExt;

#[post("form")]
async fn form(mut form: Multipart) -> String {
    let mut name_text_pairs: Vec<(String, String)> = Vec::new();
    while let Some(Ok(mut field)) = form.next().await {
        let field_name = field
            .content_disposition()
            .and_then(|cd| cd.get_name().map(ToString::to_string))
            .expect("Can't get field name!");

        let mut field_bytes: Vec<u8> = Vec::new();
        while let Some(Ok(bytes)) = field.next().await {
            for byte in bytes {
                field_bytes.push(byte);
            }
        }

        let field_text = String::from_utf8_lossy(&field_bytes).into_owned();
        name_text_pairs.push((field_name, field_text));
    }

    let output = String::new();
    for (name, text) in name_text_pairs {
        println!("{}: {}", name, text);
    }
    println!("___________________");
    output
}

#[allow(clippy::unused_async)]
async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open("./client/index.html")?)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/api/")
                    .service(form)
                    .default_service(web::route().to(web::HttpResponse::NotFound)),
            )
            .service(Files::new("/pkg", "./client/pkg"))
            .default_service(web::get().to(index))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
